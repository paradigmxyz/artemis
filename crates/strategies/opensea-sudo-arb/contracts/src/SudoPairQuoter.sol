// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

// import {ERC20} from "solmate/tokens/ERC20.sol";
import {ILSSVMPairFactoryLike} from "./protocols/LSSVMPairFactory/contracts/ILSSVMPairFactoryLike.sol";
import {LSSVMPairETH} from "./protocols/LSSVMPairFactory/contracts/LSSVMPairETH.sol";
import {LSSVMPair} from "./protocols/LSSVMPairFactory/contracts/LSSVMPair.sol";
import {CurveErrorCodes} from "./protocols/LSSVMPairFactory/contracts/bonding-curves/CurveErrorCodes.sol";

/// @title Sudo Pair Quoter
/// @author FrankieIsLost <frankie@paradigm.xyz>
/// @notice A contract to simplify getting sell quotes from sudo pools
contract SudoPairQuoter {

    /// Note that this contract CANNOT set storage, since we are injecting its bytecode into a eth_call code override 

    struct SellQuote {
        bool quoteAvailable;
        address nftAddress;
        uint256 price;
    }

    function getSellQuote(address payable pool_address) public view returns (SellQuote memory sell_quote) {
        ILSSVMPairFactoryLike factory = ILSSVMPairFactoryLike(0xb16c1342E617A5B6E4b631EB114483FDB289c0A4);
        //check that the pool is an ETH pair
        bool isEthPair = factory.isPair(pool_address, ILSSVMPairFactoryLike.PairVariant.ENUMERABLE_ETH)
            || factory.isPair(pool_address, ILSSVMPairFactoryLike.PairVariant.MISSING_ENUMERABLE_ETH);
        if (!isEthPair) return SellQuote(false, address(0), 0);
        //check that you can sell into the pair (i.e. it is a token or trade pool)
        LSSVMPairETH pair = LSSVMPairETH(pool_address);
        bool canSellToPool = pair.poolType() == LSSVMPair.PoolType.TOKEN || pair.poolType() == LSSVMPair.PoolType.TRADE;
        if (!canSellToPool) return SellQuote(false, address(0), 0);
        //get sell quote and make sure pool holds enough ETH to cover it
        (CurveErrorCodes.Error error,,, uint256 outputAmount,) = pair.getSellNFTQuote(1);
        if (error != CurveErrorCodes.Error.OK || outputAmount > address(pair).balance) return SellQuote(false, address(0), 0);
        address nftAddress = address(pair.nft());
        //return valid quote
        return SellQuote(true, nftAddress, outputAmount);
    }

    function getMultipleSellQuotes(address payable[] memory pool_addresses)
        public
        view
        returns (SellQuote[] memory sell_quotes)
    {
        sell_quotes = new SellQuote[](pool_addresses.length);
        for (uint256 i = 0; i < pool_addresses.length; i++) {
            sell_quotes[i] = getSellQuote(pool_addresses[i]);
        }
        return sell_quotes;
        
    }
}
