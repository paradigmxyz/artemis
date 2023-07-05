# Artemis Sudo x Seaport Arbitrage Strategy

This outlines the Huff implementation, ABI specification, and contains a rough solidity reference implementation.

## Gas

| Test                                   | `solc`    | `huffc`   | Savings |
| -------------------------------------- | --------- | --------- | ------- |
| SudoOpenseaArbTest:testArb             | `479_364` | `475_561` | `3_803` |
| SudoOpenseaArbTest:testUnprofitableArb | `479_798` | `475_984` | `3_814` |

## Usage

To reduce the reliance on `huffc`, the contract is precompiled in the [SudoOpenseaArbCompiled](./src/huff/SudoOpenseaArbCompiled.sol) library.

The initialization code may be accessed as a constant directly or deployed from the library.

```solidity
// deploys the contract where the caller is the owner.
address deployment = SudoOpenseaArbCompiled.deploy();

// returns the initialization code.
bytes memory initcode = SudoOpenseaArbCompiled.initcode;
```

## Documentation

Developer documentation is provided for all definitions including description of macro directives and panic cases.

The main contract may be found [here](./src/huff/SudoOpenseaArb.huff).

## Verification

To assert the `huffc` compiler output matches the bytecode, the following script may be run.

> Notice: This requires `ffi` to be enabled to execute. The command may be found [here](./scripts/VerifyHuff.s.sol).

```bash
# runs the script with ffi enabled.
# `huffc` is required for verification.
forge script ./scripts/VerifyHuff.s.sol --ffi
```

## ABI Specification

The Huff contract maintains feature parity with the Solidity implementation.

### `executeArb(BasicOrderParameters calldata, uint256, address)`

Custom data types:

```solidity
struct BasicOrderParameters {
    address considerationToken;                     // 0x24
    uint256 considerationIdentifier;                // 0x44
    uint256 considerationAmount;                    // 0x64
    address payable offerer;                        // 0x84
    address zone;                                   // 0xa4
    address offerToken;                             // 0xc4
    uint256 offerIdentifier;                        // 0xe4
    uint256 offerAmount;                            // 0x104
    BasicOrderType basicOrderType;                  // 0x124
    uint256 startTime;                              // 0x144
    uint256 endTime;                                // 0x164
    bytes32 zoneHash;                               // 0x184
    uint256 salt;                                   // 0x1a4
    bytes32 offererConduitKey;                      // 0x1c4
    bytes32 fulfillerConduitKey;                    // 0x1e4
    uint256 totalOriginalAdditionalRecipients;      // 0x204
    AdditionalRecipient[] additionalRecipients;     // 0x224
    bytes signature;                                // 0x244
    // Total length, excluding dynamic array data: 0x264 (580)
}

enum BasicOrderType {
    // -- snip --
}

struct AdditionalRecipient {
    uint256 amount;
    address payable recipient;
}
```

Calldata layout:

```
bop = basic_order_params

selector
    . bop_ptr
    . payment_value
    . sudo_pool
    . bop_consideration_token
    . bop_consideration_identifier
    . bop_consideration_amount
    . bop_offerer
    . bop_zone
    . bop_offer_token
    . bop_offer_identifier
    . bop_offer_amount
    . bop_start_time
    . bop_end_time
    . bop_zone_hash
    . bop_salt
    . bop_offerer_conduit_key
    . bop_fulfuller_conduit_key
    . bop_total_original_additional_recipients
    . bop_additional_recipients_ptr
    . bop_signatures_ptr
    . bop_additional_recipients_len
    . (bop_additional_recipients_amount . bop_additional_recipients_recipient)+
    . signature
```

| name                                     | size (bytes)  |
| ---------------------------------------- | ------------- |
| selector                                 | 4             |
| bop_ptr                                  | 30            |
| payment_value                            | 30            |
| sudo_pool                                | 30            |
| bop_consideration_token                  | 30            |
| bop_consideration_identifier             | 30            |
| bop_consideration_amount                 | 30            |
| bop_offerer                              | 30            |
| bop_zone                                 | 30            |
| bop_offer_token                          | 30            |
| bop_offer_identifier                     | 30            |
| bop_offer_amount                         | 30            |
| bop_start_time                           | 30            |
| bop_end_time                             | 30            |
| bop_zone_hash                            | 30            |
| bop_salt                                 | 30            |
| bop_offerer_conduit_key                  | 30            |
| bop_fulfuller_conduit_key                | 30            |
| bop_total_original_additional_recipients | 30            |
| bop_additional_recipients_ptr            | 30            |
| bop_signatures_ptr                       | 30            |
| bop_additional_recipients_len            | 30            |
| bop_additional_recipients                | 60 * len      |
| signature_len                            | 30            |
| signature                                | signature_len |

### `withdraw()`

Calldata layout:

```
selector
```

| name     | size (bytes) |
| -------- | ------------ |
| selector | 4            |

### `transferOwnership(address)`

Calldata layout:

```
selector . new_owner
```

| name      | size (bytes) |
| --------- | ------------ |
| selector  | 4            |
| new_owner | 32           |

