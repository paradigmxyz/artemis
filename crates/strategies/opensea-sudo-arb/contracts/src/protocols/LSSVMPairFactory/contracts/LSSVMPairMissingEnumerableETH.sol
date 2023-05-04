// SPDX-License-Identifier: AGPL-3.0
pragma solidity ^0.8.0;

import {LSSVMPairETH} from "./LSSVMPairETH.sol";
import {LSSVMPairMissingEnumerable} from "./LSSVMPairMissingEnumerable.sol";
import {ILSSVMPairFactoryLike} from "./ILSSVMPairFactoryLike.sol";

contract LSSVMPairMissingEnumerableETH is
    LSSVMPairMissingEnumerable,
    LSSVMPairETH
{
    function pairVariant()
        public
        pure
        override
        returns (ILSSVMPairFactoryLike.PairVariant)
    {
        return ILSSVMPairFactoryLike.PairVariant.MISSING_ENUMERABLE_ETH;
    }
}
