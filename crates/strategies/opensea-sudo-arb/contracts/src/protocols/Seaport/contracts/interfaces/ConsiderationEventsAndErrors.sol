// SPDX-License-Identifier: MIT
pragma solidity >=0.8.7;

import { SpentItem, ReceivedItem } from "../lib/ConsiderationStructs.sol";

/**
 * @title ConsiderationEventsAndErrors
 * @author 0age
 * @notice ConsiderationEventsAndErrors contains all events and errors.
 */
interface ConsiderationEventsAndErrors {
    /**
     * @dev Emit an event whenever an order is successfully fulfilled.
     *
     * @param orderHash     The hash of the fulfilled order.
     * @param offerer       The offerer of the fulfilled order.
     * @param zone          The zone of the fulfilled order.
     * @param recipient     The recipient of each spent item on the fulfilled
     *                      order, or the null address if there is no specific
     *                      fulfiller (i.e. the order is part of a group of
     *                      orders). Defaults to the caller unless explicitly
     *                      specified otherwise by the fulfiller.
     * @param offer         The offer items spent as part of the order.
     * @param consideration The consideration items received as part of the
     *                      order along with the recipients of each item.
     */
    event OrderFulfilled(
        bytes32 orderHash,
        address indexed offerer,
        address indexed zone,
        address recipient,
        SpentItem[] offer,
        ReceivedItem[] consideration
    );

    /**
     * @dev Emit an event whenever an order is successfully cancelled.
     *
     * @param orderHash The hash of the cancelled order.
     * @param offerer   The offerer of the cancelled order.
     * @param zone      The zone of the cancelled order.
     */
    event OrderCancelled(
        bytes32 orderHash,
        address indexed offerer,
        address indexed zone
    );

    /**
     * @dev Emit an event whenever an order is explicitly validated. Note that
     *      this event will not be emitted on partial fills even though they do
     *      validate the order as part of partial fulfillment.
     *
     * @param orderHash The hash of the validated order.
     * @param offerer   The offerer of the validated order.
     * @param zone      The zone of the validated order.
     */
    event OrderValidated(
        bytes32 orderHash,
        address indexed offerer,
        address indexed zone
    );

    /**
     * @dev Emit an event whenever a counter for a given offerer is incremented.
     *
     * @param newCounter The new counter for the offerer.
     * @param offerer  The offerer in question.
     */
    event CounterIncremented(uint256 newCounter, address indexed offerer);

    /**
     * @dev Revert with an error when attempting to fill an order that has
     *      already been fully filled.
     *
     * @param orderHash The order hash on which a fill was attempted.
     */
    error OrderAlreadyFilled(bytes32 orderHash);

    /**
     * @dev Revert with an error when attempting to fill an order outside the
     *      specified start time and end time.
     */
    error InvalidTime();

    /**
     * @dev Revert with an error when attempting to fill an order referencing an
     *      invalid conduit (i.e. one that has not been deployed).
     */
    error InvalidConduit(bytes32 conduitKey, address conduit);

    /**
     * @dev Revert with an error when an order is supplied for fulfillment with
     *      a consideration array that is shorter than the original array.
     */
    error MissingOriginalConsiderationItems();

    /**
     * @dev Revert with an error when a call to a conduit fails with revert data
     *      that is too expensive to return.
     */
    error InvalidCallToConduit(address conduit);

    /**
     * @dev Revert with an error if a consideration amount has not been fully
     *      zeroed out after applying all fulfillments.
     *
     * @param orderIndex         The index of the order with the consideration
     *                           item with a shortfall.
     * @param considerationIndex The index of the consideration item on the
     *                           order.
     * @param shortfallAmount    The unfulfilled consideration amount.
     */
    error ConsiderationNotMet(
        uint256 orderIndex,
        uint256 considerationIndex,
        uint256 shortfallAmount
    );

    /**
     * @dev Revert with an error when insufficient ether is supplied as part of
     *      msg.value when fulfilling orders.
     */
    error InsufficientEtherSupplied();

    /**
     * @dev Revert with an error when an ether transfer reverts.
     */
    error EtherTransferGenericFailure(address account, uint256 amount);

    /**
     * @dev Revert with an error when a partial fill is attempted on an order
     *      that does not specify partial fill support in its order type.
     */
    error PartialFillsNotEnabledForOrder();

    /**
     * @dev Revert with an error when attempting to fill an order that has been
     *      cancelled.
     *
     * @param orderHash The hash of the cancelled order.
     */
    error OrderIsCancelled(bytes32 orderHash);

    /**
     * @dev Revert with an error when attempting to fill a basic order that has
     *      been partially filled.
     *
     * @param orderHash The hash of the partially used order.
     */
    error OrderPartiallyFilled(bytes32 orderHash);

    /**
     * @dev Revert with an error when attempting to cancel an order as a caller
     *      other than the indicated offerer or zone.
     */
    error InvalidCanceller();

    /**
     * @dev Revert with an error when supplying a fraction with a value of zero
     *      for the numerator or denominator, or one where the numerator exceeds
     *      the denominator.
     */
    error BadFraction();

    /**
     * @dev Revert with an error when a caller attempts to supply callvalue to a
     *      non-payable basic order route or does not supply any callvalue to a
     *      payable basic order route.
     */
    error InvalidMsgValue(uint256 value);

    /**
     * @dev Revert with an error when attempting to fill a basic order using
     *      calldata not produced by default ABI encoding.
     */
    error InvalidBasicOrderParameterEncoding();

    /**
     * @dev Revert with an error when attempting to fulfill any number of
     *      available orders when none are fulfillable.
     */
    error NoSpecifiedOrdersAvailable();

    /**
     * @dev Revert with an error when attempting to fulfill an order with an
     *      offer for ETH outside of matching orders.
     */
    error InvalidNativeOfferItem();
}
