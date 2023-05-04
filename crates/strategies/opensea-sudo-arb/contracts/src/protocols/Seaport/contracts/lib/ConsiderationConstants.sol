// SPDX-License-Identifier: MIT
pragma solidity >=0.8.7;

/*
 * -------------------------- Disambiguation & Other Notes ---------------------
 *    - The term "head" is used as it is in the documentation for ABI encoding,
 *      but only in reference to dynamic types, i.e. it always refers to the
 *      offset or pointer to the body of a dynamic type. In calldata, the head
 *      is always an offset (relative to the parent object), while in memory,
 *      the head is always the pointer to the body. More information found here:
 *      https://docs.soliditylang.org/en/v0.8.14/abi-spec.html#argument-encoding
 *        - Note that the length of an array is separate from and precedes the
 *          head of the array.
 *
 *    - The term "body" is used in place of the term "head" used in the ABI
 *      documentation. It refers to the start of the data for a dynamic type,
 *      e.g. the first word of a struct or the first word of the first element
 *      in an array.
 *
 *    - The term "pointer" is used to describe the absolute position of a value
 *      and never an offset relative to another value.
 *        - The suffix "_ptr" refers to a memory pointer.
 *        - The suffix "_cdPtr" refers to a calldata pointer.
 *
 *    - The term "offset" is used to describe the position of a value relative
 *      to some parent value. For example, OrderParameters_conduit_offset is the
 *      offset to the "conduit" value in the OrderParameters struct relative to
 *      the start of the body.
 *        - Note: Offsets are used to derive pointers.
 *
 *    - Some structs have pointers defined for all of their fields in this file.
 *      Lines which are commented out are fields that are not used in the
 *      codebase but have been left in for readability.
 */

// Declare constants for name, version, and reentrancy sentinel values.

// Name is right padded, so it touches the length which is left padded. This
// enables writing both values at once. Length goes at byte 95 in memory, and
// name fills bytes 96-109, so both values can be written left-padded to 77.
uint256 constant NameLengthPtr = 77;
uint256 constant NameWithLength = 0x0d436F6E73696465726174696F6E;

uint256 constant Version = 0x312e31;
uint256 constant Version_length = 3;
uint256 constant Version_shift = 0xe8;

uint256 constant _NOT_ENTERED = 1;
uint256 constant _ENTERED = 2;

// Common Offsets
// Offsets for identically positioned fields shared by:
// OfferItem, ConsiderationItem, SpentItem, ReceivedItem

uint256 constant Common_token_offset = 0x20;
uint256 constant Common_identifier_offset = 0x40;
uint256 constant Common_amount_offset = 0x60;

uint256 constant ReceivedItem_size = 0xa0;
uint256 constant ReceivedItem_amount_offset = 0x60;
uint256 constant ReceivedItem_recipient_offset = 0x80;

uint256 constant ReceivedItem_CommonParams_size = 0x60;

uint256 constant ConsiderationItem_recipient_offset = 0xa0;
// Store the same constant in an abbreviated format for a line length fix.
uint256 constant ConsiderItem_recipient_offset = 0xa0;

uint256 constant Execution_offerer_offset = 0x20;
uint256 constant Execution_conduit_offset = 0x40;

uint256 constant InvalidFulfillmentComponentData_error_signature = (
    0x7fda727900000000000000000000000000000000000000000000000000000000
);
uint256 constant InvalidFulfillmentComponentData_error_len = 0x04;

uint256 constant Panic_error_signature = (
    0x4e487b7100000000000000000000000000000000000000000000000000000000
);
uint256 constant Panic_error_offset = 0x04;
uint256 constant Panic_error_length = 0x24;
uint256 constant Panic_arithmetic = 0x11;

uint256 constant MissingItemAmount_error_signature = (
    0x91b3e51400000000000000000000000000000000000000000000000000000000
);
uint256 constant MissingItemAmount_error_len = 0x04;

uint256 constant OrderParameters_offer_head_offset = 0x40;
uint256 constant OrderParameters_consideration_head_offset = 0x60;
uint256 constant OrderParameters_conduit_offset = 0x120;
uint256 constant OrderParameters_counter_offset = 0x140;