### `owner() view returns (address)`

Calldata layout:

```
selector
```

| name     | size (bytes) |
| -------- | ------------ |
| selector | 4            |

## Solidity x Yul Reference Implementation

```solidity
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import {Seaport} from "../src/protocols/Seaport/contracts/Seaport.sol";
import {LSSVMPairETH} from "../src/protocols/LSSVMPairFactory/contracts/LSSVMPairETH.sol";
import {BasicOrderParameters} from "../src/protocols/Seaport/contracts/lib/ConsiderationStructs.sol";
import {IERC721} from "../src/protocols/LSSVMPairFactory/contracts/imports/IERC721.sol";
import {Owned} from "solmate/auth/Owned.sol";

contract SudoOpenseaArbFast is Owned {

    // Function and Error Selectors
    uint256 constant fulfillBasicOrderSelector = 0xfb0f3ee1;
    uint256 constant approveSelector = 0x095ea7b3;
    uint256 constant swapNFTsForTokenSelector = 0xb1d3f1c1;
    uint256 constant NoProfitSelector = 0xe39aafee;

    // Memory Pointers
    uint256 constant selectorMemPtr = 0x00;
    uint256 constant argument0MemPtr = 0x20;
    uint256 constant argument1MemPtr = 0x40;
    uint256 constant argument2MemPtr = 0x60;
    uint256 constant argument3MemPtr = 0x80;
    uint256 constant argument4MemPtr = 0xa0;
    uint256 constant argument5MemPtr = 0xc0;
    uint256 constant argument6MemPtr = 0xe0;
    uint256 constant callArumentsMemPtr = 0x1c;
    uint256 constant errorMemPtr = 0x1c;
    uint256 constant nftIdsMemPtr = 0xa0;
    uint256 constant basicOrderParametersMemPtr = 0x20;

    // Calldata Pointers
    uint256 constant basicOrderParametersCdPtr = 0x64;
    uint256 constant offerTokenCdPtr = 0x104;
    uint256 constant offerIdentifierCdPtr = 0x124;
    uint256 constant sudoPoolCdPtr = 0x44;

    // Lengths
    uint256 constant fulfillBasicOrderBaseLen = 0x24;
    uint256 constant approveLen = 0x44;
    uint256 constant swapNFTsForTokenLen = 0xe4;
    uint256 constant wordLen = 0x20;
    uint256 constant nftIdsLen = 0x01;

    // Misc
    uint256 constant zero = 0x00;

    address constant seaport = 0x00000000000001ad428e4906aE43D8F9852d0dD6;

    constructor() Owned(msg.sender) {}

    function executeArb(BasicOrderParameters calldata, uint256 paymentValue, address payable sudo_pool) public {
        assembly {
            let initialBalance := selfbalance()
            let offerIdentifier := calldataload(0x124)
            let basicOrderLen := sub(calldatasize(), 0x64)

            // buy NFT on opensea
            mstore(selectorMemPtr, fulfillBasicOrderSelector)
            mstore(argument0MemPtr, basicOrderParametersMemPtr)
            calldatacopy(argument1MemPtr, basicOrderParametersCdPtr, basicOrderLen)
            call_unwrap(
                gas(),
                seaport,
                paymentValue,
                callArumentsMemPtr,
                add(basicOrderLen, fulfillBasicOrderBaseLen),
                zero,
                zero
            )

            // set approval for sudo pool
            mstore(selectorMemPtr, approveSelector)
            mstore(argument0MemPtr, sudo_pool)
            mstore(argument1MemPtr, offerIdentifier)
            call_unwrap(
                gas(),
                calldataload(offerTokenCdPtr),
                zero,
                callArumentsMemPtr,
                approveLen,
                zero,
                zero
            )

            // sell into pool
            mstore(selectorMemPtr, swapNFTsForTokenSelector)
            mstore(argument0MemPtr, nftIdsMemPtr)
            mstore(argument1MemPtr, zero)
            mstore(argument2MemPtr, address())
            mstore(argument3MemPtr, zero)
            mstore(argument4MemPtr, zero)
            mstore(argument5MemPtr, nftIdsLen)
            mstore(argument6MemPtr, offerIdentifier)
            call_unwrap(
                gas(),
                sudo_pool,
                zero,
                callArumentsMemPtr,
                swapNFTsForTokenLen,
                zero,
                zero
            )

            // revert if no profit
            if iszero(gt(selfbalance(), initialBalance)) {
                mstore(zero, NoProfitSelector)
                revert(errorMemPtr, wordLen)
            }

            function call_unwrap(g, addr, val, argMemPtr, arglen, retMemPtr, retlen) {
                if iszero(call(g, addr, val, argMemPtr, arglen, retMemPtr, retlen)) {
                    returndatacopy(zero, zero, returndatasize())
                    revert(zero, returndatasize())
                }
            }
        }
    }

    function withdraw() public onlyOwner {
        payable(msg.sender).transfer(address(this).balance);
    }

    fallback() external payable {}

    receive() external payable {}
}
```
