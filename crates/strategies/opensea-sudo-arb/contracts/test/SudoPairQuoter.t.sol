// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {SudoPairQuoter} from "../src/SudoPairQuoter.sol";

contract SudoPairQuoterTest is Test {
    SudoPairQuoter quoter;
    uint256 mainnetFork;

    function setUp() public {
        string memory MAINNET_RPC_URL = vm.envString("ETH_MAINNET_HTTP");
        mainnetFork = vm.createFork(MAINNET_RPC_URL, 16647294);
        vm.selectFork(mainnetFork);
        quoter = new SudoPairQuoter();
    }

    function testCorrectSingleQuote() public {
        vm.selectFork(mainnetFork);
        SudoPairQuoter.SellQuote memory quote = quoter.getSellQuote(payable(0x1C7064a50Cb4D37A3D21985553CeF80Ee2812937));
        assertTrue(quote.quoteAvailable);
        assertEq(quote.price, 130684210526315699); // verified on mainnet
        assertEq(quote.nftAddress, 0xCa7cA7BcC765F77339bE2d648BA53ce9c8a262bD); // verified on mainnet
    }

    function testCorrectnessMultiQuote() public {
        vm.selectFork(mainnetFork);
        address payable[] memory pools = new address payable[](2);
        pools[0] = payable(0x1C7064a50Cb4D37A3D21985553CeF80Ee2812937); // pool with quote
        pools[1] = payable(0x85B6D83483bb49B2d0A838eFAd5369861fbaF17A); // pool only sells, should not offer quote
        SudoPairQuoter.SellQuote[] memory quote = quoter.getMultipleSellQuotes(pools);
        assertTrue(quote[0].quoteAvailable);
        assertEq(quote[0].price, 130684210526315699); // verified on mainnet
        assertFalse(quote[1].quoteAvailable);
    }
}