uint256 constant Fulfillment_itemIndex_offset = 0x20;

uint256 constant AdvancedOrder_numerator_offset = 0x20;

uint256 constant AlmostOneWord = 0x1f;
uint256 constant OneWord = 0x20;
uint256 constant TwoWords = 0x40;
uint256 constant ThreeWords = 0x60;
uint256 constant FourWords = 0x80;
uint256 constant FiveWords = 0xa0;

uint256 constant FreeMemoryPointerSlot = 0x40;
uint256 constant ZeroSlot = 0x60;
uint256 constant DefaultFreeMemoryPointer = 0x80;

uint256 constant Slot0x80 = 0x80;
uint256 constant Slot0xA0 = 0xa0;

uint256 constant BasicOrder_endAmount_cdPtr = 0x104;
uint256 constant BasicOrder_common_params_size = 0xa0;
uint256 constant BasicOrder_considerationHashesArray_ptr = 0x160;

uint256 constant EIP712_Order_size = 0x180;
uint256 constant EIP712_OfferItem_size = 0xc0;
uint256 constant EIP712_ConsiderationItem_size = 0xe0;
uint256 constant AdditionalRecipients_size = 0x40;

uint256 constant EIP712_DomainSeparator_offset = 0x02;
uint256 constant EIP712_OrderHash_offset = 0x22;
uint256 constant EIP712_DigestPayload_size = 0x42;

uint256 constant receivedItemsHash_ptr = 0x60;

/*
 *  Memory layout in _prepareBasicFulfillmentFromCalldata of
 *  data for OrderFulfilled
 *
 *   event OrderFulfilled(
 *     bytes32 orderHash,
 *     address indexed offerer,
 *     address indexed zone,
 *     address fulfiller,
 *     SpentItem[] offer,
 *       > (itemType, token, id, amount)
 *     ReceivedItem[] consideration
 *       > (itemType, token, id, amount, recipient)
 *   )
 *
 *  - 0x00: orderHash
 *  - 0x20: fulfiller
 *  - 0x40: offer offset (0x80)
 *  - 0x60: consideration offset (0x120)
 *  - 0x80: offer.length (1)
 *  - 0xa0: offerItemType
 *  - 0xc0: offerToken
 *  - 0xe0: offerIdentifier
 *  - 0x100: offerAmount
 *  - 0x120: consideration.length (1 + additionalRecipients.length)
 *  - 0x140: considerationItemType
 *  - 0x160: considerationToken
 *  - 0x180: considerationIdentifier
 *  - 0x1a0: considerationAmount
 *  - 0x1c0: considerationRecipient
 *  - ...
 */

// Minimum length of the OrderFulfilled event data.
// Must be added to the size of the ReceivedItem array for additionalRecipients
// (0xa0 * additionalRecipients.length) to calculate full size of the buffer.
uint256 constant OrderFulfilled_baseSize = 0x1e0;
uint256 constant OrderFulfilled_selector = (
    0x9d9af8e38d66c62e2c12f0225249fd9d721c54b83f48d9352c97c6cacdcb6f31
);

// Minimum offset in memory to OrderFulfilled event data.
// Must be added to the size of the EIP712 hash array for additionalRecipients
// (32 * additionalRecipients.length) to calculate the pointer to event data.
uint256 constant OrderFulfilled_baseOffset = 0x180;
uint256 constant OrderFulfilled_consideration_length_baseOffset = 0x2a0;
uint256 constant OrderFulfilled_offer_length_baseOffset = 0x200;

// uint256 constant OrderFulfilled_orderHash_offset = 0x00;
uint256 constant OrderFulfilled_fulfiller_offset = 0x20;
uint256 constant OrderFulfilled_offer_head_offset = 0x40;
uint256 constant OrderFulfilled_offer_body_offset = 0x80;
uint256 constant OrderFulfilled_consideration_head_offset = 0x60;
uint256 constant OrderFulfilled_consideration_body_offset = 0x120;

