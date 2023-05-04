// SPDX-License-Identifier: MIT
pragma solidity >=0.8.7;

/**
 * @title ZoneInteractionErrors
 * @author 0age
 * @notice ZoneInteractionErrors contains errors related to zone interaction.
 */
interface ZoneInteractionErrors {
    /**
     * @dev Revert with an error when attempting to fill an order that specifies
     *      a restricted submitter as its order type when not submitted by
     *      either the offerer or the order's zone or approved as valid by the
     *      zone in question via a staticcall to `isValidOrder`.
     *
     * @param orderHash The order hash for the invalid restricted order.
     */
    error InvalidRestrictedOrder(bytes32 orderHash);
}
