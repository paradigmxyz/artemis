// SPDX-License-Identifier: MIT
pragma solidity >=0.8.13;

import { ItemType } from "./ConsiderationEnums.sol";

// prettier-ignore
import {
    OfferItem,
    ConsiderationItem,
    SpentItem,
    ReceivedItem,
    OrderParameters,
    Order,
    AdvancedOrder,
    CriteriaResolver
} from "./ConsiderationStructs.sol";

import { BasicOrderFulfiller } from "./BasicOrderFulfiller.sol";

import { CriteriaResolution } from "./CriteriaResolution.sol";

import { AmountDeriver } from "./AmountDeriver.sol";

import "./ConsiderationConstants.sol";

/**
 * @title OrderFulfiller
 * @author 0age
 * @notice OrderFulfiller contains logic related to order fulfillment where a
 *         single order is being fulfilled and where basic order fulfillment is
 *         not available as an option.
 */
contract OrderFulfiller is
    BasicOrderFulfiller,
    CriteriaResolution,
    AmountDeriver
{
    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController)
        BasicOrderFulfiller(conduitController)
    {}

    /**
     * @dev Internal function to validate an order and update its status, adjust
     *      prices based on current time, apply criteria resolvers, determine
     *      what portion to fill, and transfer relevant tokens.
     *
     * @param advancedOrder       The order to fulfill as well as the fraction
     *                            to fill. Note that all offer and consideration
     *                            components must divide with no remainder for
     *                            the partial fill to be valid.
     * @param criteriaResolvers   An array where each element contains a
     *                            reference to a specific offer or
     *                            consideration, a token identifier, and a proof
     *                            that the supplied token identifier is
     *                            contained in the order's merkle root. Note
     *                            that a criteria of zero indicates that any
     *                            (transferable) token identifier is valid and
     *                            that no proof needs to be supplied.
     * @param fulfillerConduitKey A bytes32 value indicating what conduit, if
     *                            any, to source the fulfiller's token approvals
     *                            from. The zero hash signifies that no conduit
     *                            should be used, with direct approvals set on
     *                            Consideration.
     * @param recipient           The intended recipient for all received items.
     *
     * @return A boolean indicating whether the order has been fulfilled.
     */
    function _validateAndFulfillAdvancedOrder(
        AdvancedOrder memory advancedOrder,
        CriteriaResolver[] memory criteriaResolvers,
        bytes32 fulfillerConduitKey,
        address recipient
    ) internal returns (bool) {
        // Ensure this function cannot be triggered during a reentrant call.
        _setReentrancyGuard();

        // Declare empty bytes32 array (unused, will remain empty).
        bytes32[] memory priorOrderHashes;

        // Validate order, update status, and determine fraction to fill.
        (
            bytes32 orderHash,
            uint256 fillNumerator,
            uint256 fillDenominator
        ) = _validateOrderAndUpdateStatus(
                advancedOrder,
                criteriaResolvers,
                true,
                priorOrderHashes
            );

        // Create an array with length 1 containing the order.
        AdvancedOrder[] memory advancedOrders = new AdvancedOrder[](1);

        // Populate the order as the first and only element of the new array.
        advancedOrders[0] = advancedOrder;

        // Apply criteria resolvers using generated orders and details arrays.
        _applyCriteriaResolvers(advancedOrders, criteriaResolvers);

        // Retrieve the order parameters after applying criteria resolvers.
        OrderParameters memory orderParameters = advancedOrders[0].parameters;

        // Perform each item transfer with the appropriate fractional amount.
        _applyFractionsAndTransferEach(
            orderParameters,
            fillNumerator,
            fillDenominator,
            fulfillerConduitKey,
            recipient
        );

        // Emit an event signifying that the order has been fulfilled.
        _emitOrderFulfilledEvent(
            orderHash,
            orderParameters.offerer,
            orderParameters.zone,
            recipient,
            orderParameters.offer,
            orderParameters.consideration
        );

        // Clear the reentrancy guard.
        _clearReentrancyGuard();

        return true;
    }

    /**
     * @dev Internal function to transfer each item contained in a given single
     *      order fulfillment after applying a respective fraction to the amount
     *      being transferred.
     *
     * @param orderParameters     The parameters for the fulfilled order.
     * @param numerator           A value indicating the portion of the order
     *                            that should be filled.
     * @param denominator         A value indicating the total order size.
     * @param fulfillerConduitKey A bytes32 value indicating what conduit, if
     *                            any, to source the fulfiller's token approvals
     *                            from. The zero hash signifies that no conduit
     *                            should be used, with direct approvals set on
     *                            Consideration.
     * @param recipient           The intended recipient for all received items.
     */
    function _applyFractionsAndTransferEach(
        OrderParameters memory orderParameters,
        uint256 numerator,
        uint256 denominator,
        bytes32 fulfillerConduitKey,
        address recipient
    ) internal {
        // Read start time & end time from order parameters and place on stack.
        uint256 startTime = orderParameters.startTime;
        uint256 endTime = orderParameters.endTime;

        // Initialize an accumulator array. From this point forward, no new
        // memory regions can be safely allocated until the accumulator is no
        // longer being utilized, as the accumulator operates in an open-ended
        // fashion from this memory pointer; existing memory may still be
        // accessed and modified, however.
        bytes memory accumulator = new bytes(AccumulatorDisarmed);

        // As of solidity 0.6.0, inline assembly cannot directly access function
        // definitions, but can still access locally scoped function variables.
        // This means that in order to recast the type of a function, we need to
        // create a local variable to reference the internal function definition
        // (using the same type) and a local variable with the desired type,
        // and then cast the original function pointer to the desired type.

        /**
         * Repurpose existing OfferItem memory regions on the offer array for
         * the order by overriding the _transfer function pointer to accept a
         * modified OfferItem argument in place of the usual ReceivedItem:
         *
         *   ========= OfferItem ==========   ====== ReceivedItem ======
         *   ItemType itemType; ------------> ItemType itemType;
         *   address token; ----------------> address token;
         *   uint256 identifierOrCriteria; -> uint256 identifier;
         *   uint256 startAmount; ----------> uint256 amount;
         *   uint256 endAmount; ------------> address recipient;
         */

        // Declare a nested scope to minimize stack depth.
        unchecked {
            // Declare a virtual function pointer taking an OfferItem argument.
            function(OfferItem memory, address, bytes32, bytes memory)
                internal _transferOfferItem;

            {
                // Assign _transfer function to a new function pointer (it takes
                // a ReceivedItem as its initial argument)
                function(ReceivedItem memory, address, bytes32, bytes memory)
                    internal _transferReceivedItem = _transfer;

                // Utilize assembly to override the virtual function pointer.
                assembly {
                    // Cast initial ReceivedItem type to an OfferItem type.
                    _transferOfferItem := _transferReceivedItem
                }
            }

            // Read offer array length from memory and place on stack.
            uint256 totalOfferItems = orderParameters.offer.length;

            // Iterate over each offer on the order.
            // Skip overflow check as for loop is indexed starting at zero.
            for (uint256 i = 0; i < totalOfferItems; ++i) {
                // Retrieve the offer item.
                OfferItem memory offerItem = orderParameters.offer[i];

                // Offer items for the native token can not be received
                // outside of a match order function.
                if (offerItem.itemType == ItemType.NATIVE) {
                    revert InvalidNativeOfferItem();
                }

                // Declare an additional nested scope to minimize stack depth.
                {
                    // Apply fill fraction to get offer item amount to transfer.
                    uint256 amount = _applyFraction(
                        offerItem.startAmount,
                        offerItem.endAmount,
                        numerator,
                        denominator,
                        startTime,
                        endTime,
                        false
                    );

                    // Utilize assembly to set overloaded offerItem arguments.
                    assembly {
                        // Write new fractional amount to startAmount as amount.
                        mstore(
                            add(offerItem, ReceivedItem_amount_offset),
                            amount
                        )

                        // Write recipient to endAmount.
                        mstore(
                            add(offerItem, ReceivedItem_recipient_offset),
                            recipient
                        )
                    }
                }

                // Transfer the item from the offerer to the recipient.
                _transferOfferItem(
                    offerItem,
                    orderParameters.offerer,
                    orderParameters.conduitKey,
                    accumulator
                );
            }
        }

        // Put ether value supplied by the caller on the stack.
        uint256 etherRemaining = msg.value;

        /**
         * Repurpose existing ConsiderationItem memory regions on the
         * consideration array for the order by overriding the _transfer
         * function pointer to accept a modified ConsiderationItem argument in
         * place of the usual ReceivedItem:
         *
         *   ====== ConsiderationItem =====   ====== ReceivedItem ======
         *   ItemType itemType; ------------> ItemType itemType;
         *   address token; ----------------> address token;
         *   uint256 identifierOrCriteria;--> uint256 identifier;
         *   uint256 startAmount; ----------> uint256 amount;
         *   uint256 endAmount;        /----> address recipient;
         *   address recipient; ------/
         */

        // Declare a nested scope to minimize stack depth.
        unchecked {
            // Declare virtual function pointer with ConsiderationItem argument.
            function(ConsiderationItem memory, address, bytes32, bytes memory)
                internal _transferConsiderationItem;
            {
                // Reassign _transfer function to a new function pointer (it
                // takes a ReceivedItem as its initial argument).
                function(ReceivedItem memory, address, bytes32, bytes memory)
                    internal _transferReceivedItem = _transfer;

                // Utilize assembly to override the virtual function pointer.
                assembly {
                    // Cast ReceivedItem type to ConsiderationItem type.
                    _transferConsiderationItem := _transferReceivedItem
                }
            }

            // Read consideration array length from memory and place on stack.
            uint256 totalConsiderationItems = orderParameters
                .consideration
                .length;

            // Iterate over each consideration item on the order.
            // Skip overflow check as for loop is indexed starting at zero.
            for (uint256 i = 0; i < totalConsiderationItems; ++i) {
                // Retrieve the consideration item.
                ConsiderationItem memory considerationItem = (
                    orderParameters.consideration[i]
                );

                // Apply fraction & derive considerationItem amount to transfer.
                uint256 amount = _applyFraction(
                    considerationItem.startAmount,
                    considerationItem.endAmount,
                    numerator,
                    denominator,
                    startTime,
                    endTime,
                    true
                );

                // Use assembly to set overloaded considerationItem arguments.
                assembly {
                    // Write derived fractional amount to startAmount as amount.
                    mstore(
                        add(considerationItem, ReceivedItem_amount_offset),
                        amount
                    )

                    // Write original recipient to endAmount as recipient.
                    mstore(
                        add(considerationItem, ReceivedItem_recipient_offset),
                        mload(
                            add(
                                considerationItem,
                                ConsiderationItem_recipient_offset
                            )
                        )
                    )
                }

                // Reduce available value if offer spent ETH or a native token.
                if (considerationItem.itemType == ItemType.NATIVE) {
                    // Ensure that sufficient native tokens are still available.
                    if (amount > etherRemaining) {
                        revert InsufficientEtherSupplied();
                    }

                    // Skip underflow check as a comparison has just been made.
                    etherRemaining -= amount;
                }

                // Transfer item from caller to recipient specified by the item.
                _transferConsiderationItem(
                    considerationItem,
                    msg.sender,
                    fulfillerConduitKey,
                    accumulator
                );
            }
        }

        // Trigger any remaining accumulated transfers via call to the conduit.
        _triggerIfArmed(accumulator);

        // If any ether remains after fulfillments...
        if (etherRemaining != 0) {
            // return it to the caller.
            _transferEth(payable(msg.sender), etherRemaining);
        }
    }

    /**
     * @dev Internal function to emit an OrderFulfilled event. OfferItems are
     *      translated into SpentItems and ConsiderationItems are translated
     *      into ReceivedItems.
     *
     * @param orderHash     The order hash.
     * @param offerer       The offerer for the order.
     * @param zone          The zone for the order.
     * @param fulfiller     The fulfiller of the order, or the null address if
     *                      the order was fulfilled via order matching.
     * @param offer         The offer items for the order.
     * @param consideration The consideration items for the order.
     */
    function _emitOrderFulfilledEvent(
        bytes32 orderHash,
        address offerer,
        address zone,
        address fulfiller,
        OfferItem[] memory offer,
        ConsiderationItem[] memory consideration
    ) internal {
        // Cast already-modified offer memory region as spent items.
        SpentItem[] memory spentItems;
        assembly {
            spentItems := offer
        }

        // Cast already-modified consideration memory region as received items.
        ReceivedItem[] memory receivedItems;
        assembly {
            receivedItems := consideration
        }

        // Emit an event signifying that the order has been fulfilled.
        emit OrderFulfilled(
            orderHash,
            offerer,
            zone,
            fulfiller,
            spentItems,
            receivedItems
        );
    }

    /**
     * @dev Internal pure function to convert an order to an advanced order with
     *      numerator and denominator of 1 and empty extraData.
     *
     * @param order The order to convert.
     *
     * @return advancedOrder The new advanced order.
     */
    function _convertOrderToAdvanced(Order calldata order)
        internal
        pure
        returns (AdvancedOrder memory advancedOrder)
    {
        // Convert to partial order (1/1 or full fill) and return new value.
        advancedOrder = AdvancedOrder(
            order.parameters,
            1,
            1,
            order.signature,
            ""
        );
    }

    /**
     * @dev Internal pure function to convert an array of orders to an array of
     *      advanced orders with numerator and denominator of 1.
     *
     * @param orders The orders to convert.
     *
     * @return advancedOrders The new array of partial orders.
     */
    function _convertOrdersToAdvanced(Order[] calldata orders)
        internal
        pure
        returns (AdvancedOrder[] memory advancedOrders)
    {
        // Read the number of orders from calldata and place on the stack.
        uint256 totalOrders = orders.length;

        // Allocate new empty array for each partial order in memory.
        advancedOrders = new AdvancedOrder[](totalOrders);

        // Skip overflow check as the index for the loop starts at zero.
        unchecked {
            // Iterate over the given orders.
            for (uint256 i = 0; i < totalOrders; ++i) {
                // Convert to partial order (1/1 or full fill) and update array.
                advancedOrders[i] = _convertOrderToAdvanced(orders[i]);
            }
        }

        // Return the array of advanced orders.
        return advancedOrders;
    }
}
