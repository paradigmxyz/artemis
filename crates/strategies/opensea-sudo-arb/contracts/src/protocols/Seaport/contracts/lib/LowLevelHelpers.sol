// SPDX-License-Identifier: MIT
pragma solidity >=0.8.13;

import "./ConsiderationConstants.sol";

/**
 * @title LowLevelHelpers
 * @author 0age
 * @notice LowLevelHelpers contains logic for performing various low-level
 *         operations.
 */
contract LowLevelHelpers {
    /**
     * @dev Internal view function to staticcall an arbitrary target with given
     *      calldata. Note that no data is written to memory and no contract
     *      size check is performed.
     *
     * @param target   The account to staticcall.
     * @param callData The calldata to supply when staticcalling the target.
     *
     * @return success The status of the staticcall to the target.
     */
    function _staticcall(address target, bytes memory callData)
        internal
        view
        returns (bool success)
    {
        assembly {
            // Perform the staticcall.
            success := staticcall(
                gas(),
                target,
                add(callData, OneWord),
                mload(callData),
                0,
                0
            )
        }
    }

    /**
     * @dev Internal view function to revert and pass along the revert reason if
     *      data was returned by the last call and that the size of that data
     *      does not exceed the currently allocated memory size.
     */
    function _revertWithReasonIfOneIsReturned() internal view {
        assembly {
            // If it returned a message, bubble it up as long as sufficient gas
            // remains to do so:
            if returndatasize() {
                // Ensure that sufficient gas is available to copy returndata
                // while expanding memory where necessary. Start by computing
                // the word size of returndata and allocated memory.
                let returnDataWords := div(
                    add(returndatasize(), AlmostOneWord),
                    OneWord
                )

                // Note: use the free memory pointer in place of msize() to work
                // around a Yul warning that prevents accessing msize directly
                // when the IR pipeline is activated.
                let msizeWords := div(mload(FreeMemoryPointerSlot), OneWord)

                // Next, compute the cost of the returndatacopy.
                let cost := mul(CostPerWord, returnDataWords)

                // Then, compute cost of new memory allocation.
                if gt(returnDataWords, msizeWords) {
                    cost := add(
                        cost,
                        add(
                            mul(sub(returnDataWords, msizeWords), CostPerWord),
                            div(
                                sub(
                                    mul(returnDataWords, returnDataWords),
                                    mul(msizeWords, msizeWords)
                                ),
                                MemoryExpansionCoefficient
                            )
                        )
                    )
                }

                // Finally, add a small constant and compare to gas remaining;
                // bubble up the revert data if enough gas is still available.
                if lt(add(cost, ExtraGasBuffer), gas()) {
                    // Copy returndata to memory; overwrite existing memory.
                    returndatacopy(0, 0, returndatasize())

                    // Revert, specifying memory region with copied returndata.
                    revert(0, returndatasize())
                }
            }
        }
    }

    /**
     * @dev Internal pure function to determine if the first word of returndata
     *      matches an expected magic value.
     *
     * @param expected The expected magic value.
     *
     * @return A boolean indicating whether the expected value matches the one
     *         located in the first word of returndata.
     */
    function _doesNotMatchMagic(bytes4 expected) internal pure returns (bool) {
        // Declare a variable for the value held by the return data buffer.
        bytes4 result;

        // Utilize assembly in order to read directly from returndata buffer.
        assembly {
            // Only put result on stack if return data is exactly one word.
            if eq(returndatasize(), OneWord) {
                // Copy the word directly from return data into scratch space.
                returndatacopy(0, 0, OneWord)

                // Take value from scratch space and place it on the stack.
                result := mload(0)
            }
        }

        // Return a boolean indicating whether expected and located value match.
        return result != expected;
    }
}
