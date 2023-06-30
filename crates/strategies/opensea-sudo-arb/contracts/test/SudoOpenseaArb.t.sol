// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {Seaport} from "../src/protocols/Seaport/contracts/Seaport.sol";
import {LSSVMPair} from "../src/protocols/LSSVMPairFactory/contracts/LSSVMPair.sol";
import {LSSVMPairETH} from "../src/protocols/LSSVMPairFactory/contracts/LSSVMPairETH.sol";
import {ICurve} from "../src/protocols/LSSVMPairFactory/contracts/bonding-curves/ICurve.sol";
import {CurveErrorCodes} from "../src/protocols/LSSVMPairFactory/contracts/bonding-curves/CurveErrorCodes.sol";
import {LSSVMPairFactory} from "../src/protocols/LSSVMPairFactory/contracts/LSSVMPairFactory.sol";
import {LSSVMPairEnumerableETH} from "../src/protocols/LSSVMPairFactory/contracts/LSSVMPairEnumerableETH.sol";
import {BasicOrderParameters, AdditionalRecipient} from "../src/protocols/Seaport/contracts/lib/ConsiderationStructs.sol";
import {BasicOrderType} from "../src/protocols/Seaport/contracts/lib/ConsiderationEnums.sol";
import {IERC721} from "../src/protocols/LSSVMPairFactory/contracts/imports/IERC721.sol";
import {SudoOpenseaArb} from "../src/SudoOpenseaArb.sol";
import {SudoOpenseaArbCompiled} from "../src/huff/SudoOpenseaArbCompiled.sol";
import {FixedPointMathLib} from "../lib/solmate/src/utils/FixedPointMathLib.sol";

contract SudoOpenseaArbTest is Test {
    error Unauthorized();

    SudoOpenseaArb arb;
    address alice = vm.addr(42);
    address bob = vm.addr(43);

    function setUp() public {
        vm.prank(alice);
        arb = SudoOpenseaArb(SudoOpenseaArbCompiled.deploy());
    }

    function testOwner() public {
        assertEq(arb.owner(), alice);
    }

    function testFuzzWithdraw(uint256 value) public {
        vm.deal(address(arb), value);
        assertEq(address(arb).balance, value);

        vm.prank(alice);
        arb.withdraw();

        assertEq(address(arb).balance, 0);
        assertEq(alice.balance, value);
    }

    function testFuzzWithdrawUnauthorized(address attacker) public {
        vm.assume(attacker != alice);
        vm.deal(address(arb), 1);
        assertEq(address(arb).balance, 1);

        vm.expectRevert(Unauthorized.selector);
        vm.prank(attacker);
        arb.withdraw();

        assertEq(address(arb).balance, 1);
    }

    function testFuzzTransferOwnership(address owner) public {
        assertEq(arb.owner(), alice);

        vm.prank(alice);
        arb.transferOwnership(owner);

        assertEq(arb.owner(), owner);
    }

    function testFuzzTransferOwnershipUnauthorized(address attacker) public {
        vm.assume(attacker != alice);

        assertEq(arb.owner(), alice);

        vm.prank(attacker);
        vm.expectRevert(Unauthorized.selector);
        arb.transferOwnership(attacker);

        assertEq(arb.owner(), alice);
    }
}
