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

contract SudoOpenseaArbForkTest is Test {
    uint256 mainnetFork;
    uint256 localFork;
    LSSVMPairFactory pairFactory = LSSVMPairFactory(payable(0xb16c1342E617A5B6E4b631EB114483FDB289c0A4));
    ICurve curve = ICurve(0x5B6aC51d9B1CeDE0068a1B26533CAce807f883Ee);
    SudoOpenseaArb arb;
    address alice = vm.addr(42);
    address bob = vm.addr(43);

    function setUp() public {
        string memory MAINNET_RPC_URL = vm.envString("ETH_MAINNET_HTTP");
        mainnetFork = vm.createFork(MAINNET_RPC_URL, 16801465); // block where specific order is available
        vm.selectFork(mainnetFork);
        vm.prank(alice);
        arb = SudoOpenseaArb(SudoOpenseaArbCompiled.deploy());
    }

    function testArb() public {
        vm.selectFork(mainnetFork);
        (BasicOrderParameters memory order, uint256 price) = getOrderAndPrice();
        // fund arb contract
        vm.deal(address(arb), price); 
        //set up sudo pool 
        uint128 buyPrice = uint128(order.considerationAmount + 10 ether);
        LSSVMPairETH sudoPool = setupSudoPool(order.offerToken, buyPrice);
        payable(address(sudoPool)).transfer(buyPrice);
        // execute arb 
        uint256 initialBalance = address(arb).balance;
        arb.executeArb(order, price, payable(address(sudoPool)));
        uint256 finalBalance = address(arb).balance;
        assertTrue(finalBalance > initialBalance);
    }

    function testUnprofitableArb() public {
        vm.selectFork(mainnetFork);
        (BasicOrderParameters memory order, uint256 price) = getOrderAndPrice();
        // fund arb contract
        vm.deal(address(arb), price);
        //set up sudo pool
        uint128 buyPrice = uint128(order.considerationAmount - 1);
        LSSVMPairETH sudoPool = setupSudoPool(order.offerToken, buyPrice);
        payable(address(sudoPool)).transfer(buyPrice);
        // execute arb
        uint256 initialBalance = address(arb).balance;
        vm.expectRevert(SudoOpenseaArb.NoProfit.selector);
        arb.executeArb(order, price, payable(address(sudoPool)));
        uint256 finalBalance = address(arb).balance;
        assertTrue(finalBalance == initialBalance);
    }

    // return a basic order we want to fulfill for our arb 
    // this is a real order pulled from the opensea api 
    function getOrderAndPrice() internal pure returns (BasicOrderParameters memory, uint256 price) {
        AdditionalRecipient memory recipient1 = AdditionalRecipient(275000000000000, payable(0x0000a26b00c1F0DF003000390027140000fAa719));
        AdditionalRecipient memory recipient2 = AdditionalRecipient(2750000000000000, payable(0xE7DEbDCfd92a2b6f7AB03B4b3c73494CAdCCB371));
        AdditionalRecipient[] memory recipients = new AdditionalRecipient[](2);
        recipients[0] = recipient1;
        recipients[1] = recipient2;

        BasicOrderParameters memory basicOrder;

        basicOrder.considerationToken = 0x0000000000000000000000000000000000000000;
        basicOrder.considerationIdentifier = 0;
        basicOrder.considerationAmount = 51975000000000000;
        basicOrder.offerer = payable(0x9278bf6A3222d3f9e778De1267fB26D00ee3ae33);
        basicOrder.zone = 0x004C00500000aD104D7DBd00e3ae0A5C00560C00;
        basicOrder.offerToken = 0x50ca8e24D80946B9ccF4A15279DfF9eafde7e240; 
        basicOrder.offerIdentifier = 1222;
        basicOrder.offerAmount = 1; 
        basicOrder.basicOrderType = BasicOrderType.ETH_TO_ERC721_FULL_OPEN;
        basicOrder.startTime = 1678495668;
        basicOrder.endTime = 1681174068; 
        basicOrder.zoneHash = 0x0000000000000000000000000000000000000000000000000000000000000000; 
        basicOrder.salt = 24446860302761739304752683030156737591518664810215442929809313052708787659410; 
        basicOrder.offererConduitKey = 0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000; 
        basicOrder.fulfillerConduitKey = 0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000;
        basicOrder.totalOriginalAdditionalRecipients = 2; 
        basicOrder.additionalRecipients = recipients; 
        basicOrder.signature = hex"9e36fd22058e033e8fca619ab589922817c81188273afdca9a82590edc3862a835fa39ef55742209d2079326a6ba02d2ba7b54e5daf2f8d99bcf7fd35285ec8200000165903770b3fc90e3e7b981e525daa72ad3709f2cf65753a37bb8347e5193c90f"; // 0x244

        return (basicOrder, 55000000000000000);
    }

    // set up pool buying NFTs at price 
    function setupSudoPool(address nft, uint128 price) internal returns (LSSVMPairETH sudoPool) {
        sudoPool = pairFactory.createPairETH(
            IERC721(nft), 
            curve, 
            payable(address(0)),
            LSSVMPair.PoolType.TRADE,
            0,
            0,
            price, 
            new uint256[](0)
        );
    }
}
