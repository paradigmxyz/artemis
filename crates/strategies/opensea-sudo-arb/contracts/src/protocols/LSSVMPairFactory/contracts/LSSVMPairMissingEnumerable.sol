// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.0;

import {IERC721} from "./imports/IERC721.sol";
import {EnumerableSet} from "./imports/EnumerableSet.sol";
import {LSSVMPair} from "./LSSVMPair.sol";
import {LSSVMRouter} from "./LSSVMRouter.sol";
import {ILSSVMPairFactoryLike} from "./ILSSVMPairFactoryLike.sol";

/**
    @title An NFT/Token pair for an NFT that does not implement ERC721Enumerable
    @author boredGenius and 0xmons
 */
abstract contract LSSVMPairMissingEnumerable is LSSVMPair {
    using EnumerableSet for EnumerableSet.UintSet;

    // Used for internal ID tracking
    EnumerableSet.UintSet private idSet;

    /// @inheritdoc LSSVMPair
    function _sendAnyNFTsToRecipient(
        IERC721 _nft,
        address nftRecipient,
        uint256 numNFTs
    ) internal override {
        // Send NFTs to recipient
        // We're missing enumerable, so we also update the pair's own ID set
        // NOTE: We start from last index to first index to save on gas
        uint256 lastIndex = idSet.length() - 1;
        for (uint256 i; i < numNFTs; ) {
            uint256 nftId = idSet.at(lastIndex);
            _nft.safeTransferFrom(address(this), nftRecipient, nftId);
            idSet.remove(nftId);

            unchecked {
                --lastIndex;
                ++i;
            }
        }
    }

    /// @inheritdoc LSSVMPair
    function _sendSpecificNFTsToRecipient(
        IERC721 _nft,
        address nftRecipient,
        uint256[] calldata nftIds
    ) internal override {
        // Send NFTs to caller
        // If missing enumerable, update pool's own ID set
        uint256 numNFTs = nftIds.length;
        for (uint256 i; i < numNFTs; ) {
            _nft.safeTransferFrom(address(this), nftRecipient, nftIds[i]);
            // Remove from id set
            idSet.remove(nftIds[i]);

            unchecked {
                ++i;
            }
        }
    }

    /// @inheritdoc LSSVMPair
    function getAllHeldIds() external view override returns (uint256[] memory) {
        uint256 numNFTs = idSet.length();
        uint256[] memory ids = new uint256[](numNFTs);
        for (uint256 i; i < numNFTs; ) {
            ids[i] = idSet.at(i);

            unchecked {
                ++i;
            }
        }
        return ids;
    }

    /**
        @dev When safeTransfering an ERC721 in, we add ID to the idSet
        if it's the same collection used by pool. (As it doesn't auto-track because no ERC721Enumerable)
     */
    function onERC721Received(
        address,
        address,
        uint256 id,
        bytes memory
    ) public virtual returns (bytes4) {
        IERC721 _nft = nft();
        // If it's from the pair's NFT, add the ID to ID set
        if (msg.sender == address(_nft)) {
            idSet.add(id);
        }
        return this.onERC721Received.selector;
    }

    /// @inheritdoc LSSVMPair
    function withdrawERC721(IERC721 a, uint256[] calldata nftIds)
        external
        override
        onlyOwner
    {
        IERC721 _nft = nft();
        uint256 numNFTs = nftIds.length;

        // If it's not the pair's NFT, just withdraw normally
        if (a != _nft) {
            for (uint256 i; i < numNFTs; ) {
                a.safeTransferFrom(address(this), msg.sender, nftIds[i]);

                unchecked {
                    ++i;
                }
            }
        }
        // Otherwise, withdraw and also remove the ID from the ID set
        else {
            for (uint256 i; i < numNFTs; ) {
                _nft.safeTransferFrom(address(this), msg.sender, nftIds[i]);
                idSet.remove(nftIds[i]);

                unchecked {
                    ++i;
                }
            }

            emit NFTWithdrawal();
        }
    }
}
