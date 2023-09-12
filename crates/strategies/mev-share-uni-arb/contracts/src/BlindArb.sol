// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Owned} from "solmate/auth/Owned.sol";

interface IWETH {
    function deposit() external payable;
    function withdraw(uint256) external;
    function balanceOf(address) external view returns (uint256);
    function transfer(address, uint256) external returns (bool);
}

interface IUniswapV2Pair {
    function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
    function swap(uint256 amount0Out, uint256 amount1Out, address to, bytes calldata data) external;
}

interface IUniswapV3Pool {
    function swap(
        address recipient,
        bool zeroForOne,
        int256 amountSpecified,
        uint160 sqrtPriceLimitX96,
        bytes calldata data
    ) external returns (int256 amount0, int256 amount1);
}

interface IUniswapV3SwapCallback {
    function uniswapV3SwapCallback(
        int256 amount0Delta,
        int256 amount1Delta,
        bytes calldata data
    ) external;
}

contract BlindArb is Owned, IUniswapV3SwapCallback {
    IWETH internal constant WETH = IWETH(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);

    uint160 internal constant MIN_SQRT_RATIO = 4295128739;
    uint160 internal constant MAX_SQRT_RATIO = 1461446703485210103287273052203988822378723970342;

    address callBackAddress = address(0);

    constructor() Owned(msg.sender) {}

    function executeArb__WETH_token0(
        address v2Pair,
        address v3Pair,
        uint256 amountIn,
        uint256 percentageToPayToCoinbase
    ) public onlyOwner {
        callBackAddress = v3Pair;

        uint256 balanceBefore = WETH.balanceOf(address(this));

        // Swap on V3 
        (, int256 tokenOut) = IUniswapV3Pool(v3Pair).swap(
            v2Pair,
            true,
            int256(amountIn),
            MIN_SQRT_RATIO + 1,
            ""
        );

        uint256 tokenOutExact = uint256(-tokenOut);

        IUniswapV2Pair v2Pair = IUniswapV2Pair(v2Pair);
        (uint256 v2Reserve0, uint256 v2Reserve1,) = v2Pair.getReserves();
        uint256 v2AmountOut = getAmountOut(tokenOutExact, v2Reserve1, v2Reserve0);
        v2Pair.swap(v2AmountOut, 0, address(this), "");

        uint256 balanceAfter = WETH.balanceOf(address(this));
        uint profit = balanceAfter - balanceBefore;
        uint profitToCoinbase = profit *  percentageToPayToCoinbase / 100;
        WETH.withdraw(profitToCoinbase);
        block.coinbase.transfer(profitToCoinbase);
        require(balanceAfter - profitToCoinbase > balanceBefore, "arb failed");

        callBackAddress = address(0);
    }

    function executeArb__WETH_token1(
        address v2Pair,
        address v3Pair,
        uint256 amountIn,
        uint256 percentageToPayToCoinbase
    ) public onlyOwner {
        callBackAddress = v3Pair;

        uint256 balanceBefore = WETH.balanceOf(address(this));

        // Swap on V3 
        (int256 tokenOut, ) = IUniswapV3Pool(v3Pair).swap(
            v2Pair,
            false,
            int256(amountIn),
            MAX_SQRT_RATIO - 1,
            ""
        );

        uint256 tokenOutExact = uint256(-tokenOut);

        IUniswapV2Pair v2Pair = IUniswapV2Pair(v2Pair);
        (uint256 v2Reserve0, uint256 v2Reserve1,) = v2Pair.getReserves();
        uint256 v2AmountOut = getAmountOut(tokenOutExact, v2Reserve0, v2Reserve1);
        v2Pair.swap(0, v2AmountOut, address(this), "");

        uint256 balanceAfter = WETH.balanceOf(address(this));
        uint profit = balanceAfter - balanceBefore;
        uint profitToCoinbase = profit *  percentageToPayToCoinbase / 100;
        WETH.withdraw(profitToCoinbase);
        block.coinbase.transfer(profitToCoinbase);
        require(balanceAfter - profitToCoinbase > balanceBefore, "arb failed");

        callBackAddress = address(0);
    }

    /// Pay back WETH
    function uniswapV3SwapCallback(
        int256 amount0Delta,
        int256 amount1Delta,
        bytes calldata _data
    ) external override {
        require(msg.sender == callBackAddress, "invalid sender");
        uint256 amountOwed = uint256(amount0Delta > 0 ? amount0Delta : amount1Delta);
        WETH.transfer(callBackAddress, amountOwed);
    }

    function getAmountOut(uint256 amountIn, uint256 reserveIn, uint256 reserveOut)
        internal
        pure
        returns (uint256 amountOut)
    {
        uint256 amountInWithFee = amountIn * 997;
        uint256 numerator = amountInWithFee * reserveOut;
        uint256 denominator = reserveIn * 1000 + amountInWithFee;
        amountOut = numerator / denominator;
    }

    function withdrawWETHToOwner() external onlyOwner {
        uint256 balance = WETH.balanceOf(address(this));
        WETH.transfer(msg.sender, balance);
    }

    function withdrawETHToOwner() external onlyOwner {
        uint256 balance = address(this).balance;
        payable(msg.sender).transfer(balance);
    }

    receive() external payable {}
}
