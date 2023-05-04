// SPDX-License-Identifier: MIT
pragma solidity >=0.8.7;

import { ZoneInterface } from "../interfaces/ZoneInterface.sol";

// prettier-ignore
import {
    AdvancedOrder,
    CriteriaResolver
} from "../lib/ConsiderationStructs.sol";

contract TestZone is ZoneInterface {
    function isValidOrder(
        bytes32 orderHash,
        address caller,
        address offerer,
        bytes32 zoneHash
    ) external pure override returns (bytes4 validOrderMagicValue) {
        orderHash;
        caller;
        offerer;

        if (zoneHash == bytes32(uint256(1))) {
            revert("Revert on zone hash 1");
        } else if (zoneHash == bytes32(uint256(2))) {
            assembly {
                revert(0, 0)
            }
        }

        validOrderMagicValue = zoneHash != bytes32(uint256(3))
            ? ZoneInterface.isValidOrder.selector
            : bytes4(0xffffffff);
    }

    function isValidOrderIncludingExtraData(
        bytes32 orderHash,
        address caller,
        AdvancedOrder calldata order,
        bytes32[] calldata priorOrderHashes,
        CriteriaResolver[] calldata criteriaResolvers
    ) external pure override returns (bytes4 validOrderMagicValue) {
        orderHash;
        caller;
        order;
        priorOrderHashes;
        criteriaResolvers;

        if (order.extraData.length == 4) {
            revert("Revert on extraData length 4");
        } else if (order.extraData.length == 5) {
            assembly {
                revert(0, 0)
            }
        }

        validOrderMagicValue = order.parameters.zoneHash != bytes32(uint256(3))
            ? ZoneInterface.isValidOrder.selector
            : bytes4(0xffffffff);
    }
}
