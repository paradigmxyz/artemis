// SPDX-License-Identifier: MIT
pragma solidity >=0.8.7;

/**
 * @title AmountDerivationErrors
 * @author 0age
 * @notice AmountDerivationErrors contains errors related to amount derivation.
 */
interface AmountDerivationErrors {
    /**
     * @dev Revert with an error when attempting to apply a fraction as part of
     *      a partial fill that does not divide the target amount cleanly.
     */
    error InexactFraction();
}
