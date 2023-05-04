// SPDX-License-Identifier: MIT
pragma solidity >=0.8.7;

import { Side } from "../lib/ConsiderationEnums.sol";

/**
 * @title FulfillmentApplicationErrors
 * @author 0age
 * @notice FulfillmentApplicationErrors contains errors related to fulfillment
 *         application and aggregation.
 */
interface FulfillmentApplicationErrors {
    /**
     * @dev Revert with an error when a fulfillment is provided that does not
     *      declare at least one component as part of a call to fulfill
     *      available orders.
     */
    error MissingFulfillmentComponentOnAggregation(Side side);

    /**
     * @dev Revert with an error when a fulfillment is provided that does not
     *      declare at least one offer component and at least one consideration
     *      component.
     */
    error OfferAndConsiderationRequiredOnFulfillment();

    /**
     * @dev Revert with an error when the initial offer item named by a
     *      fulfillment component does not match the type, token, identifier,
     *      or conduit preference of the initial consideration item.
     */
    error MismatchedFulfillmentOfferAndConsiderationComponents();

    /**
     * @dev Revert with an error when an order or item index are out of range
     *      or a fulfillment component does not match the type, token,
     *      identifier, or conduit preference of the initial consideration item.
     */
    error InvalidFulfillmentComponentData();
}
