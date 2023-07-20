// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Script.sol";
import {SudoOpenseaArbCompiled} from "../src/huff/SudoOpenseaArbCompiled.sol";

contract VerifyHuffScript is Script {
    error CompiledBytecodeDoesNotMatch();

    function run() public {
        string[] memory compileCmd = new string[](3);
        compileCmd[0] = "huffc";
        compileCmd[1] = "--bytecode";
        compileCmd[2] = "./src/huff/SudoOpenseaArb.huff";
        bytes memory compiled = vm.ffi(compileCmd);

        if (keccak256(compiled) != keccak256(SudoOpenseaArbCompiled.initcode)) {
            revert CompiledBytecodeDoesNotMatch();
        }
    }
}
