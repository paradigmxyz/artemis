// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Seaport} from "../src/protocols/Seaport/contracts/Seaport.sol";
import {LSSVMPairETH} from "../src/protocols/LSSVMPairFactory/contracts/LSSVMPairETH.sol";
import {BasicOrderParameters} from "../src/protocols/Seaport/contracts/lib/ConsiderationStructs.sol";
import {IERC721} from "../src/protocols/LSSVMPairFactory/contracts/imports/IERC721.sol";
import {Owned} from "solmate/auth/Owned.sol";

contract SudoOpenseaArb is Owned {
    error NoProfit();

    constructor() Owned(msg.sender) {}

    receive() external payable {}

    fallback() external payable {}

    Seaport constant seaport =
        Seaport(0x00000000000000ADc04C56Bf30aC9d3c0aAF14dC);

    function executeArb(
        BasicOrderParameters calldata basicOrder,
        uint256 paymentValue,
        address payable sudo_pool
    ) public {
        uint256 initialBalance = address(this).balance;

        // Buy NFT on OpenSea.
        seaport.fulfillBasicOrder{value: paymentValue}(basicOrder);

        // Set approval for Sudoswap pool.
        IERC721(basicOrder.offerToken).approve(
            sudo_pool,
            basicOrder.offerIdentifier
        );

        // Sell into Sudoswap pool.
        uint256[] memory nftIds = new uint256[](1);

        nftIds[0] = basicOrder.offerIdentifier;

        LSSVMPairETH(sudo_pool).swapNFTsForToken(
            nftIds,
            0, // No need to set min. output since we might revert later on if execution isn't profitable.
            payable(address(this)),
            false,
            address(0)
        );

        // Revert if trade wasn't profitable.
        if (address(this).balance <= initialBalance) revert NoProfit();
    }

    function withdraw() public onlyOwner {
        payable(msg.sender).transfer(address(this).balance);
    }
}
