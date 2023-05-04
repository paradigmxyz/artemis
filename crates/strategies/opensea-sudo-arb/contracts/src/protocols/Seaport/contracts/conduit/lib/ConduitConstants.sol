// SPDX-License-Identifier: MIT
pragma solidity >=0.8.7;

// error ChannelClosed(address channel)
uint256 constant ChannelClosed_error_signature = (
    0x93daadf200000000000000000000000000000000000000000000000000000000
);
uint256 constant ChannelClosed_error_ptr = 0x00;
uint256 constant ChannelClosed_channel_ptr = 0x4;
uint256 constant ChannelClosed_error_length = 0x24;

// For the mapping:
// mapping(address => bool) channels
// The position in storage for a particular account is:
// keccak256(abi.encode(account, channels.slot))
uint256 constant ChannelKey_channel_ptr = 0x00;
uint256 constant ChannelKey_slot_ptr = 0x20;
uint256 constant ChannelKey_length = 0x40;
