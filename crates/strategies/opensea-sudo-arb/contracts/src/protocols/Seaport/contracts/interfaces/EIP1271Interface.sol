// SPDX-License-Identifier: MIT
pragma solidity >=0.8.7;

interface EIP1271Interface {
    function isValidSignature(bytes32 digest, bytes calldata signature)
        external
        view
        returns (bytes4);
}
