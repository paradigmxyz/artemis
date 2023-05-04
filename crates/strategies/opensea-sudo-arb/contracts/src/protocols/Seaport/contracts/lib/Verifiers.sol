// SPDX-License-Identifier: MIT
pragma solidity >=0.8.13;

import { OrderStatus } from "./ConsiderationStructs.sol";

import { Assertions } from "./Assertions.sol";

import { SignatureVerification } from "./SignatureVerification.sol";

/**
 * @title Verifiers
 * @author 0age
 * @notice Verifiers contains functions for performing verifications.
 */
contract Verifiers is Assertions, SignatureVerification {
    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) Assertions(conduitController) {}

    /**
     * @dev Internal view function to ensure that the current time falls within
     *      an order's valid timespan.
     *
     * @param startTime       The time at which the order becomes active.
     * @param endTime         The time at which the order becomes inactive.
     * @param revertOnInvalid A boolean indicating whether to revert if the
     *                        order is not active.
     *
     * @return valid A boolean indicating whether the order is active.
     */
    function _verifyTime(
        uint256 startTime,
        uint256 endTime,
        bool revertOnInvalid
    ) internal view returns (bool valid) {
        // Revert if order's timespan hasn't started yet or has already ended.
        if (startTime > block.timestamp || endTime <= block.timestamp) {
            // Only revert if revertOnInvalid has been supplied as true.
            if (revertOnInvalid) {
                revert InvalidTime();
            }

            // Return false as the order is invalid.
            return false;
        }

        // Return true as the order time is valid.
        valid = true;
    }

    /**
     * @dev Internal view function to verify the signature of an order. An
     *      ERC-1271 fallback will be attempted if either the signature length
     *      is not 32 or 33 bytes or if the recovered signer does not match the
     *      supplied offerer. Note that in cases where a 32 or 33 byte signature
     *      is supplied, only standard ECDSA signatures that recover to a
     *      non-zero address are supported.
     *
     * @param offerer   The offerer for the order.
     * @param orderHash The order hash.
     * @param signature A signature from the offerer indicating that the order
     *                  has been approved.
     */
    function _verifySignature(
        address offerer,
        bytes32 orderHash,
        bytes memory signature
    ) internal view {
        // Skip signature verification if the offerer is the caller.
        if (offerer == msg.sender) {
            return;
        }

        // Derive EIP-712 digest using the domain separator and the order hash.
        bytes32 digest = _deriveEIP712Digest(_domainSeparator(), orderHash);

        // Ensure that the signature for the digest is valid for the offerer.
        _assertValidSignature(offerer, digest, signature);
    }

    /**
     * @dev Internal view function to validate that a given order is fillable
     *      and not cancelled based on the order status.
     *
     * @param orderHash       The order hash.
     * @param orderStatus     The status of the order, including whether it has
     *                        been cancelled and the fraction filled.
     * @param onlyAllowUnused A boolean flag indicating whether partial fills
     *                        are supported by the calling function.
     * @param revertOnInvalid A boolean indicating whether to revert if the
     *                        order has been cancelled or filled beyond the
     *                        allowable amount.
     *
     * @return valid A boolean indicating whether the order is valid.
     */
    function _verifyOrderStatus(
        bytes32 orderHash,
        OrderStatus storage orderStatus,
        bool onlyAllowUnused,
        bool revertOnInvalid
    ) internal view returns (bool valid) {
        // Ensure that the order has not been cancelled.
        if (orderStatus.isCancelled) {
            // Only revert if revertOnInvalid has been supplied as true.
            if (revertOnInvalid) {
                revert OrderIsCancelled(orderHash);
            }

            // Return false as the order status is invalid.
            return false;
        }

        // Read order status numerator from storage and place on stack.
        uint256 orderStatusNumerator = orderStatus.numerator;

        // If the order is not entirely unused...
        if (orderStatusNumerator != 0) {
            // ensure the order has not been partially filled when not allowed.
            if (onlyAllowUnused) {
                // Always revert on partial fills when onlyAllowUnused is true.
                revert OrderPartiallyFilled(orderHash);
            }
            // Otherwise, ensure that order has not been entirely filled.
            else if (orderStatusNumerator >= orderStatus.denominator) {
                // Only revert if revertOnInvalid has been supplied as true.
                if (revertOnInvalid) {
                    revert OrderAlreadyFilled(orderHash);
                }

                // Return false as the order status is invalid.
                return false;
            }
        }

        // Return true as the order status is valid.
        valid = true;
    }
}
