// SPDX-License-Identifier: MIT
pragma solidity >=0.8.13;

import { ItemType, Side } from "./ConsiderationEnums.sol";

// prettier-ignore
import {
    OfferItem,
    ConsiderationItem,
    ReceivedItem,
    OrderParameters,
    AdvancedOrder,
    Execution,
    FulfillmentComponent
} from "./ConsiderationStructs.sol";

import "./ConsiderationConstants.sol";

// prettier-ignore
import {
    FulfillmentApplicationErrors
} from "../interfaces/FulfillmentApplicationErrors.sol";

/**
 * @title FulfillmentApplier
 * @author 0age
 * @notice FulfillmentApplier contains logic related to applying fulfillments,
 *         both as part of order matching (where offer items are matched to
 *         consideration items) as well as fulfilling available orders (where
 *         order items and consideration items are independently aggregated).
 */
contract FulfillmentApplier is FulfillmentApplicationErrors {
    /**
     * @dev Internal pure function to match offer items to consideration items
     *      on a group of orders via a supplied fulfillment.
     *
     * @param advancedOrders          The orders to match.
     * @param offerComponents         An array designating offer components to
     *                                match to consideration components.
     * @param considerationComponents An array designating consideration
     *                                components to match to offer components.
     *                                Note that each consideration amount must
     *                                be zero in order for the match operation
     *                                to be valid.
     *
     * @return execution The transfer performed as a result of the fulfillment.
     */
    function _applyFulfillment(
        AdvancedOrder[] memory advancedOrders,
        FulfillmentComponent[] calldata offerComponents,
        FulfillmentComponent[] calldata considerationComponents
    ) internal pure returns (Execution memory execution) {
        // Ensure 1+ of both offer and consideration components are supplied.
        if (
            offerComponents.length == 0 || considerationComponents.length == 0
        ) {
            revert OfferAndConsiderationRequiredOnFulfillment();
        }

        // Declare a new Execution struct.
        Execution memory considerationExecution;

        // Validate & aggregate consideration items to new Execution object.
        _aggregateValidFulfillmentConsiderationItems(
            advancedOrders,
            considerationComponents,
            considerationExecution
        );

        // Retrieve the consideration item from the execution struct.
        ReceivedItem memory considerationItem = considerationExecution.item;

        // Recipient does not need to be specified because it will always be set
        // to that of the consideration.
        // Validate & aggregate offer items to Execution object.
        _aggregateValidFulfillmentOfferItems(
            advancedOrders,
            offerComponents,
            execution
        );

        // Ensure offer and consideration share types, tokens and identifiers.
        if (
            execution.item.itemType != considerationItem.itemType ||
            execution.item.token != considerationItem.token ||
            execution.item.identifier != considerationItem.identifier
        ) {
            revert MismatchedFulfillmentOfferAndConsiderationComponents();
        }

        // If total consideration amount exceeds the offer amount...
        if (considerationItem.amount > execution.item.amount) {
            // Retrieve the first consideration component from the fulfillment.
            FulfillmentComponent memory targetComponent = (
                considerationComponents[0]
            );

            // Skip underflow check as the conditional being true implies that
            // considerationItem.amount > execution.item.amount.
            unchecked {
                // Add excess consideration item amount to original order array.
                advancedOrders[targetComponent.orderIndex]
                    .parameters
                    .consideration[targetComponent.itemIndex]
                    .startAmount = (considerationItem.amount -
                    execution.item.amount);
            }

            // Reduce total consideration amount to equal the offer amount.
            considerationItem.amount = execution.item.amount;
        } else {
            // Retrieve the first offer component from the fulfillment.
            FulfillmentComponent memory targetComponent = offerComponents[0];

            // Skip underflow check as the conditional being false implies that
            // execution.item.amount >= considerationItem.amount.
            unchecked {
                // Add excess offer item amount to the original array of orders.
                advancedOrders[targetComponent.orderIndex]
                    .parameters
                    .offer[targetComponent.itemIndex]
                    .startAmount = (execution.item.amount -
                    considerationItem.amount);
            }

            // Reduce total offer amount to equal the consideration amount.
            execution.item.amount = considerationItem.amount;
        }

        // Reuse consideration recipient.
        execution.item.recipient = considerationItem.recipient;

        // Return the final execution that will be triggered for relevant items.
        return execution; // Execution(considerationItem, offerer, conduitKey);
    }

    /**
     * @dev Internal view function to aggregate offer or consideration items
     *      from a group of orders into a single execution via a supplied array
     *      of fulfillment components. Items that are not available to aggregate
     *      will not be included in the aggregated execution.
     *
     * @param advancedOrders        The orders to aggregate.
     * @param side                  The side (i.e. offer or consideration).
     * @param fulfillmentComponents An array designating item components to
     *                              aggregate if part of an available order.
     * @param fulfillerConduitKey   A bytes32 value indicating what conduit, if
     *                              any, to source the fulfiller's token
     *                              approvals from. The zero hash signifies that
     *                              no conduit should be used, with approvals
     *                              set directly on this contract.
     * @param recipient             The intended recipient for all received
     *                              items.
     *
     * @return execution The transfer performed as a result of the fulfillment.
     */
    function _aggregateAvailable(
        AdvancedOrder[] memory advancedOrders,
        Side side,
        FulfillmentComponent[] memory fulfillmentComponents,
        bytes32 fulfillerConduitKey,
        address recipient
    ) internal view returns (Execution memory execution) {
        // Skip overflow / underflow checks; conditions checked or unreachable.
        unchecked {
            // Retrieve fulfillment components array length and place on stack.
            // Ensure at least one fulfillment component has been supplied.
            if (fulfillmentComponents.length == 0) {
                revert MissingFulfillmentComponentOnAggregation(side);
            }

            // If the fulfillment components are offer components...
            if (side == Side.OFFER) {
                // Set the supplied recipient on the execution item.
                execution.item.recipient = payable(recipient);

                // Return execution for aggregated items provided by offerer.
                _aggregateValidFulfillmentOfferItems(
                    advancedOrders,
                    fulfillmentComponents,
                    execution
                );
            } else {
                // Otherwise, fulfillment components are consideration
                // components. Return execution for aggregated items provided by
                // the fulfiller.
                _aggregateValidFulfillmentConsiderationItems(
                    advancedOrders,
                    fulfillmentComponents,
                    execution
                );

                // Set the caller as the offerer on the execution.
                execution.offerer = msg.sender;

                // Set fulfiller conduit key as the conduit key on execution.
                execution.conduitKey = fulfillerConduitKey;
            }

            // Set the offerer and recipient to null address if execution
            // amount is zero. This will cause the execution item to be skipped.
            if (execution.item.amount == 0) {
                execution.offerer = address(0);
                execution.item.recipient = payable(0);
            }
        }
    }

    /**
     * @dev Internal pure function to aggregate a group of offer items using
     *      supplied directives on which component items are candidates for
     *      aggregation, skipping items on orders that are not available.
     *
     * @param advancedOrders  The orders to aggregate offer items from.
     * @param offerComponents An array of FulfillmentComponent structs
     *                        indicating the order index and item index of each
     *                        candidate offer item for aggregation.
     * @param execution       The execution to apply the aggregation to.
     */
    function _aggregateValidFulfillmentOfferItems(
        AdvancedOrder[] memory advancedOrders,
        FulfillmentComponent[] memory offerComponents,
        Execution memory execution
    ) internal pure {
        assembly {
            // Declare function for reverts on invalid fulfillment data.
            function throwInvalidFulfillmentComponentData() {
                // Store the InvalidFulfillmentComponentData error signature.
                mstore(0, InvalidFulfillmentComponentData_error_signature)

                // Return, supplying InvalidFulfillmentComponentData signature.
                revert(0, InvalidFulfillmentComponentData_error_len)
            }

            // Declare function for reverts due to arithmetic overflows.
            function throwOverflow() {
                // Store the Panic error signature.
                mstore(0, Panic_error_signature)

                // Store the arithmetic (0x11) panic code as initial argument.
                mstore(Panic_error_offset, Panic_arithmetic)

                // Return, supplying Panic signature and arithmetic code.
                revert(0, Panic_error_length)
            }

            // Get position in offerComponents head.
            let fulfillmentHeadPtr := add(offerComponents, OneWord)

            // Retrieve the order index using the fulfillment pointer.
            let orderIndex := mload(mload(fulfillmentHeadPtr))

            // Ensure that the order index is not out of range.
            if iszero(lt(orderIndex, mload(advancedOrders))) {
                throwInvalidFulfillmentComponentData()
            }

            // Read advancedOrders[orderIndex] pointer from its array head.
            let orderPtr := mload(
                // Calculate head position of advancedOrders[orderIndex].
                add(add(advancedOrders, OneWord), mul(orderIndex, OneWord))
            )

            // Read the pointer to OrderParameters from the AdvancedOrder.
            let paramsPtr := mload(orderPtr)

            // Load the offer array pointer.
            let offerArrPtr := mload(
                add(paramsPtr, OrderParameters_offer_head_offset)
            )

            // Retrieve item index using an offset of the fulfillment pointer.
            let itemIndex := mload(
                add(mload(fulfillmentHeadPtr), Fulfillment_itemIndex_offset)
            )

            // Only continue if the fulfillment is not invalid.
            if iszero(lt(itemIndex, mload(offerArrPtr))) {
                throwInvalidFulfillmentComponentData()
            }

            // Retrieve consideration item pointer using the item index.
            let offerItemPtr := mload(
                add(
                    // Get pointer to beginning of receivedItem.
                    add(offerArrPtr, OneWord),
                    // Calculate offset to pointer for desired order.
                    mul(itemIndex, OneWord)
                )
            )

            // Declare a variable for the final aggregated item amount.
            let amount := 0

            // Create variable to track errors encountered with amount.
            let errorBuffer := 0

            // Only add offer amount to execution amount on a nonzero numerator.
            if mload(add(orderPtr, AdvancedOrder_numerator_offset)) {
                // Retrieve amount pointer using consideration item pointer.
                let amountPtr := add(offerItemPtr, Common_amount_offset)

                // Set the amount.
                amount := mload(amountPtr)

                // Zero out amount on item to indicate it is credited.
                mstore(amountPtr, 0)

                // Buffer indicating whether issues were found.
                errorBuffer := iszero(amount)
            }

            // Retrieve the received item pointer.
            let receivedItemPtr := mload(execution)

            // Set the item type on the received item.
            mstore(receivedItemPtr, mload(offerItemPtr))

            // Set the token on the received item.
            mstore(
                add(receivedItemPtr, Common_token_offset),
                mload(add(offerItemPtr, Common_token_offset))
            )

            // Set the identifier on the received item.
            mstore(
                add(receivedItemPtr, Common_identifier_offset),
                mload(add(offerItemPtr, Common_identifier_offset))
            )

            // Set the offerer on returned execution using order pointer.
            mstore(add(execution, Execution_offerer_offset), mload(paramsPtr))

            // Set conduitKey on returned execution via offset of order pointer.
            mstore(
                add(execution, Execution_conduit_offset),
                mload(add(paramsPtr, OrderParameters_conduit_offset))
            )

            // Calculate the hash of (itemType, token, identifier).
            let dataHash := keccak256(
                receivedItemPtr,
                ReceivedItem_CommonParams_size
            )

            // Get position one word past last element in head of array.
            let endPtr := add(
                offerComponents,
                mul(mload(offerComponents), OneWord)
            )

            // Iterate over remaining offer components.
            // prettier-ignore
            for {} lt(fulfillmentHeadPtr,  endPtr) {} {
                // Increment the pointer to the fulfillment head by one word.
                fulfillmentHeadPtr := add(fulfillmentHeadPtr, OneWord)

                // Get the order index using the fulfillment pointer.
                orderIndex := mload(mload(fulfillmentHeadPtr))

                // Ensure the order index is in range.
                if iszero(lt(orderIndex, mload(advancedOrders))) {
                  throwInvalidFulfillmentComponentData()
                }

                // Get pointer to AdvancedOrder element.
                orderPtr := mload(
                    add(
                        add(advancedOrders, OneWord),
                        mul(orderIndex, OneWord)
                    )
                )

                // Only continue if numerator is not zero.
                if iszero(mload(
                    add(orderPtr, AdvancedOrder_numerator_offset)
                )) {
                  continue
                }

                // Read the pointer to OrderParameters from the AdvancedOrder.
                paramsPtr := mload(orderPtr)

                // Load offer array pointer.
                offerArrPtr := mload(
                    add(
                        paramsPtr,
                        OrderParameters_offer_head_offset
                    )
                )

                // Get the item index using the fulfillment pointer.
                itemIndex := mload(add(mload(fulfillmentHeadPtr), OneWord))

                // Throw if itemIndex is out of the range of array.
                if iszero(
                    lt(itemIndex, mload(offerArrPtr))
                ) {
                    throwInvalidFulfillmentComponentData()
                }

                // Retrieve offer item pointer using index.
                offerItemPtr := mload(
                    add(
                        // Get pointer to beginning of receivedItem.
                        add(offerArrPtr, OneWord),
                        // Use offset to pointer for desired order.
                        mul(itemIndex, OneWord)
                    )
                )

                // Retrieve amount pointer using offer item pointer.
                let amountPtr := add(
                      offerItemPtr,
                      Common_amount_offset
                )

                // Add offer amount to execution amount.
                let newAmount := add(amount, mload(amountPtr))

                // Update error buffer: 1 = zero amount, 2 = overflow, 3 = both.
                errorBuffer := or(
                  errorBuffer,
                  or(
                    shl(1, lt(newAmount, amount)),
                    iszero(mload(amountPtr))
                  )
                )

                // Update the amount to the new, summed amount.
                amount := newAmount

                // Zero out amount on original item to indicate it is credited.
                mstore(amountPtr, 0)

                // Ensure the indicated item matches original item.
                if iszero(
                    and(
                        and(
                          // The offerer must match on both items.
                          eq(
                              mload(paramsPtr),
                              mload(
                                  add(execution, Execution_offerer_offset)
                              )
                          ),
                          // The conduit key must match on both items.
                          eq(
                              mload(
                                  add(
                                      paramsPtr,
                                      OrderParameters_conduit_offset
                                  )
                              ),
                              mload(
                                  add(
                                      execution,
                                      Execution_conduit_offset
                                  )
                              )
                          )
                        ),
                        // The itemType, token, and identifier must match.
                        eq(
                            dataHash,
                            keccak256(
                                offerItemPtr,
                                ReceivedItem_CommonParams_size
                            )
                        )
                    )
                ) {
                    // Throw if any of the requirements are not met.
                    throwInvalidFulfillmentComponentData()
                }
            }
            // Write final amount to execution.
            mstore(add(mload(execution), Common_amount_offset), amount)

            // Determine whether the error buffer contains a nonzero error code.
            if errorBuffer {
                // If errorBuffer is 1, an item had an amount of zero.
                if eq(errorBuffer, 1) {
                    // Store the MissingItemAmount error signature.
                    mstore(0, MissingItemAmount_error_signature)

                    // Return, supplying MissingItemAmount signature.
                    revert(0, MissingItemAmount_error_len)
                }

                // If errorBuffer is not 1 or 0, the sum overflowed.
                // Panic!
                throwOverflow()
            }
        }
    }

    /**
     * @dev Internal pure function to aggregate a group of consideration items
     *      using supplied directives on which component items are candidates
     *      for aggregation, skipping items on orders that are not available.
     *
     * @param advancedOrders          The orders to aggregate consideration
     *                                items from.
     * @param considerationComponents An array of FulfillmentComponent structs
     *                                indicating the order index and item index
     *                                of each candidate consideration item for
     *                                aggregation.
     * @param execution       The execution to apply the aggregation to.
     */
    function _aggregateValidFulfillmentConsiderationItems(
        AdvancedOrder[] memory advancedOrders,
        FulfillmentComponent[] memory considerationComponents,
        Execution memory execution
    ) internal pure {
        // Utilize assembly in order to efficiently aggregate the items.
        assembly {
            // Declare function for reverts on invalid fulfillment data.
            function throwInvalidFulfillmentComponentData() {
                // Store the InvalidFulfillmentComponentData error signature.
                mstore(0, InvalidFulfillmentComponentData_error_signature)

                // Return, supplying InvalidFulfillmentComponentData signature.
                revert(0, InvalidFulfillmentComponentData_error_len)
            }

            // Declare function for reverts due to arithmetic overflows.
            function throwOverflow() {
                // Store the Panic error signature.
                mstore(0, Panic_error_signature)

                // Store the arithmetic (0x11) panic code as initial argument.
                mstore(Panic_error_offset, Panic_arithmetic)

                // Return, supplying Panic signature and arithmetic code.
                revert(0, Panic_error_length)
            }

            // Get position in considerationComponents head.
            let fulfillmentHeadPtr := add(considerationComponents, OneWord)

            // Retrieve the order index using the fulfillment pointer.
            let orderIndex := mload(mload(fulfillmentHeadPtr))

            // Ensure that the order index is not out of range.
            if iszero(lt(orderIndex, mload(advancedOrders))) {
                throwInvalidFulfillmentComponentData()
            }

            // Read advancedOrders[orderIndex] pointer from its array head.
            let orderPtr := mload(
                // Calculate head position of advancedOrders[orderIndex].
                add(add(advancedOrders, OneWord), mul(orderIndex, OneWord))
            )

            // Load consideration array pointer.
            let considerationArrPtr := mload(
                add(
                    // Read pointer to OrderParameters from the AdvancedOrder.
                    mload(orderPtr),
                    OrderParameters_consideration_head_offset
                )
            )

            // Retrieve item index using an offset of the fulfillment pointer.
            let itemIndex := mload(
                add(mload(fulfillmentHeadPtr), Fulfillment_itemIndex_offset)
            )

            // Ensure that the order index is not out of range.
            if iszero(lt(itemIndex, mload(considerationArrPtr))) {
                throwInvalidFulfillmentComponentData()
            }

            // Retrieve consideration item pointer using the item index.
            let considerationItemPtr := mload(
                add(
                    // Get pointer to beginning of receivedItem.
                    add(considerationArrPtr, OneWord),
                    // Calculate offset to pointer for desired order.
                    mul(itemIndex, OneWord)
                )
            )

            // Declare a variable for the final aggregated item amount.
            let amount := 0

            // Create variable to track errors encountered with amount.
            let errorBuffer := 0

            // Only add consideration amount to execution amount if numerator is
            // greater than zero.
            if mload(add(orderPtr, AdvancedOrder_numerator_offset)) {
                // Retrieve amount pointer using consideration item pointer.
                let amountPtr := add(considerationItemPtr, Common_amount_offset)

                // Set the amount.
                amount := mload(amountPtr)

                // Set error bit if amount is zero.
                errorBuffer := iszero(amount)

                // Zero out amount on item to indicate it is credited.
                mstore(amountPtr, 0)
            }

            // Retrieve ReceivedItem pointer from Execution.
            let receivedItem := mload(execution)

            // Set the item type on the received item.
            mstore(receivedItem, mload(considerationItemPtr))

            // Set the token on the received item.
            mstore(
                add(receivedItem, Common_token_offset),
                mload(add(considerationItemPtr, Common_token_offset))
            )

            // Set the identifier on the received item.
            mstore(
                add(receivedItem, Common_identifier_offset),
                mload(add(considerationItemPtr, Common_identifier_offset))
            )

            // Set the recipient on the received item.
            mstore(
                add(receivedItem, ReceivedItem_recipient_offset),
                mload(
                    add(
                        considerationItemPtr,
                        ConsiderationItem_recipient_offset
                    )
                )
            )

            // Calculate the hash of (itemType, token, identifier).
            let dataHash := keccak256(
                receivedItem,
                ReceivedItem_CommonParams_size
            )

            // Get position one word past last element in head of array.
            let endPtr := add(
                considerationComponents,
                mul(mload(considerationComponents), OneWord)
            )

            // Iterate over remaining offer components.
            // prettier-ignore
            for {} lt(fulfillmentHeadPtr,  endPtr) {} {
                // Increment position in considerationComponents head.
                fulfillmentHeadPtr := add(fulfillmentHeadPtr, OneWord)

                // Get the order index using the fulfillment pointer.
                orderIndex := mload(mload(fulfillmentHeadPtr))

                // Ensure the order index is in range.
                if iszero(lt(orderIndex, mload(advancedOrders))) {
                  throwInvalidFulfillmentComponentData()
                }

                // Get pointer to AdvancedOrder element.
                orderPtr := mload(
                    add(
                        add(advancedOrders, OneWord),
                        mul(orderIndex, OneWord)
                    )
                )

                // Only continue if numerator is not zero.
                if iszero(
                    mload(add(orderPtr, AdvancedOrder_numerator_offset))
                ) {
                  continue
                }

                // Load consideration array pointer from OrderParameters.
                considerationArrPtr := mload(
                    add(
                        // Get pointer to OrderParameters from AdvancedOrder.
                        mload(orderPtr),
                        OrderParameters_consideration_head_offset
                    )
                )

                // Get the item index using the fulfillment pointer.
                itemIndex := mload(add(mload(fulfillmentHeadPtr), OneWord))

                // Check if itemIndex is within the range of array.
                if iszero(lt(itemIndex, mload(considerationArrPtr))) {
                    throwInvalidFulfillmentComponentData()
                }

                // Retrieve consideration item pointer using index.
                considerationItemPtr := mload(
                    add(
                        // Get pointer to beginning of receivedItem.
                        add(considerationArrPtr, OneWord),
                        // Use offset to pointer for desired order.
                        mul(itemIndex, OneWord)
                    )
                )

                // Retrieve amount pointer using consideration item pointer.
                let amountPtr := add(
                      considerationItemPtr,
                      Common_amount_offset
                )

                // Add offer amount to execution amount.
                let newAmount := add(amount, mload(amountPtr))

                // Update error buffer: 1 = zero amount, 2 = overflow, 3 = both.
                errorBuffer := or(
                  errorBuffer,
                  or(
                    shl(1, lt(newAmount, amount)),
                    iszero(mload(amountPtr))
                  )
                )

                // Update the amount to the new, summed amount.
                amount := newAmount

                // Zero out amount on original item to indicate it is credited.
                mstore(amountPtr, 0)

                // Ensure the indicated item matches original item.
                if iszero(
                    and(
                        // Item recipients must match.
                        eq(
                            mload(
                                add(
                                    considerationItemPtr,
                                    ConsiderItem_recipient_offset
                                )
                            ),
                            mload(
                                add(
                                    receivedItem,
                                    ReceivedItem_recipient_offset
                                )
                            )
                        ),
                        // The itemType, token, identifier must match.
                        eq(
                          dataHash,
                          keccak256(
                            considerationItemPtr,
                            ReceivedItem_CommonParams_size
                          )
                        )
                    )
                ) {
                    // Throw if any of the requirements are not met.
                    throwInvalidFulfillmentComponentData()
                }
            }
            // Write final amount to execution.
            mstore(add(receivedItem, Common_amount_offset), amount)

            // Determine whether the error buffer contains a nonzero error code.
            if errorBuffer {
                // If errorBuffer is 1, an item had an amount of zero.
                if eq(errorBuffer, 1) {
                    // Store the MissingItemAmount error signature.
                    mstore(0, MissingItemAmount_error_signature)

                    // Return, supplying MissingItemAmount signature.
                    revert(0, MissingItemAmount_error_len)
                }

                // If errorBuffer is not 1 or 0, the sum overflowed.
                // Panic!
                throwOverflow()
            }
        }
    }
}
