// SPDX-License-Identifier: MIT
pragma solidity >=0.8.13;

// prettier-ignore
import {
    ConduitControllerInterface
} from "../interfaces/ConduitControllerInterface.sol";

// prettier-ignore
import {
    ConsiderationEventsAndErrors
} from "../interfaces/ConsiderationEventsAndErrors.sol";

import "./ConsiderationConstants.sol";

/**
 * @title ConsiderationBase
 * @author 0age
 * @notice ConsiderationBase contains immutable constants and constructor logic.
 */
contract ConsiderationBase is ConsiderationEventsAndErrors {
    // Precompute hashes, original chainId, and domain separator on deployment.
    bytes32 internal immutable _NAME_HASH;
    bytes32 internal immutable _VERSION_HASH;
    bytes32 internal immutable _EIP_712_DOMAIN_TYPEHASH;
    bytes32 internal immutable _OFFER_ITEM_TYPEHASH;
    bytes32 internal immutable _CONSIDERATION_ITEM_TYPEHASH;
    bytes32 internal immutable _ORDER_TYPEHASH;
    uint256 internal immutable _CHAIN_ID;
    bytes32 internal immutable _DOMAIN_SEPARATOR;

    // Allow for interaction with the conduit controller.
    ConduitControllerInterface internal immutable _CONDUIT_CONTROLLER;

    // Cache the conduit creation code hash used by the conduit controller.
    bytes32 internal immutable _CONDUIT_CREATION_CODE_HASH;

    /**
     * @dev Derive and set hashes, reference chainId, and associated domain
     *      separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) {
        // Derive name and version hashes alongside required EIP-712 typehashes.
        (
            _NAME_HASH,
            _VERSION_HASH,
            _EIP_712_DOMAIN_TYPEHASH,
            _OFFER_ITEM_TYPEHASH,
            _CONSIDERATION_ITEM_TYPEHASH,
            _ORDER_TYPEHASH
        ) = _deriveTypehashes();

        // Store the current chainId and derive the current domain separator.
        _CHAIN_ID = block.chainid;
        _DOMAIN_SEPARATOR = _deriveDomainSeparator();

        // Set the supplied conduit controller.
        _CONDUIT_CONTROLLER = ConduitControllerInterface(conduitController);

        // Retrieve the conduit creation code hash from the supplied controller.
        (_CONDUIT_CREATION_CODE_HASH, ) = (
            _CONDUIT_CONTROLLER.getConduitCodeHashes()
        );
    }

    /**
     * @dev Internal view function to derive the EIP-712 domain separator.
     *
     * @return The derived domain separator.
     */
    function _deriveDomainSeparator() internal view returns (bytes32) {
        // prettier-ignore
        return keccak256(
            abi.encode(
                _EIP_712_DOMAIN_TYPEHASH,
                _NAME_HASH,
                _VERSION_HASH,
                block.chainid,
                address(this)
            )
        );
    }

    /**
     * @dev Internal pure function to retrieve the default name of this
     *      contract and return.
     *
     * @return The name of this contract.
     */
    function _name() internal pure virtual returns (string memory) {
        // Return the name of the contract.
        assembly {
            // First element is the offset for the returned string. Offset the
            // value in memory by one word so that the free memory pointer will
            // be overwritten by the next write.
            mstore(OneWord, OneWord)

            // Name is right padded, so it touches the length which is left
            // padded. This enables writing both values at once. The free memory
            // pointer will be overwritten in the process.
            mstore(NameLengthPtr, NameWithLength)

            // Standard ABI encoding pads returned data to the nearest word. Use
            // the already empty zero slot memory region for this purpose and
            // return the final name string, offset by the original single word.
            return(OneWord, ThreeWords)
        }
    }

    /**
     * @dev Internal pure function to retrieve the default name of this contract
     *      as a string that can be used internally.
     *
     * @return The name of this contract.
     */
    function _nameString() internal pure virtual returns (string memory) {
        // Return the name of the contract.
        return "Consideration";
    }

    /**
     * @dev Internal pure function to derive required EIP-712 typehashes and
     *      other hashes during contract creation.
     *
     * @return nameHash                  The hash of the name of the contract.
     * @return versionHash               The hash of the version string of the
     *                                   contract.
     * @return eip712DomainTypehash      The primary EIP-712 domain typehash.
     * @return offerItemTypehash         The EIP-712 typehash for OfferItem
     *                                   types.
     * @return considerationItemTypehash The EIP-712 typehash for
     *                                   ConsiderationItem types.
     * @return orderTypehash             The EIP-712 typehash for Order types.
     */
    function _deriveTypehashes()
        internal
        pure
        returns (
            bytes32 nameHash,
            bytes32 versionHash,
            bytes32 eip712DomainTypehash,
            bytes32 offerItemTypehash,
            bytes32 considerationItemTypehash,
            bytes32 orderTypehash
        )
    {
        // Derive hash of the name of the contract.
        nameHash = keccak256(bytes(_nameString()));

        // Derive hash of the version string of the contract.
        versionHash = keccak256(bytes("1.1"));

        // Construct the OfferItem type string.
        // prettier-ignore
        bytes memory offerItemTypeString = abi.encodePacked(
            "OfferItem(",
                "uint8 itemType,",
                "address token,",
                "uint256 identifierOrCriteria,",
                "uint256 startAmount,",
                "uint256 endAmount",
            ")"
        );

        // Construct the ConsiderationItem type string.
        // prettier-ignore
        bytes memory considerationItemTypeString = abi.encodePacked(
            "ConsiderationItem(",
                "uint8 itemType,",
                "address token,",
                "uint256 identifierOrCriteria,",
                "uint256 startAmount,",
                "uint256 endAmount,",
                "address recipient",
            ")"
        );

        // Construct the OrderComponents type string, not including the above.
        // prettier-ignore
        bytes memory orderComponentsPartialTypeString = abi.encodePacked(
            "OrderComponents(",
                "address offerer,",
                "address zone,",
                "OfferItem[] offer,",
                "ConsiderationItem[] consideration,",
                "uint8 orderType,",
                "uint256 startTime,",
                "uint256 endTime,",
                "bytes32 zoneHash,",
                "uint256 salt,",
                "bytes32 conduitKey,",
                "uint256 counter",
            ")"
        );

        // Construct the primary EIP-712 domain type string.
        // prettier-ignore
        eip712DomainTypehash = keccak256(
            abi.encodePacked(
                "EIP712Domain(",
                    "string name,",
                    "string version,",
                    "uint256 chainId,",
                    "address verifyingContract",
                ")"
            )
        );

        // Derive the OfferItem type hash using the corresponding type string.
        offerItemTypehash = keccak256(offerItemTypeString);

        // Derive ConsiderationItem type hash using corresponding type string.
        considerationItemTypehash = keccak256(considerationItemTypeString);

        // Derive OrderItem type hash via combination of relevant type strings.
        orderTypehash = keccak256(
            abi.encodePacked(
                orderComponentsPartialTypeString,
                considerationItemTypeString,
                offerItemTypeString
            )
        );
    }
}
