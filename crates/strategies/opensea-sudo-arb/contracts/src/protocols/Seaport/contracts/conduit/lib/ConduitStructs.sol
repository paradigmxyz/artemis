// SPDX-License-Identifier: MIT
pragma solidity >=0.8.7;

import { ConduitItemType } from "./ConduitEnums.sol";

struct ConduitTransfer {
    ConduitItemType itemType;
    address token;
    address from;
    address to;
    uint256 identifier;
    uint256 amount;
}

struct ConduitBatch1155Transfer {
    address token;
    address from;
    address to;
    uint256[] ids;
    uint256[] amounts;
}
