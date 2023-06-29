// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Test} from "forge-std/Test.sol";

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
import {FixedPointMathLib} from "../lib/solmate/src/utils/FixedPointMathLib.sol";

contract SudoOpenseaArbTest is Test {
    LSSVMPairFactory pairFactory =
        LSSVMPairFactory(payable(0xb16c1342E617A5B6E4b631EB114483FDB289c0A4));

    ICurve curve = ICurve(0x5B6aC51d9B1CeDE0068a1B26533CAce807f883Ee);

    SudoOpenseaArb arb;

    uint256 mainnetFork;

    function setUp() public {
        mainnetFork = vm.createFork(vm.envString("ETH_MAINNET_HTTP"), 17573260); // Block where specific order is available.

        vm.selectFork(mainnetFork);

        arb = new SudoOpenseaArb();
    }

    function testArb() public {
        (BasicOrderParameters memory order, uint256 price) = getOrderAndPrice();

        // Fund arbitrage contract.
        vm.deal(address(arb), price);

        // Set up Sudoswap pool.
        uint128 buyPrice = uint128(order.considerationAmount + 10 ether);

        LSSVMPairETH sudoPool = setupSudoPool(order.offerToken, buyPrice);

        payable(address(sudoPool)).transfer(buyPrice);

        // Execute arbitrage.

        uint256 initialBalance = address(arb).balance;

        arb.executeArb(order, price, payable(address(sudoPool)));

        uint256 finalBalance = address(arb).balance;

        assertTrue(finalBalance > initialBalance);
    }

    function testUnprofitableArb() public {
        (BasicOrderParameters memory order, uint256 price) = getOrderAndPrice();

        // Fund arbitrage contract.
        vm.deal(address(arb), price);

        // Set up Sudoswap pool.
        uint128 buyPrice = uint128(order.considerationAmount - 1);

        LSSVMPairETH sudoPool = setupSudoPool(order.offerToken, buyPrice);

        payable(address(sudoPool)).transfer(buyPrice);

        // Execute arbitrage.

        uint256 initialBalance = address(arb).balance;

        vm.expectRevert(SudoOpenseaArb.NoProfit.selector);

        arb.executeArb(order, price, payable(address(sudoPool)));

        uint256 finalBalance = address(arb).balance;

        assertTrue(finalBalance == initialBalance);
    }

    /// @notice Get a basic order to fulfill.
    /// @dev Real order pulled from the OpenSea API.
    function getOrderAndPrice()
        internal
        pure
        returns (BasicOrderParameters memory, uint256)
    {
        AdditionalRecipient memory recipient1 = AdditionalRecipient(
            2000000000000000000,
            payable(0x0000a26b00c1F0DF003000390027140000fAa719)
        );

        AdditionalRecipient[] memory recipients = new AdditionalRecipient[](1);

        recipients[0] = recipient1;

        BasicOrderParameters memory basicOrder;
        basicOrder
            .considerationToken = 0x0000000000000000000000000000000000000000;
        basicOrder.considerationIdentifier = 0;
        basicOrder.considerationAmount = 78000000000000000000;
        basicOrder.offerer = payable(
            0x175b0777EaFEcA2b5d8C80d3B262be534cFc6229
        );
        basicOrder.zone = 0x004C00500000aD104D7DBd00e3ae0A5C00560C00;
        basicOrder.offerToken = 0xD3D9ddd0CF0A5F0BFB8f7fcEAe075DF687eAEBaB;
        basicOrder.offerIdentifier = 2145;
        basicOrder.offerAmount = 1;
        basicOrder.basicOrderType = BasicOrderType.ETH_TO_ERC721_FULL_OPEN;
        basicOrder.startTime = 1683315963;
        basicOrder.endTime = 1698867963;
        basicOrder
            .zoneHash = 0x0000000000000000000000000000000000000000000000000000000000000000;
        basicOrder
            .salt = 24446860302761739304752683030156737591518664810215442929812625904300442592992;
        basicOrder
            .offererConduitKey = 0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000;
        basicOrder
            .fulfillerConduitKey = 0x0000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f0000;
        basicOrder.totalOriginalAdditionalRecipients = 1;
        basicOrder.additionalRecipients = recipients;
        basicOrder
            .signature = hex"aa86c8d5c3318e667cc48f8f7b33a2cce147ba90b311126d769306ba85ee3646c534d6fde9a644d750c28615422d706f831e4d6897aeb231418079178dd10125";

        return (basicOrder, 80000000000000000000);
    }

    /// @notice Set up Sudoswap pool to buy NFTs at a certain price.
    /// @param _collection Address of NFT collection.
    /// @param _price Price to buy NFTs at.
    function setupSudoPool(
        address _collection,
        uint128 _price
    ) internal returns (LSSVMPairETH sudoPool) {
        sudoPool = pairFactory.createPairETH(
            IERC721(_collection),
            curve,
            payable(address(0)),
            LSSVMPair.PoolType.TRADE,
            0,
            0,
            _price,
            new uint256[](0)
        );
    }
}