// BasicOrderParameters
uint256 constant BasicOrder_parameters_cdPtr = 0x04;
uint256 constant BasicOrder_considerationToken_cdPtr = 0x24;
// uint256 constant BasicOrder_considerationIdentifier_cdPtr = 0x44;
uint256 constant BasicOrder_considerationAmount_cdPtr = 0x64;
uint256 constant BasicOrder_offerer_cdPtr = 0x84;
uint256 constant BasicOrder_zone_cdPtr = 0xa4;
uint256 constant BasicOrder_offerToken_cdPtr = 0xc4;
// uint256 constant BasicOrder_offerIdentifier_cdPtr = 0xe4;
uint256 constant BasicOrder_offerAmount_cdPtr = 0x104;
uint256 constant BasicOrder_basicOrderType_cdPtr = 0x124;
uint256 constant BasicOrder_startTime_cdPtr = 0x144;
// uint256 constant BasicOrder_endTime_cdPtr = 0x164;
// uint256 constant BasicOrder_zoneHash_cdPtr = 0x184;
// uint256 constant BasicOrder_salt_cdPtr = 0x1a4;
uint256 constant BasicOrder_offererConduit_cdPtr = 0x1c4;
uint256 constant BasicOrder_fulfillerConduit_cdPtr = 0x1e4;
uint256 constant BasicOrder_totalOriginalAdditionalRecipients_cdPtr = 0x204;
uint256 constant BasicOrder_additionalRecipients_head_cdPtr = 0x224;
uint256 constant BasicOrder_signature_cdPtr = 0x244;
uint256 constant BasicOrder_additionalRecipients_length_cdPtr = 0x264;
uint256 constant BasicOrder_additionalRecipients_data_cdPtr = 0x284;

uint256 constant BasicOrder_parameters_ptr = 0x20;

uint256 constant BasicOrder_basicOrderType_range = 0x18; // 24 values

/*
 *  Memory layout in _prepareBasicFulfillmentFromCalldata of
 *  EIP712 data for ConsiderationItem
 *   - 0x80: ConsiderationItem EIP-712 typehash (constant)
 *   - 0xa0: itemType
 *   - 0xc0: token
 *   - 0xe0: identifier
 *   - 0x100: startAmount
 *   - 0x120: endAmount
 *   - 0x140: recipient
 */
uint256 constant BasicOrder_considerationItem_typeHash_ptr = 0x80; // memoryPtr
uint256 constant BasicOrder_considerationItem_itemType_ptr = 0xa0;
uint256 constant BasicOrder_considerationItem_token_ptr = 0xc0;
uint256 constant BasicOrder_considerationItem_identifier_ptr = 0xe0;
uint256 constant BasicOrder_considerationItem_startAmount_ptr = 0x100;
uint256 constant BasicOrder_considerationItem_endAmount_ptr = 0x120;
// uint256 constant BasicOrder_considerationItem_recipient_ptr = 0x140;

/*
 *  Memory layout in _prepareBasicFulfillmentFromCalldata of
 *  EIP712 data for OfferItem
 *   - 0x80:  OfferItem EIP-712 typehash (constant)
 *   - 0xa0:  itemType
 *   - 0xc0:  token
 *   - 0xe0:  identifier (reused for offeredItemsHash)
 *   - 0x100: startAmount
 *   - 0x120: endAmount
 */
uint256 constant BasicOrder_offerItem_typeHash_ptr = DefaultFreeMemoryPointer;
uint256 constant BasicOrder_offerItem_itemType_ptr = 0xa0;
uint256 constant BasicOrder_offerItem_token_ptr = 0xc0;
// uint256 constant BasicOrder_offerItem_identifier_ptr = 0xe0;
// uint256 constant BasicOrder_offerItem_startAmount_ptr = 0x100;
uint256 constant BasicOrder_offerItem_endAmount_ptr = 0x120;

