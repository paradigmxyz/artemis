// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

/// @title Sudo Opensea Arbitrage Compiled Library
/// @author jtriley.eth
/// @notice Contains the initcode and a function to deploy it.
library SudoOpenseaArbCompiled {
    /// @dev Thrown when deployment fails.
    error DeployFailed();

    /// @dev Initcode for the contract, no constructor args necessary.
    bytes constant initcode = hex"336000556101ae80600e3d393df360003560e01c8063650dc8e5146100325780633ccfd60b14610126578063f2fde38b1461016757638da5cb5b146101a257005b4763fb0f3ee160005260208052606436038060646040376000600091602401601c6024356d01ad428e4906ae43d8f9852d0dd65af1610076573d600060003e3d6000fd5b63095ea7b360005260443560205261012435604052600060006044601c6000610104355af16100aa573d600060003e3d6000fd5b63b1d3f1c160005260a06020526000604052306060526000608052600060a052600160c0526101243560e0526000600060e4601c60006044355af16100f4573d600060003e3d6000fd5b4711610124577fe39aafee0000000000000000000000000000000000000000000000000000000060005260046000fd5b005b6000543314610159577f82b429000000000000000000000000000000000000000000000000000000000060005260046000fd5b600060006000600047335af1005b600054331461019a577f82b429000000000000000000000000000000000000000000000000000000000060005260046000fd5b600435600055005b60005460005260206000f3";

    /// @notice Deploys the contract and returns the deployment address.
    /// @return deployment The address of the deployed contract.
    /// @dev Throws if deployment fails.
    /// @dev The caller of this internal function becomes the `owner` of the deployment.
    function deploy() internal returns (address payable deployment) {
        bytes memory initcode_ = initcode;
        assembly {
            deployment := create(0, add(initcode_, 0x20), mload(initcode_))
        }
        if (deployment == address(0)) revert DeployFailed();
    }
}

