// SPDX-License-Identifier: MIT
pragma solidity >=0.8.7;

// prettier-ignore
import {
    ConduitTransfer,
    ConduitBatch1155Transfer
} from "../conduit/lib/ConduitStructs.sol";

/**
 * @title ConduitInterface
 * @author 0age
 * @notice ConduitInterface contains all external function interfaces, events,
 *         and errors for conduit contracts.
 */
interface ConduitInterface {
    /**
     * @dev Revert with an error when attempting to execute transfers using a
     *      caller that does not have an open channel.
     */
    error ChannelClosed(address channel);

    /**
     * @dev Revert with an error when attempting to update a channel to the
     *      current status of that channel.
     */
    error ChannelStatusAlreadySet(address channel, bool isOpen);

    /**
     * @dev Revert with an error when attempting to execute a transfer for an
     *      item that does not have an ERC20/721/1155 item type.
     */
    error InvalidItemType();

    /**
     * @dev Revert with an error when attempting to update the status of a
     *      channel from a caller that is not the conduit controller.
     */
    error InvalidController();

    /**
     * @dev Emit an event whenever a channel is opened or closed.
     *
     * @param channel The channel that has been updated.
     * @param open    A boolean indicating whether the conduit is open or not.
     */
    event ChannelUpdated(address indexed channel, bool open);

    /**
     * @notice Execute a sequence of ERC20/721/1155 transfers. Only a caller
     *         with an open channel can call this function.
     *
     * @param transfers The ERC20/721/1155 transfers to perform.
     *
     * @return magicValue A magic value indicating that the transfers were
     *                    performed successfully.
     */
    function execute(ConduitTransfer[] calldata transfers)
        external
        returns (bytes4 magicValue);

    /**
     * @notice Execute a sequence of batch 1155 transfers. Only a caller with an
     *         open channel can call this function.
     *
     * @param batch1155Transfers The 1155 batch transfers to perform.
     *
     * @return magicValue A magic value indicating that the transfers were
     *                    performed successfully.
     */
    function executeBatch1155(
        ConduitBatch1155Transfer[] calldata batch1155Transfers
    ) external returns (bytes4 magicValue);

    /**
     * @notice Execute a sequence of transfers, both single and batch 1155. Only
     *         a caller with an open channel can call this function.
     *
     * @param standardTransfers  The ERC20/721/1155 transfers to perform.
     * @param batch1155Transfers The 1155 batch transfers to perform.
     *
     * @return magicValue A magic value indicating that the transfers were
     *                    performed successfully.
     */
    function executeWithBatch1155(
        ConduitTransfer[] calldata standardTransfers,
        ConduitBatch1155Transfer[] calldata batch1155Transfers
    ) external returns (bytes4 magicValue);

    /**
     * @notice Open or close a given channel. Only callable by the controller.
     *
     * @param channel The channel to open or close.
     * @param isOpen  The status of the channel (either open or closed).
     */
    function updateChannel(address channel, bool isOpen) external;
}