/*
 *  Memory layout in _prepareBasicFulfillmentFromCalldata of
 *  EIP712 data for Order
 *   - 0x80:   Order EIP-712 typehash (constant)
 *   - 0xa0:   orderParameters.offerer
 *   - 0xc0:   orderParameters.zone
 *   - 0xe0:   keccak256(abi.encodePacked(offerHashes))
 *   - 0x100:  keccak256(abi.encodePacked(considerationHashes))
 *   - 0x120:  orderType
 *   - 0x140:  startTime
 *   - 0x160:  endTime
 *   - 0x180:  zoneHash
 *   - 0x1a0:  salt
 *   - 0x1c0:  conduit
 *   - 0x1e0:  _counters[orderParameters.offerer] (from storage)
 */
uint256 constant BasicOrder_order_typeHash_ptr = 0x80;
uint256 constant BasicOrder_order_offerer_ptr = 0xa0;
// uint256 constant BasicOrder_order_zone_ptr = 0xc0;
uint256 constant BasicOrder_order_offerHashes_ptr = 0xe0;
uint256 constant BasicOrder_order_considerationHashes_ptr = 0x100;
uint256 constant BasicOrder_order_orderType_ptr = 0x120;
uint256 constant BasicOrder_order_startTime_ptr = 0x140;
// uint256 constant BasicOrder_order_endTime_ptr = 0x160;
// uint256 constant BasicOrder_order_zoneHash_ptr = 0x180;
// uint256 constant BasicOrder_order_salt_ptr = 0x1a0;
// uint256 constant BasicOrder_order_conduitKey_ptr = 0x1c0;
uint256 constant BasicOrder_order_counter_ptr = 0x1e0;
uint256 constant BasicOrder_additionalRecipients_head_ptr = 0x240;
uint256 constant BasicOrder_signature_ptr = 0x260;

// Signature-related
bytes32 constant EIP2098_allButHighestBitMask = (
    0x7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff
);
bytes32 constant ECDSA_twentySeventhAndTwentyEighthBytesSet = (
    0x0000000000000000000000000000000000000000000000000000000101000000
);
uint256 constant ECDSA_MaxLength = 65;
uint256 constant ECDSA_signature_s_offset = 0x40;
uint256 constant ECDSA_signature_v_offset = 0x60;

bytes32 constant EIP1271_isValidSignature_selector = (
    0x1626ba7e00000000000000000000000000000000000000000000000000000000
);
uint256 constant EIP1271_isValidSignature_signatureHead_negativeOffset = 0x20;
uint256 constant EIP1271_isValidSignature_digest_negativeOffset = 0x40;
uint256 constant EIP1271_isValidSignature_selector_negativeOffset = 0x44;
uint256 constant EIP1271_isValidSignature_calldata_baseLength = 0x64;

uint256 constant EIP1271_isValidSignature_signature_head_offset = 0x40;

// abi.encodeWithSignature("NoContract(address)")
uint256 constant NoContract_error_signature = (
    0x5f15d67200000000000000000000000000000000000000000000000000000000
);
uint256 constant NoContract_error_sig_ptr = 0x0;
uint256 constant NoContract_error_token_ptr = 0x4;
uint256 constant NoContract_error_length = 0x24; // 4 + 32 == 36

uint256 constant EIP_712_PREFIX = (
    0x1901000000000000000000000000000000000000000000000000000000000000
);

uint256 constant ExtraGasBuffer = 0x20;
uint256 constant CostPerWord = 3;
uint256 constant MemoryExpansionCoefficient = 0x200; // 512

uint256 constant Create2AddressDerivation_ptr = 0x0b;
uint256 constant Create2AddressDerivation_length = 0x55;

uint256 constant MaskOverByteTwelve = (
    0x0000000000000000000000ff0000000000000000000000000000000000000000
);

uint256 constant MaskOverLastTwentyBytes = (
    0x000000000000000000000000ffffffffffffffffffffffffffffffffffffffff
);

uint256 constant MaskOverFirstFourBytes = (
    0xffffffff00000000000000000000000000000000000000000000000000000000
);

uint256 constant Conduit_execute_signature = (
    0x4ce34aa200000000000000000000000000000000000000000000000000000000
);

