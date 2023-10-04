// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import {BlindArb} from "../src/BlindArb.sol";

interface IWETH {
    function deposit() external payable;
    function withdraw(uint256) external;
    function balanceOf(address) external view returns (uint256);
    function transfer(address, uint256) external returns (bool);
}

interface IERC20 {
    function balanceOf(address) external view returns (uint256);
    function approve(address, uint256) external returns (bool);
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

interface ISwapRouter {
    struct ExactInputSingleParams {
        address tokenIn;
        address tokenOut;
        uint24 fee;
        address recipient;
        uint256 deadline;
        uint256 amountIn;
        uint256 amountOutMinimum;
        uint160 sqrtPriceLimitX96;
    }

    function exactInputSingle(ExactInputSingleParams calldata params) external payable returns (uint256 amountOut);
}


contract BlindArbTest is Test {
    using stdStorage for StdStorage;
    // StdStorage stdstore;
    
    uint256 mainnetFork;
    IWETH internal constant WETH = IWETH(0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2);
    IERC20 internal constant SC = IERC20(0xD8912C10681D8B21Fd3742244f44658dBA12264E);
    IUniswapV3Pool internal constant V3 = IUniswapV3Pool(0xe11Ee9c18d03B43d6A7fC53e51AeDda8451e837A);
    IUniswapV2Pair internal constant V2 = IUniswapV2Pair(0x87C9524237a19338be7DbCAc01D6D208fF31136F);
    bool internal constant WETH_TOKEN0 = true;
    ISwapRouter internal constant ROUTER = ISwapRouter(0xE592427A0AEce92De3Edee1F18E0157C05861564);
    uint24 internal constant FEE = 3000;
    uint256 internal constant SWAP_AMOUNT = 5000 ether;
    uint256 internal constant ARB_AMOUNT = 1 ether;

    BlindArb arb;

    function setUp() public {
        string memory MAINNET_RPC_URL = vm.envString("ETH_MAINNET_HTTP");
        mainnetFork = vm.createFork(MAINNET_RPC_URL, 17319257); // block where specific order is available
        vm.selectFork(mainnetFork);
        arb = new BlindArb();
        // WETH.deposit{value: 10 ether}();
        // WETH.transfer(address(arb), 10 ether);
        writeTokenBalance(address(arb), address(WETH), ARB_AMOUNT);
        writeTokenBalance(address(this), address(SC), SWAP_AMOUNT);

    }

    function testArb() public {
        vm.selectFork(mainnetFork);

        console.log("router balance before: %s", SC.balanceOf(address(V3)));
        console.log("ARB BALANCE BEFORE: %s", WETH.balanceOf(address(arb)));

        // approve router to spend 
        SC.approve(address(ROUTER), SWAP_AMOUNT);

        ISwapRouter.ExactInputSingleParams memory params = ISwapRouter.ExactInputSingleParams({
            tokenIn: address(SC),
            tokenOut: address(WETH),
            fee: FEE,
            recipient: address(this),
            deadline: block.timestamp + 1000,
            amountIn: SWAP_AMOUNT,
            amountOutMinimum: 0,
            sqrtPriceLimitX96: 0
        });

        uint256 amount_out = ROUTER.exactInputSingle(params);
        console.log("router balance after: %s", SC.balanceOf(address(V3)));
        console.log("amount out: %s", amount_out);

        arb.executeArb__WETH_token0(address(V2), address(V3), ARB_AMOUNT, 0);
        console.log("ARB BALANCE AFTER: %s", WETH.balanceOf(address(arb)));

    }


   function writeTokenBalance(address who, address token, uint256 amt) internal {
        stdstore
            .target(token)
            .sig(IERC20(token).balanceOf.selector)
            .with_key(who)
            .checked_write(amt);
    }
 
}