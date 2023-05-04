// SPDX-License-Identifier: MIT
pragma solidity >=0.8.13;

import { Side, ItemType } from "./ConsiderationEnums.sol";

// prettier-ignore
import {
    OfferItem,
    ConsiderationItem,
    ReceivedItem,
    OrderParameters,
    Fulfillment,
    FulfillmentComponent,
    Execution,
    Order,
    AdvancedOrder,
    CriteriaResolver
} from "./ConsiderationStructs.sol";

import { OrderFulfiller } from "./OrderFulfiller.sol";

import { FulfillmentApplier } from "./FulfillmentApplier.sol";

import "./ConsiderationConstants.sol";

/**
 * @title OrderCombiner
 * @author 0age
 * @notice OrderCombiner contains logic for fulfilling combinations of orders,
 *         either by matching offer items to consideration items or by
 *         fulfilling orders where available.
 */
contract OrderCombiner is OrderFulfiller, FulfillmentApplier {
    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) OrderFulfiller(conduitController) {}

    /**
     * @notice Internal function to attempt to fill a group of orders, fully or
     *         partially, with an arbitrary number of items for offer and
     *         consideration per order alongside criteria resolvers containing
     *         specific token identifiers and associated proofs. Any order that
     *         is not currently active, has already been fully filled, or has
     *         been cancelled will be omitted. Remaining offer and consideration
     *         items will then be aggregated where possible as indicated by the
     *         supplied offer and consideration component arrays and aggregated
     *         items will be transferred to the fulfiller or to each intended
     *         recipient, respectively. Note that a failing item transfer or an
     *         issue with order formatting will cause the entire batch to fail.
     *
     * @param advancedOrders            The orders to fulfill along with the
     *                                  fraction of those orders to attempt to
     *                                  fill. Note that both the offerer and the
     *                                  fulfiller must first approve this
     *                                  contract (or a conduit if indicated by
     *                                  the order) to transfer any relevant
     *                                  tokens on their behalf and that
     *                                  contracts must implement
     *                                  `onERC1155Received` in order to receive
     *                                  ERC1155 tokens as consideration. Also
     *                                  note that all offer and consideration
     *                                  components must have no remainder after
     *                                  multiplication of the respective amount
     *                                  with the supplied fraction for an
     *                                  order's partial fill amount to be
     *                                  considered valid.
     * @param criteriaResolvers         An array where each element contains a
     *                                  reference to a specific offer or
     *                                  consideration, a token identifier, and a
     *                                  proof that the supplied token identifier
     *                                  is contained in the merkle root held by
     *                                  the item in question's criteria element.
     *                                  Note that an empty criteria indicates
     *                                  that any (transferable) token
     *                                  identifier on the token in question is
     *                                  valid and that no associated proof needs
     *                                  to be supplied.
     * @param offerFulfillments         An array of FulfillmentComponent arrays
     *                                  indicating which offer items to attempt
     *                                  to aggregate when preparing executions.
     * @param considerationFulfillments An array of FulfillmentComponent arrays
     *                                  indicating which consideration items to
     *                                  attempt to aggregate when preparing
     *                                  executions.
     * @param fulfillerConduitKey       A bytes32 value indicating what conduit,
     *                                  if any, to source the fulfiller's token
     *                                  approvals from. The zero hash signifies
     *                                  that no conduit should be used (and
     *                                  direct approvals set on Consideration).
     * @param recipient                 The intended recipient for all received
     *                                  items.
     * @param maximumFulfilled          The maximum number of orders to fulfill.
     *
     * @return availableOrders An array of booleans indicating if each order
     *                         with an index corresponding to the index of the
     *                         returned boolean was fulfillable or not.
     * @return executions      An array of elements indicating the sequence of
     *                         transfers performed as part of matching the given
     *                         orders.
     */
    function _fulfillAvailableAdvancedOrders(
        AdvancedOrder[] memory advancedOrders,
        CriteriaResolver[] memory criteriaResolvers,
        FulfillmentComponent[][] calldata offerFulfillments,
        FulfillmentComponent[][] calldata considerationFulfillments,
        bytes32 fulfillerConduitKey,
        address recipient,
        uint256 maximumFulfilled
    )
        internal
        returns (bool[] memory availableOrders, Execution[] memory executions)
    {
        // Validate orders, apply amounts, & determine if they utilize conduits.
        _validateOrdersAndPrepareToFulfill(
            advancedOrders,
            criteriaResolvers,
            false, // Signifies that invalid orders should NOT revert.
            maximumFulfilled,
            recipient
        );

        // Aggregate used offer and consideration items and execute transfers.
        (availableOrders, executions) = _executeAvailableFulfillments(
            advancedOrders,
            offerFulfillments,
            considerationFulfillments,
            fulfillerConduitKey,
            recipient
        );

        // Return order fulfillment details and executions.
        return (availableOrders, executions);
    }

    /**
     * @dev Internal function to validate a group of orders, update their
     *      statuses, reduce amounts by their previously filled fractions, apply
     *      criteria resolvers, and emit OrderFulfilled events.
     *
     * @param advancedOrders    The advanced orders to validate and reduce by
     *                          their previously filled amounts.
     * @param criteriaResolvers An array where each element contains a reference
     *                          to a specific order as well as that order's
     *                          offer or consideration, a token identifier, and
     *                          a proof that the supplied token identifier is
     *                          contained in the order's merkle root. Note that
     *                          a root of zero indicates that any transferable
     *                          token identifier is valid and that no proof
     *                          needs to be supplied.
     * @param revertOnInvalid   A boolean indicating whether to revert on any
     *                          order being invalid; setting this to false will
     *                          instead cause the invalid order to be skipped.
     * @param maximumFulfilled  The maximum number of orders to fulfill.
     * @param recipient         The intended recipient for all received items.
     */
    function _validateOrdersAndPrepareToFulfill(
        AdvancedOrder[] memory advancedOrders,
        CriteriaResolver[] memory criteriaResolvers,
        bool revertOnInvalid,
        uint256 maximumFulfilled,
        address recipient
    ) internal {
        // Ensure this function cannot be triggered during a reentrant call.
        _setReentrancyGuard();

        // Read length of orders array and place on the stack.
        uint256 totalOrders = advancedOrders.length;

        // Track the order hash for each order being fulfilled.
        bytes32[] memory orderHashes = new bytes32[](totalOrders);

        // Override orderHashes length to zero after memory has been allocated.
        assembly {
            mstore(orderHashes, 0)
        }

        // Declare an error buffer indicating status of any native offer items.
        // {00} == 0 => In a match function, no native offer items: allow.
        // {01} == 1 => In a match function, some native offer items: allow.
        // {10} == 2 => Not in a match function, no native offer items: allow.
        // {11} == 3 => Not in a match function, some native offer items: THROW.
        uint256 invalidNativeOfferItemErrorBuffer;

        // Use assembly to set the value for the second bit of the error buffer.
        assembly {
            // Use the second bit of the error buffer to indicate whether the
            // current function is not matchAdvancedOrders or matchOrders.
            invalidNativeOfferItemErrorBuffer := shl(
                1,
                gt(
                    // Take the remainder of the selector modulo a magic value.
                    mod(
                        shr(NumBitsAfterSelector, calldataload(0)),
                        NonMatchSelector_MagicModulus
                    ),
                    // Check if remainder is higher than the greatest remainder
                    // of the two match selectors modulo the magic value.
                    NonMatchSelector_MagicRemainder
                )
            )
        }

        // Skip overflow checks as all for loops are indexed starting at zero.
        unchecked {
            // Iterate over each order.
            for (uint256 i = 0; i < totalOrders; ++i) {
                // Retrieve the current order.
                AdvancedOrder memory advancedOrder = advancedOrders[i];

                // Determine if max number orders have already been fulfilled.
                if (maximumFulfilled == 0) {
                    // Mark fill fraction as zero as the order will not be used.
                    advancedOrder.numerator = 0;

                    // Update the length of the orderHashes array.
                    assembly {
                        mstore(orderHashes, add(i, 1))
                    }

                    // Continue iterating through the remaining orders.
                    continue;
                }

                // Validate it, update status, and determine fraction to fill.
                (
                    bytes32 orderHash,
                    uint256 numerator,
                    uint256 denominator
                ) = _validateOrderAndUpdateStatus(
                        advancedOrder,
                        criteriaResolvers,
                        revertOnInvalid,
                        orderHashes
                    );

                // Update the length of the orderHashes array.
                assembly {
                    mstore(orderHashes, add(i, 1))
                }

                // Do not track hash or adjust prices if order is not fulfilled.
                if (numerator == 0) {
                    // Mark fill fraction as zero if the order is not fulfilled.
                    advancedOrder.numerator = 0;

                    // Continue iterating through the remaining orders.
                    continue;
                }

                // Otherwise, track the order hash in question.
                orderHashes[i] = orderHash;

                // Decrement the number of fulfilled orders.
                // Skip underflow check as the condition before
                // implies that maximumFulfilled > 0.
                maximumFulfilled--;

                // Place the start time for the order on the stack.
                uint256 startTime = advancedOrder.parameters.startTime;

                // Place the end time for the order on the stack.
                uint256 endTime = advancedOrder.parameters.endTime;

                // Retrieve array of offer items for the order in question.
                OfferItem[] memory offer = advancedOrder.parameters.offer;

                // Read length of offer array and place on the stack.
                uint256 totalOfferItems = offer.length;

                // Iterate over each offer item on the order.
                for (uint256 j = 0; j < totalOfferItems; ++j) {
                    // Retrieve the offer item.
                    OfferItem memory offerItem = offer[j];

                    assembly {
                        // If the offer item is for the native token, set the
                        // first bit of the error buffer to true.
                        invalidNativeOfferItemErrorBuffer := or(
                            invalidNativeOfferItemErrorBuffer,
                            iszero(mload(offerItem))
                        )
                    }

                    // Apply order fill fraction to offer item end amount.
                    uint256 endAmount = _getFraction(
                        numerator,
                        denominator,
                        offerItem.endAmount
                    );

                    // Reuse same fraction if start and end amounts are equal.
                    if (offerItem.startAmount == offerItem.endAmount) {
                        // Apply derived amount to both start and end amount.
                        offerItem.startAmount = endAmount;
                    } else {
                        // Apply order fill fraction to offer item start amount.
                        offerItem.startAmount = _getFraction(
                            numerator,
                            denominator,
                            offerItem.startAmount
                        );
                    }

                    // Update end amount in memory to match the derived amount.
                    offerItem.endAmount = endAmount;

                    // Adjust offer amount using current time; round down.
                    offerItem.startAmount = _locateCurrentAmount(
                        offerItem.startAmount,
                        offerItem.endAmount,
                        startTime,
                        endTime,
                        false // round down
                    );
                }

                // Retrieve array of consideration items for order in question.
                ConsiderationItem[] memory consideration = (
                    advancedOrder.parameters.consideration
                );

                // Read length of consideration array and place on the stack.
                uint256 totalConsiderationItems = consideration.length;

                // Iterate over each consideration item on the order.
                for (uint256 j = 0; j < totalConsiderationItems; ++j) {
                    // Retrieve the consideration item.
                    ConsiderationItem memory considerationItem = (
                        consideration[j]
                    );

                    // Apply fraction to consideration item end amount.
                    uint256 endAmount = _getFraction(
                        numerator,
                        denominator,
                        considerationItem.endAmount
                    );

                    // Reuse same fraction if start and end amounts are equal.
                    if (
                        considerationItem.startAmount ==
                        considerationItem.endAmount
                    ) {
                        // Apply derived amount to both start and end amount.
                        considerationItem.startAmount = endAmount;
                    } else {
                        // Apply fraction to consideration item start amount.
                        considerationItem.startAmount = _getFraction(
                            numerator,
                            denominator,
                            considerationItem.startAmount
                        );
                    }

                    // Update end amount in memory to match the derived amount.
                    considerationItem.endAmount = endAmount;

                    // Adjust consideration amount using current time; round up.
                    considerationItem.startAmount = (
                        _locateCurrentAmount(
                            considerationItem.startAmount,
                            considerationItem.endAmount,
                            startTime,
                            endTime,
                            true // round up
                        )
                    );

                    // Utilize assembly to manually "shift" the recipient value.
                    assembly {
                        // Write recipient to endAmount, as endAmount is not
                        // used from this point on and can be repurposed to fit
                        // the layout of a ReceivedItem.
                        mstore(
                            add(
                                considerationItem,
                                ReceivedItem_recipient_offset // old endAmount
                            ),
                            mload(
                                add(
                                    considerationItem,
                                    ConsiderationItem_recipient_offset
                                )
                            )
                        )
                    }
                }
            }
        }

        // If the first bit is set, a native offer item was encountered. If the
        // second bit is set in the error buffer, the current function is not
        // matchOrders or matchAdvancedOrders. If the value is three, both the
        // first and second bits were set; in that case, revert with an error.
        if (invalidNativeOfferItemErrorBuffer == 3) {
            revert InvalidNativeOfferItem();
        }

        // Apply criteria resolvers to each order as applicable.
        _applyCriteriaResolvers(advancedOrders, criteriaResolvers);

        // Emit an event for each order signifying that it has been fulfilled.
        // Skip overflow checks as all for loops are indexed starting at zero.
        unchecked {
            // Iterate over each order.
            for (uint256 i = 0; i < totalOrders; ++i) {
                // Do not emit an event if no order hash is present.
                if (orderHashes[i] == bytes32(0)) {
                    continue;
                }

                // Retrieve parameters for the order in question.
                OrderParameters memory orderParameters = (
                    advancedOrders[i].parameters
                );

                // Emit an OrderFulfilled event.
                _emitOrderFulfilledEvent(
                    orderHashes[i],
                    orderParameters.offerer,
                    orderParameters.zone,
                    recipient,
                    orderParameters.offer,
                    orderParameters.consideration
                );
            }
        }
    }

    /**
     * @dev Internal function to fulfill a group of validated orders, fully or
     *      partially, with an arbitrary number of items for offer and
     *      consideration per order and to execute transfers. Any order that is
     *      not currently active, has already been fully filled, or has been
     *      cancelled will be omitted. Remaining offer and consideration items
     *      will then be aggregated where possible as indicated by the supplied
     *      offer and consideration component arrays and aggregated items will
     *      be transferred to the fulfiller or to each intended recipient,
     *      respectively. Note that a failing item transfer or an issue with
     *      order formatting will cause the entire batch to fail.
     *
     * @param advancedOrders            The orders to fulfill along with the
     *                                  fraction of those orders to attempt to
     *                                  fill. Note that both the offerer and the
     *                                  fulfiller must first approve this
     *                                  contract (or the conduit if indicated by
     *                                  the order) to transfer any relevant
     *                                  tokens on their behalf and that
     *                                  contracts must implement
     *                                  `onERC1155Received` in order to receive
     *                                  ERC1155 tokens as consideration. Also
     *                                  note that all offer and consideration
     *                                  components must have no remainder after
     *                                  multiplication of the respective amount
     *                                  with the supplied fraction for an
     *                                  order's partial fill amount to be
     *                                  considered valid.
     * @param offerFulfillments         An array of FulfillmentComponent arrays
     *                                  indicating which offer items to attempt
     *                                  to aggregate when preparing executions.
     * @param considerationFulfillments An array of FulfillmentComponent arrays
     *                                  indicating which consideration items to
     *                                  attempt to aggregate when preparing
     *                                  executions.
     * @param fulfillerConduitKey       A bytes32 value indicating what conduit,
     *                                  if any, to source the fulfiller's token
     *                                  approvals from. The zero hash signifies
     *                                  that no conduit should be used, with
     *                                  direct approvals set on Consideration.
     * @param recipient                 The intended recipient for all received
     *                                  items.
     *
     * @return availableOrders An array of booleans indicating if each order
     *                         with an index corresponding to the index of the
     *                         returned boolean was fulfillable or not.
     * @return executions      An array of elements indicating the sequence of
     *                         transfers performed as part of matching the given
     *                         orders.
     */
    function _executeAvailableFulfillments(
        AdvancedOrder[] memory advancedOrders,
        FulfillmentComponent[][] memory offerFulfillments,
        FulfillmentComponent[][] memory considerationFulfillments,
        bytes32 fulfillerConduitKey,
        address recipient
    )
        internal
        returns (bool[] memory availableOrders, Execution[] memory executions)
    {
        // Retrieve length of offer fulfillments array and place on the stack.
        uint256 totalOfferFulfillments = offerFulfillments.length;

        // Retrieve length of consideration fulfillments array & place on stack.
        uint256 totalConsiderationFulfillments = (
            considerationFulfillments.length
        );

        // Allocate an execution for each offer and consideration fulfillment.
        executions = new Execution[](
            totalOfferFulfillments + totalConsiderationFulfillments
        );

        // Skip overflow checks as all for loops are indexed starting at zero.
        unchecked {
            // Track number of filtered executions.
            uint256 totalFilteredExecutions = 0;

            // Iterate over each offer fulfillment.
            for (uint256 i = 0; i < totalOfferFulfillments; ++i) {
                /// Retrieve the offer fulfillment components in question.
                FulfillmentComponent[] memory components = (
                    offerFulfillments[i]
                );

                // Derive aggregated execution corresponding with fulfillment.
                Execution memory execution = _aggregateAvailable(
                    advancedOrders,
                    Side.OFFER,
                    components,
                    fulfillerConduitKey,
                    recipient
                );

                // If offerer and recipient on the execution are the same...
                if (execution.item.recipient == execution.offerer) {
                    // Increment total filtered executions.
                    ++totalFilteredExecutions;
                } else {
                    // Otherwise, assign the execution to the executions array.
                    executions[i - totalFilteredExecutions] = execution;
                }
            }

            // Iterate over each consideration fulfillment.
            for (uint256 i = 0; i < totalConsiderationFulfillments; ++i) {
                /// Retrieve consideration fulfillment components in question.
                FulfillmentComponent[] memory components = (
                    considerationFulfillments[i]
                );

                // Derive aggregated execution corresponding with fulfillment.
                Execution memory execution = _aggregateAvailable(
                    advancedOrders,
                    Side.CONSIDERATION,
                    components,
                    fulfillerConduitKey,
                    address(0) // unused
                );

                // If offerer and recipient on the execution are the same...
                if (execution.item.recipient == execution.offerer) {
                    // Increment total filtered executions.
                    ++totalFilteredExecutions;
                } else {
                    // Otherwise, assign the execution to the executions array.
                    executions[
                        i + totalOfferFulfillments - totalFilteredExecutions
                    ] = execution;
                }
            }

            // If some number of executions have been filtered...
            if (totalFilteredExecutions != 0) {
                // reduce the total length of the executions array.
                assembly {
                    mstore(
                        executions,
                        sub(mload(executions), totalFilteredExecutions)
                    )
                }
            }
        }

        // Revert if no orders are available.
        if (executions.length == 0) {
            revert NoSpecifiedOrdersAvailable();
        }

        // Perform final checks and return.
        availableOrders = _performFinalChecksAndExecuteOrders(
            advancedOrders,
            executions
        );

        return (availableOrders, executions);
    }

    /**
     * @dev Internal function to perform a final check that each consideration
     *      item for an arbitrary number of fulfilled orders has been met and to
     *      trigger associated executions, transferring the respective items.
     *
     * @param advancedOrders     The orders to check and perform executions for.
     * @param executions         An array of elements indicating the sequence of
     *                           transfers to perform when fulfilling the given
     *                           orders.
     *
     * @return availableOrders An array of booleans indicating if each order
     *                         with an index corresponding to the index of the
     *                         returned boolean was fulfillable or not.
     */
    function _performFinalChecksAndExecuteOrders(
        AdvancedOrder[] memory advancedOrders,
        Execution[] memory executions
    ) internal returns (bool[] memory availableOrders) {
        // Retrieve the length of the advanced orders array and place on stack.
        uint256 totalOrders = advancedOrders.length;

        // Initialize array for tracking available orders.
        availableOrders = new bool[](totalOrders);

        // Skip overflow checks as all for loops are indexed starting at zero.
        unchecked {
            // Iterate over orders to ensure all considerations are met.
            for (uint256 i = 0; i < totalOrders; ++i) {
                // Retrieve the order in question.
                AdvancedOrder memory advancedOrder = advancedOrders[i];

                // Skip consideration item checks for order if not fulfilled.
                if (advancedOrder.numerator == 0) {
                    // Note: orders do not need to be marked as unavailable as a
                    // new memory region has been allocated. Review carefully if
                    // altering compiler version or managing memory manually.
                    continue;
                }

                // Mark the order as available.
                availableOrders[i] = true;

                // Retrieve consideration items to ensure they are fulfilled.
                ConsiderationItem[] memory consideration = (
                    advancedOrder.parameters.consideration
                );

                // Read length of consideration array and place on the stack.
                uint256 totalConsiderationItems = consideration.length;

                // Iterate over each consideration item to ensure it is met.
                for (uint256 j = 0; j < totalConsiderationItems; ++j) {
                    // Retrieve remaining amount on the consideration item.
                    uint256 unmetAmount = consideration[j].startAmount;

                    // Revert if the remaining amount is not zero.
                    if (unmetAmount != 0) {
                        revert ConsiderationNotMet(i, j, unmetAmount);
                    }
                }
            }
        }

        // Put ether value supplied by the caller on the stack.
        uint256 etherRemaining = msg.value;

        // Initialize an accumulator array. From this point forward, no new
        // memory regions can be safely allocated until the accumulator is no
        // longer being utilized, as the accumulator operates in an open-ended
        // fashion from this memory pointer; existing memory may still be
        // accessed and modified, however.
        bytes memory accumulator = new bytes(AccumulatorDisarmed);

        // Retrieve the length of the executions array and place on stack.
        uint256 totalExecutions = executions.length;

        // Iterate over each execution.
        for (uint256 i = 0; i < totalExecutions; ) {
            // Retrieve the execution and the associated received item.
            Execution memory execution = executions[i];
            ReceivedItem memory item = execution.item;

            // If execution transfers native tokens, reduce value available.
            if (item.itemType == ItemType.NATIVE) {
                // Ensure that sufficient native tokens are still available.
                if (item.amount > etherRemaining) {
                    revert InsufficientEtherSupplied();
                }

                // Skip underflow check as amount is less than ether remaining.
                unchecked {
                    etherRemaining -= item.amount;
                }
            }

            // Transfer the item specified by the execution.
            _transfer(
                item,
                execution.offerer,
                execution.conduitKey,
                accumulator
            );

            // Skip overflow check as for loop is indexed starting at zero.
            unchecked {
                ++i;
            }
        }

        // Trigger any remaining accumulated transfers via call to the conduit.
        _triggerIfArmed(accumulator);

        // If any ether remains after fulfillments, return it to the caller.
        if (etherRemaining != 0) {
            _transferEth(payable(msg.sender), etherRemaining);
        }

        // Clear the reentrancy guard.
        _clearReentrancyGuard();

        // Return the array containing available orders.
        return (availableOrders);
    }

    /**
     * @dev Internal function to match an arbitrary number of full or partial
     *      orders, each with an arbitrary number of items for offer and
     *      consideration, supplying criteria resolvers containing specific
     *      token identifiers and associated proofs as well as fulfillments
     *      allocating offer components to consideration components.
     *
     * @param advancedOrders    The advanced orders to match. Note that both the
     *                          offerer and fulfiller on each order must first
     *                          approve this contract (or their conduit if
     *                          indicated by the order) to transfer any relevant
     *                          tokens on their behalf and each consideration
     *                          recipient must implement `onERC1155Received` in
     *                          order to receive ERC1155 tokens. Also note that
     *                          the offer and consideration components for each
     *                          order must have no remainder after multiplying
     *                          the respective amount with the supplied fraction
     *                          in order for the group of partial fills to be
     *                          considered valid.
     * @param criteriaResolvers An array where each element contains a reference
     *                          to a specific order as well as that order's
     *                          offer or consideration, a token identifier, and
     *                          a proof that the supplied token identifier is
     *                          contained in the order's merkle root. Note that
     *                          an empty root indicates that any (transferable)
     *                          token identifier is valid and that no associated
     *                          proof needs to be supplied.
     * @param fulfillments      An array of elements allocating offer components
     *                          to consideration components. Note that each
     *                          consideration component must be fully met in
     *                          order for the match operation to be valid.
     *
     * @return executions An array of elements indicating the sequence of
     *                    transfers performed as part of matching the given
     *                    orders.
     */
    function _matchAdvancedOrders(
        AdvancedOrder[] memory advancedOrders,
        CriteriaResolver[] memory criteriaResolvers,
        Fulfillment[] calldata fulfillments
    ) internal returns (Execution[] memory executions) {
        // Validate orders, update order status, and determine item amounts.
        _validateOrdersAndPrepareToFulfill(
            advancedOrders,
            criteriaResolvers,
            true, // Signifies that invalid orders should revert.
            advancedOrders.length,
            address(0) // OrderFulfilled event has no recipient when matching.
        );

        // Fulfill the orders using the supplied fulfillments.
        return _fulfillAdvancedOrders(advancedOrders, fulfillments);
    }

    /**
     * @dev Internal function to fulfill an arbitrary number of orders, either
     *      full or partial, after validating, adjusting amounts, and applying
     *      criteria resolvers.
     *
     * @param advancedOrders     The orders to match, including a fraction to
     *                           attempt to fill for each order.
     * @param fulfillments       An array of elements allocating offer
     *                           components to consideration components. Note
     *                           that the final amount of each consideration
     *                           component must be zero for a match operation to
     *                           be considered valid.
     *
     * @return executions An array of elements indicating the sequence of
     *                    transfers performed as part of matching the given
     *                    orders.
     */
    function _fulfillAdvancedOrders(
        AdvancedOrder[] memory advancedOrders,
        Fulfillment[] calldata fulfillments
    ) internal returns (Execution[] memory executions) {
        // Retrieve fulfillments array length and place on the stack.
        uint256 totalFulfillments = fulfillments.length;

        // Allocate executions by fulfillment and apply them to each execution.
        executions = new Execution[](totalFulfillments);

        // Skip overflow checks as all for loops are indexed starting at zero.
        unchecked {
            // Track number of filtered executions.
            uint256 totalFilteredExecutions = 0;

            // Iterate over each fulfillment.
            for (uint256 i = 0; i < totalFulfillments; ++i) {
                /// Retrieve the fulfillment in question.
                Fulfillment calldata fulfillment = fulfillments[i];

                // Derive the execution corresponding with the fulfillment.
                Execution memory execution = _applyFulfillment(
                    advancedOrders,
                    fulfillment.offerComponents,
                    fulfillment.considerationComponents
                );

                // If offerer and recipient on the execution are the same...
                if (execution.item.recipient == execution.offerer) {
                    // Increment total filtered executions.
                    ++totalFilteredExecutions;
                } else {
                    // Otherwise, assign the execution to the executions array.
                    executions[i - totalFilteredExecutions] = execution;
                }
            }

            // If some number of executions have been filtered...
            if (totalFilteredExecutions != 0) {
                // reduce the total length of the executions array.
                assembly {
                    mstore(
                        executions,
                        sub(mload(executions), totalFilteredExecutions)
                    )
                }
            }
        }

        // Perform final checks and execute orders.
        _performFinalChecksAndExecuteOrders(advancedOrders, executions);

        // Return the executions array.
        return (executions);
    }
}
