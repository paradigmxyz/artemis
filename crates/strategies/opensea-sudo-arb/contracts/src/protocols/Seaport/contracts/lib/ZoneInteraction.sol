// SPDX-License-Identifier: MIT
pragma solidity >=0.8.13;

import { ZoneInterface } from "../interfaces/ZoneInterface.sol";

import { OrderType } from "./ConsiderationEnums.sol";

// prettier-ignore
import { AdvancedOrder, CriteriaResolver } from "./ConsiderationStructs.sol";

import "./ConsiderationConstants.sol";

// prettier-ignore
import {
    ZoneInteractionErrors
} from "../interfaces/ZoneInteractionErrors.sol";

import { LowLevelHelpers } from "./LowLevelHelpers.sol";

/**
 * @title ZoneInteraction
 * @author 0age
 * @notice ZoneInteraction contains logic related to interacting with zones.
 */
contract ZoneInteraction is ZoneInteractionErrors, LowLevelHelpers {
    /**
     * @dev Internal view function to determine if an order has a restricted
     *      order type and, if so, to ensure that either the offerer or the zone
     *      are the fulfiller or that a staticcall to `isValidOrder` on the zone
     *      returns a magic value indicating that the order is currently valid.
     *
     * @param orderHash The hash of the order.
     * @param zoneHash  The hash to provide upon calling the zone.
     * @param orderType The type of the order.
     * @param offerer   The offerer in question.
     * @param zone      The zone in question.
     */
    function _assertRestrictedBasicOrderValidity(
        bytes32 orderHash,
        bytes32 zoneHash,
        OrderType orderType,
        address offerer,
        address zone
    ) internal view {
        // Order type 2-3 require zone or offerer be caller or zone to approve.
        if (
            uint256(orderType) > 1 &&
            msg.sender != zone &&
            msg.sender != offerer
        ) {
            // Perform minimal staticcall to the zone.
            _callIsValidOrder(zone, orderHash, offerer, zoneHash);
        }
    }

    function _callIsValidOrder(
        address zone,
        bytes32 orderHash,
        address offerer,
        bytes32 zoneHash
    ) internal view {
        // Perform minimal staticcall to the zone.
        bool success = _staticcall(
            zone,
            abi.encodeWithSelector(
                ZoneInterface.isValidOrder.selector,
                orderHash,
                msg.sender,
                offerer,
                zoneHash
            )
        );

        // Ensure call was successful and returned the correct magic value.
        _assertIsValidOrderStaticcallSuccess(success, orderHash);
    }

    /**
     * @dev Internal view function to determine whether an order is a restricted
     *      order and, if so, to ensure that it was either submitted by the
     *      offerer or the zone for the order, or that the zone returns the
     *      expected magic value upon performing a staticcall to `isValidOrder`
     *      or `isValidOrderIncludingExtraData` depending on whether the order
     *      fulfillment specifies extra data or criteria resolvers.
     *
     * @param advancedOrder     The advanced order in question.
     * @param criteriaResolvers An array where each element contains a reference
     *                          to a specific offer or consideration, a token
     *                          identifier, and a proof that the supplied token
     *                          identifier is contained in the order's merkle
     *                          root. Note that a criteria of zero indicates
     *                          that any (transferable) token identifier is
     *                          valid and that no proof needs to be supplied.
     * @param priorOrderHashes  The order hashes of each order supplied prior to
     *                          the current order as part of a "match" variety
     *                          of order fulfillment (e.g. this array will be
     *                          empty for single or "fulfill available").
     * @param orderHash         The hash of the order.
     * @param zoneHash          The hash to provide upon calling the zone.
     * @param orderType         The type of the order.
     * @param offerer           The offerer in question.
     * @param zone              The zone in question.
     */
    function _assertRestrictedAdvancedOrderValidity(
        AdvancedOrder memory advancedOrder,
        CriteriaResolver[] memory criteriaResolvers,
        bytes32[] memory priorOrderHashes,
        bytes32 orderHash,
        bytes32 zoneHash,
        OrderType orderType,
        address offerer,
        address zone
    ) internal view {
        // Order type 2-3 require zone or offerer be caller or zone to approve.
        if (
            uint256(orderType) > 1 &&
            msg.sender != zone &&
            msg.sender != offerer
        ) {
            // If no extraData or criteria resolvers are supplied...
            if (
                advancedOrder.extraData.length == 0 &&
                criteriaResolvers.length == 0
            ) {
                // Perform minimal staticcall to the zone.
                _callIsValidOrder(zone, orderHash, offerer, zoneHash);
            } else {
                // Otherwise, extra data or criteria resolvers were supplied; in
                // that event, perform a more verbose staticcall to the zone.
                bool success = _staticcall(
                    zone,
                    abi.encodeWithSelector(
                        ZoneInterface.isValidOrderIncludingExtraData.selector,
                        orderHash,
                        msg.sender,
                        advancedOrder,
                        priorOrderHashes,
                        criteriaResolvers
                    )
                );

                // Ensure call was successful and returned correct magic value.
                _assertIsValidOrderStaticcallSuccess(success, orderHash);
            }
        }
    }

    /**
     * @dev Internal view function to ensure that a staticcall to `isValidOrder`
     *      or `isValidOrderIncludingExtraData` as part of validating a
     *      restricted order that was not submitted by the named offerer or zone
     *      was successful and returned the required magic value.
     *
     * @param success   A boolean indicating the status of the staticcall.
     * @param orderHash The order hash of the order in question.
     */
    function _assertIsValidOrderStaticcallSuccess(
        bool success,
        bytes32 orderHash
    ) internal view {
        // If the call failed...
        if (!success) {
            // Revert and pass reason along if one was returned.
            _revertWithReasonIfOneIsReturned();

            // Otherwise, revert with a generic error message.
            revert InvalidRestrictedOrder(orderHash);
        }

        // Ensure result was extracted and matches isValidOrder magic value.
        if (_doesNotMatchMagic(ZoneInterface.isValidOrder.selector)) {
            revert InvalidRestrictedOrder(orderHash);
        }
    }
}
