// SPDX-License-Identifier: MIT
pragma solidity >=0.8.7;

import "./TransferHelperStructs.sol";

import { TokenTransferrer } from "../lib/TokenTransferrer.sol";

import { ConduitInterface } from "../interfaces/ConduitInterface.sol";

// prettier-ignore
import {
    ConduitControllerInterface
} from "../interfaces/ConduitControllerInterface.sol";

import { Conduit } from "../conduit/Conduit.sol";

import { ConduitTransfer } from "../conduit/lib/ConduitStructs.sol";

// prettier-ignore
import {
    TransferHelperInterface
} from "../interfaces/TransferHelperInterface.sol";

/**
 * @title TransferHelper
 * @author stuckinaboot, stephankmin
 * @notice TransferHelper is a utility contract for transferring
 *         ERC20/ERC721/ERC1155 items in bulk to a specific recipient.
 */
contract TransferHelper is TransferHelperInterface, TokenTransferrer {
    // Allow for interaction with the conduit controller.
    ConduitControllerInterface internal immutable _CONDUIT_CONTROLLER;

    // Cache the conduit creation hash used by the conduit controller.
    bytes32 internal immutable _CONDUIT_CREATION_CODE_HASH;

    /**
     * @dev Set the supplied conduit controller and retrieve its
     *      conduit creation code hash.
     *
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) {
        // Get the conduit creation code hash from the supplied conduit
        // controller and set it as an immutable.
        ConduitControllerInterface controller = ConduitControllerInterface(
            conduitController
        );
        (_CONDUIT_CREATION_CODE_HASH, ) = controller.getConduitCodeHashes();

        // Set the supplied conduit controller as an immutable.
        _CONDUIT_CONTROLLER = controller;
    }

    /**
     * @notice Transfer multiple items to a single recipient.
     *
     * @param items      The items to transfer.
     * @param recipient  The address the items should be transferred to.
     * @param conduitKey The key of the conduit through which the bulk transfer
     *                   should occur.
     *
     * @return magicValue A value indicating that the transfers were successful.
     */
    function bulkTransfer(
        TransferHelperItem[] calldata items,
        address recipient,
        bytes32 conduitKey
    ) external override returns (bytes4 magicValue) {
        // Retrieve total number of transfers and place on stack.
        uint256 totalTransfers = items.length;

        // If no conduitKey is given, use TokenTransferrer to perform transfers.
        if (conduitKey == bytes32(0)) {
            // Skip overflow checks: all for loops are indexed starting at zero.
            unchecked {
                // Iterate over each transfer.
                for (uint256 i = 0; i < totalTransfers; ++i) {
                    // Retrieve the transfer in question.
                    TransferHelperItem calldata item = items[i];

                    // Perform a transfer based on the transfer's item type.
                    // Revert if item being transferred is a native token.
                    if (item.itemType == ConduitItemType.NATIVE) {
                        revert InvalidItemType();
                    } else if (item.itemType == ConduitItemType.ERC20) {
                        _performERC20Transfer(
                            item.token,
                            msg.sender,
                            recipient,
                            item.amount
                        );
                    } else if (item.itemType == ConduitItemType.ERC721) {
                        _performERC721Transfer(
                            item.token,
                            msg.sender,
                            recipient,
                            item.identifier
                        );
                    } else {
                        _performERC1155Transfer(
                            item.token,
                            msg.sender,
                            recipient,
                            item.identifier,
                            item.amount
                        );
                    }
                }
            }
        }
        // Otherwise, a conduitKey was provided.
        else {
            // Derive the conduit address from the deployer, conduit key
            // and creation code hash.
            address conduit = address(
                uint160(
                    uint256(
                        keccak256(
                            abi.encodePacked(
                                bytes1(0xff),
                                address(_CONDUIT_CONTROLLER),
                                conduitKey,
                                _CONDUIT_CREATION_CODE_HASH
                            )
                        )
                    )
                )
            );

            // Declare a new array to populate with each token transfer.
            ConduitTransfer[] memory conduitTransfers = new ConduitTransfer[](
                totalTransfers
            );

            // Skip overflow checks: all for loops are indexed starting at zero.
            unchecked {
                // Iterate over each transfer.
                for (uint256 i = 0; i < totalTransfers; ++i) {
                    // Retrieve the transfer in question.
                    TransferHelperItem calldata item = items[i];

                    // Create a ConduitTransfer corresponding to each
                    // TransferHelperItem.
                    conduitTransfers[i] = ConduitTransfer(
                        item.itemType,
                        item.token,
                        msg.sender,
                        recipient,
                        item.identifier,
                        item.amount
                    );
                }
            }

            // Call the conduit and execute bulk transfers.
            ConduitInterface(conduit).execute(conduitTransfers);
        }

        // Return a magic value indicating that the transfers were performed.
        magicValue = this.bulkTransfer.selector;
    }
}
