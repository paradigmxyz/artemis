// SPDX-License-Identifier: MIT
pragma solidity >=0.8.7;

import { TransferHelperItem } from "../helpers/TransferHelperStructs.sol";

interface TransferHelperInterface {
    /**
     * @dev Revert with an error when attempting to execute transfers with a
     *      NATIVE itemType.
     */
    error InvalidItemType();

    /**
     * @notice Transfer multiple items to a single recipient.
     *
     * @param items The items to transfer.
     * @param recipient  The address the items should be transferred to.
     * @param conduitKey  The key of the conduit performing the bulk transfer.
     */
    function bulkTransfer(
        TransferHelperItem[] calldata items,
        address recipient,
        bytes32 conduitKey
    ) external returns (bytes4);
}