uint256 constant MaxUint8 = 0xff;
uint256 constant MaxUint120 = 0xffffffffffffffffffffffffffffff;

uint256 constant Conduit_execute_ConduitTransfer_ptr = 0x20;
uint256 constant Conduit_execute_ConduitTransfer_length = 0x01;

uint256 constant Conduit_execute_ConduitTransfer_offset_ptr = 0x04;
uint256 constant Conduit_execute_ConduitTransfer_length_ptr = 0x24;
uint256 constant Conduit_execute_transferItemType_ptr = 0x44;
uint256 constant Conduit_execute_transferToken_ptr = 0x64;
uint256 constant Conduit_execute_transferFrom_ptr = 0x84;
uint256 constant Conduit_execute_transferTo_ptr = 0xa4;
uint256 constant Conduit_execute_transferIdentifier_ptr = 0xc4;
uint256 constant Conduit_execute_transferAmount_ptr = 0xe4;

uint256 constant OneConduitExecute_size = 0x104;

// Sentinel value to indicate that the conduit accumulator is not armed.
uint256 constant AccumulatorDisarmed = 0x20;
uint256 constant AccumulatorArmed = 0x40;
uint256 constant Accumulator_conduitKey_ptr = 0x20;
uint256 constant Accumulator_selector_ptr = 0x40;
uint256 constant Accumulator_array_offset_ptr = 0x44;
uint256 constant Accumulator_array_length_ptr = 0x64;

uint256 constant Accumulator_itemSizeOffsetDifference = 0x3c;

uint256 constant Accumulator_array_offset = 0x20;
uint256 constant Conduit_transferItem_size = 0xc0;
uint256 constant Conduit_transferItem_token_ptr = 0x20;
uint256 constant Conduit_transferItem_from_ptr = 0x40;
uint256 constant Conduit_transferItem_to_ptr = 0x60;
uint256 constant Conduit_transferItem_identifier_ptr = 0x80;
uint256 constant Conduit_transferItem_amount_ptr = 0xa0;

// Declare constant for errors related to amount derivation.
// error InexactFraction() @ AmountDerivationErrors.sol
uint256 constant InexactFraction_error_signature = (
    0xc63cf08900000000000000000000000000000000000000000000000000000000
);
uint256 constant InexactFraction_error_len = 0x04;

// Declare constant for errors related to signature verification.
uint256 constant Ecrecover_precompile = 1;
uint256 constant Ecrecover_args_size = 0x80;
uint256 constant Signature_lower_v = 27;

// error BadSignatureV(uint8) @ SignatureVerificationErrors.sol
uint256 constant BadSignatureV_error_signature = (
    0x1f003d0a00000000000000000000000000000000000000000000000000000000
);
uint256 constant BadSignatureV_error_offset = 0x04;
uint256 constant BadSignatureV_error_length = 0x24;

// error InvalidSigner() @ SignatureVerificationErrors.sol
uint256 constant InvalidSigner_error_signature = (
    0x815e1d6400000000000000000000000000000000000000000000000000000000
);
uint256 constant InvalidSigner_error_length = 0x04;

// error InvalidSignature() @ SignatureVerificationErrors.sol
uint256 constant InvalidSignature_error_signature = (
    0x8baa579f00000000000000000000000000000000000000000000000000000000
);
uint256 constant InvalidSignature_error_length = 0x04;

// error BadContractSignature() @ SignatureVerificationErrors.sol
uint256 constant BadContractSignature_error_signature = (
    0x4f7fb80d00000000000000000000000000000000000000000000000000000000
);
uint256 constant BadContractSignature_error_length = 0x04;

uint256 constant NumBitsAfterSelector = 0xe0;

// 69 is the lowest modulus for which the remainder
// of every selector other than the two match functions
// is greater than those of the match functions.
uint256 constant NonMatchSelector_MagicModulus = 69;
// Of the two match function selectors, the highest
// remainder modulo 69 is 29.
uint256 constant NonMatchSelector_MagicRemainder = 0x1d;
