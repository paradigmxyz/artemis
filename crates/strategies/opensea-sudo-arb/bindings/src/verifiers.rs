pub use verifiers::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod verifiers {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduitController\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"constructor\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BadContractSignature\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"BadFraction\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"BadReturnValueFromERC20OnTransfer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]}],\"type\":\"error\",\"name\":\"BadSignatureV\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"considerationIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"shortfallAmount\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ConsiderationNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"identifiers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}],\"type\":\"error\",\"name\":\"ERC1155BatchTransferGenericFailure\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"EtherTransferGenericFailure\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InsufficientEtherSupplied\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Invalid1155BatchTransferEncoding\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidBasicOrderParameterEncoding\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidCallToConduit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidCanceller\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidConduit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidERC721TransferAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidMsgValue\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidNativeOfferItem\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSignature\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSigner\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidTime\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MissingItemAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MissingOriginalConsiderationItems\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"NoContract\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NoReentrantCalls\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NoSpecifiedOrdersAvailable\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"OrderAlreadyFilled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"OrderIsCancelled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"OrderPartiallyFilled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PartialFillsNotEnabledForOrder\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"TokenTransferGenericFailure\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"UnusedItemParameters\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newCounter\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CounterIncremented\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OrderCancelled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"struct SpentItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"indexed\":false},{\"internalType\":\"struct ReceivedItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"OrderFulfilled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OrderValidated\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static VERIFIERS_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        97,
        1,
        192,
        96,
        64,
        82,
        52,
        128,
        21,
        97,
        0,
        17,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        64,
        81,
        97,
        6,
        244,
        56,
        3,
        128,
        97,
        6,
        244,
        131,
        57,
        129,
        1,
        96,
        64,
        129,
        144,
        82,
        97,
        0,
        48,
        145,
        97,
        5,
        175,
        86,
        91,
        128,
        128,
        128,
        97,
        0,
        59,
        97,
        1,
        25,
        86,
        91,
        97,
        1,
        32,
        82,
        97,
        1,
        0,
        82,
        96,
        224,
        82,
        96,
        192,
        129,
        129,
        82,
        96,
        160,
        131,
        129,
        82,
        96,
        128,
        133,
        129,
        82,
        70,
        97,
        1,
        64,
        129,
        144,
        82,
        96,
        64,
        128,
        81,
        96,
        32,
        129,
        129,
        1,
        151,
        144,
        151,
        82,
        128,
        130,
        1,
        152,
        144,
        152,
        82,
        96,
        96,
        136,
        1,
        150,
        144,
        150,
        82,
        144,
        134,
        1,
        82,
        48,
        133,
        130,
        1,
        82,
        131,
        81,
        128,
        134,
        3,
        144,
        145,
        1,
        129,
        82,
        147,
        1,
        144,
        145,
        82,
        129,
        81,
        145,
        1,
        32,
        97,
        1,
        96,
        82,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        129,
        22,
        97,
        1,
        128,
        129,
        144,
        82,
        96,
        64,
        128,
        81,
        99,
        10,
        150,
        173,
        57,
        96,
        224,
        27,
        129,
        82,
        129,
        81,
        99,
        10,
        150,
        173,
        57,
        146,
        96,
        4,
        128,
        132,
        1,
        147,
        145,
        146,
        145,
        130,
        144,
        3,
        1,
        129,
        134,
        90,
        250,
        21,
        128,
        21,
        97,
        0,
        225,
        87,
        61,
        96,
        0,
        128,
        62,
        61,
        96,
        0,
        253,
        91,
        80,
        80,
        80,
        80,
        96,
        64,
        81,
        61,
        96,
        31,
        25,
        96,
        31,
        130,
        1,
        22,
        130,
        1,
        128,
        96,
        64,
        82,
        80,
        129,
        1,
        144,
        97,
        1,
        5,
        145,
        144,
        97,
        5,
        223,
        86,
        91,
        80,
        97,
        1,
        160,
        82,
        80,
        80,
        96,
        1,
        96,
        0,
        85,
        80,
        97,
        6,
        90,
        144,
        80,
        86,
        91,
        96,
        0,
        128,
        128,
        128,
        128,
        128,
        97,
        1,
        74,
        96,
        64,
        128,
        81,
        128,
        130,
        1,
        144,
        145,
        82,
        96,
        13,
        129,
        82,
        108,
        33,
        183,
        183,
        57,
        180,
        178,
        50,
        185,
        48,
        186,
        52,
        183,
        183,
        96,
        153,
        27,
        96,
        32,
        130,
        1,
        82,
        144,
        86,
        91,
        128,
        81,
        96,
        32,
        145,
        130,
        1,
        32,
        96,
        64,
        128,
        81,
        128,
        130,
        1,
        130,
        82,
        96,
        3,
        129,
        82,
        98,
        49,
        46,
        49,
        96,
        232,
        27,
        144,
        132,
        1,
        82,
        81,
        144,
        151,
        80,
        127,
        114,
        44,
        14,
        12,
        128,
        72,
        114,
        102,
        232,
        198,
        164,
        94,
        58,
        26,
        128,
        58,
        171,
        35,
        55,
        138,
        156,
        50,
        230,
        235,
        224,
        41,
        212,
        250,
        215,
        191,
        201,
        101,
        150,
        80,
        96,
        0,
        145,
        97,
        2,
        78,
        145,
        1,
        105,
        9,
        236,
        204,
        204,
        174,
        73,
        46,
        140,
        173,
        165,
        96,
        179,
        27,
        129,
        82,
        110,
        29,
        90,
        91,
        157,
        14,
        8,
        26,
        93,
        25,
        91,
        85,
        30,
        92,
        25,
        75,
        96,
        138,
        27,
        96,
        10,
        130,
        1,
        82,
        109,
        24,
        89,
        25,
        28,
        153,
        92,
        220,
        200,
        29,
        27,
        218,
        217,
        91,
        139,
        96,
        146,
        27,
        96,
        25,
        130,
        1,
        82,
        127,
        117,
        105,
        110,
        116,
        50,
        53,
        54,
        32,
        105,
        100,
        101,
        110,
        116,
        105,
        102,
        105,
        101,
        114,
        79,
        114,
        67,
        114,
        105,
        116,
        101,
        114,
        105,
        97,
        44,
        0,
        0,
        0,
        96,
        39,
        130,
        1,
        82,
        127,
        117,
        105,
        110,
        116,
        50,
        53,
        54,
        32,
        115,
        116,
        97,
        114,
        116,
        65,
        109,
        111,
        117,
        110,
        116,
        44,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        68,
        130,
        1,
        82,
        112,
        29,
        90,
        91,
        157,
        12,
        141,
        77,
        136,
        25,
        91,
        153,
        16,
        91,
        91,
        221,
        91,
        157,
        96,
        122,
        27,
        96,
        88,
        130,
        1,
        82,
        96,
        41,
        96,
        248,
        27,
        96,
        105,
        130,
        1,
        82,
        96,
        106,
        1,
        144,
        86,
        91,
        96,
        64,
        128,
        81,
        96,
        31,
        25,
        129,
        132,
        3,
        1,
        129,
        82,
        130,
        130,
        82,
        113,
        8,
        109,
        237,
        206,
        109,
        44,
        140,
        174,
        76,
        46,
        141,
        45,
        237,
        201,
        46,
        140,
        173,
        165,
        96,
        115,
        27,
        96,
        32,
        132,
        1,
        82,
        110,
        29,
        90,
        91,
        157,
        14,
        8,
        26,
        93,
        25,
        91,
        85,
        30,
        92,
        25,
        75,
        96,
        138,
        27,
        96,
        50,
        132,
        1,
        82,
        109,
        24,
        89,
        25,
        28,
        153,
        92,
        220,
        200,
        29,
        27,
        218,
        217,
        91,
        139,
        96,
        146,
        27,
        96,
        65,
        132,
        1,
        82,
        127,
        117,
        105,
        110,
        116,
        50,
        53,
        54,
        32,
        105,
        100,
        101,
        110,
        116,
        105,
        102,
        105,
        101,
        114,
        79,
        114,
        67,
        114,
        105,
        116,
        101,
        114,
        105,
        97,
        44,
        0,
        0,
        0,
        96,
        79,
        132,
        1,
        82,
        127,
        117,
        105,
        110,
        116,
        50,
        53,
        54,
        32,
        115,
        116,
        97,
        114,
        116,
        65,
        109,
        111,
        117,
        110,
        116,
        44,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        108,
        132,
        1,
        82,
        113,
        29,
        90,
        91,
        157,
        12,
        141,
        77,
        136,
        25,
        91,
        153,
        16,
        91,
        91,
        221,
        91,
        157,
        11,
        96,
        114,
        27,
        96,
        128,
        132,
        1,
        82,
        112,
        24,
        89,
        25,
        28,
        153,
        92,
        220,
        200,
        28,
        153,
        88,
        218,
        92,
        26,
        89,
        91,
        157,
        96,
        122,
        27,
        96,
        146,
        132,
        1,
        82,
        96,
        41,
        96,
        248,
        27,
        96,
        163,
        132,
        1,
        129,
        144,
        82,
        130,
        81,
        128,
        133,
        3,
        96,
        132,
        1,
        129,
        82,
        96,
        164,
        133,
        1,
        144,
        147,
        82,
        111,
        9,
        238,
        76,
        140,
        174,
        72,
        109,
        237,
        174,
        13,
        237,
        204,
        173,
        206,
        142,
        101,
        96,
        131,
        27,
        96,
        196,
        133,
        1,
        82,
        111,
        24,
        89,
        25,
        28,
        153,
        92,
        220,
        200,
        27,
        217,
        153,
        153,
        92,
        153,
        92,
        139,
        96,
        130,
        27,
        96,
        212,
        133,
        1,
        82,
        108,
        24,
        89,
        25,
        28,
        153,
        92,
        220,
        200,
        30,
        155,
        219,
        153,
        75,
        96,
        154,
        27,
        96,
        228,
        133,
        1,
        82,
        113,
        19,
        217,
        153,
        153,
        92,
        146,
        93,
        25,
        91,
        86,
        215,
        72,
        27,
        217,
        153,
        153,
        92,
        139,
        96,
        114,
        27,
        96,
        241,
        133,
        1,
        82,
        127,
        67,
        111,
        110,
        115,
        105,
        100,
        101,
        114,
        97,
        116,
        105,
        111,
        110,
        73,
        116,
        101,
        109,
        91,
        93,
        32,
        99,
        111,
        110,
        115,
        105,
        100,
        101,
        114,
        97,
        116,
        105,
        111,
        97,
        1,
        3,
        133,
        1,
        82,
        97,
        27,
        139,
        96,
        242,
        27,
        97,
        1,
        35,
        133,
        1,
        82,
        111,
        29,
        90,
        91,
        157,
        14,
        8,
        27,
        220,
        153,
        25,
        92,
        149,
        30,
        92,
        25,
        75,
        96,
        130,
        27,
        97,
        1,
        37,
        133,
        1,
        82,
        113,
        29,
        90,
        91,
        157,
        12,
        141,
        77,
        136,
        28,
        221,
        24,
        92,
        157,
        21,
        26,
        91,
        89,
        75,
        96,
        114,
        27,
        97,
        1,
        53,
        133,
        1,
        82,
        111,
        29,
        90,
        91,
        157,
        12,
        141,
        77,
        136,
        25,
        91,
        153,
        21,
        26,
        91,
        89,
        75,
        96,
        130,
        27,
        97,
        1,
        71,
        133,
        1,
        82,
        112,
        24,
        158,
        93,
        25,
        92,
        204,
        204,
        136,
        30,
        155,
        219,
        153,
        82,
        24,
        92,
        218,
        11,
        96,
        122,
        27,
        97,
        1,
        87,
        133,
        1,
        82,
        108,
        29,
        90,
        91,
        157,
        12,
        141,
        77,
        136,
        28,
        216,
        91,
        29,
        11,
        96,
        154,
        27,
        97,
        1,
        104,
        133,
        1,
        82,
        127,
        98,
        121,
        116,
        101,
        115,
        51,
        50,
        32,
        99,
        111,
        110,
        100,
        117,
        105,
        116,
        75,
        101,
        121,
        44,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        97,
        1,
        117,
        133,
        1,
        82,
        110,
        58,
        180,
        183,
        58,
        25,
        26,
        155,
        16,
        49,
        183,
        186,
        183,
        58,
        50,
        185,
        96,
        137,
        27,
        97,
        1,
        136,
        133,
        1,
        82,
        97,
        1,
        151,
        132,
        1,
        82,
        146,
        80,
        144,
        96,
        0,
        144,
        97,
        1,
        152,
        1,
        96,
        64,
        128,
        81,
        96,
        31,
        25,
        129,
        132,
        3,
        1,
        129,
        82,
        144,
        130,
        144,
        82,
        108,
        8,
        169,
        42,
        6,
        230,
        38,
        72,
        141,
        237,
        172,
        45,
        45,
        197,
        96,
        155,
        27,
        96,
        32,
        131,
        1,
        82,
        107,
        28,
        221,
        28,
        154,
        91,
        153,
        200,
        27,
        152,
        91,
        89,
        75,
        96,
        162,
        27,
        96,
        45,
        131,
        1,
        82,
        110,
        28,
        221,
        28,
        154,
        91,
        153,
        200,
        29,
        153,
        92,
        156,
        218,
        91,
        219,
        139,
        96,
        138,
        27,
        96,
        57,
        131,
        1,
        82,
        111,
        29,
        90,
        91,
        157,
        12,
        141,
        77,
        136,
        24,
        218,
        24,
        90,
        91,
        146,
        89,
        11,
        96,
        130,
        27,
        96,
        72,
        131,
        1,
        82,
        127,
        97,
        100,
        100,
        114,
        101,
        115,
        115,
        32,
        118,
        101,
        114,
        105,
        102,
        121,
        105,
        110,
        103,
        67,
        111,
        110,
        116,
        114,
        97,
        99,
        116,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        96,
        88,
        131,
        1,
        82,
        96,
        41,
        96,
        248,
        27,
        96,
        113,
        131,
        1,
        82,
        145,
        80,
        96,
        114,
        1,
        96,
        64,
        128,
        81,
        96,
        31,
        25,
        129,
        132,
        3,
        1,
        129,
        82,
        144,
        130,
        144,
        82,
        128,
        81,
        96,
        32,
        145,
        130,
        1,
        32,
        133,
        81,
        134,
        131,
        1,
        32,
        133,
        81,
        134,
        132,
        1,
        32,
        145,
        154,
        80,
        152,
        80,
        150,
        80,
        97,
        5,
        140,
        145,
        131,
        145,
        133,
        145,
        135,
        145,
        1,
        97,
        6,
        51,
        86,
        91,
        96,
        64,
        81,
        96,
        32,
        129,
        131,
        3,
        3,
        129,
        82,
        144,
        96,
        64,
        82,
        128,
        81,
        144,
        96,
        32,
        1,
        32,
        147,
        80,
        80,
        80,
        80,
        144,
        145,
        146,
        147,
        148,
        149,
        86,
        91,
        96,
        0,
        96,
        32,
        130,
        132,
        3,
        18,
        21,
        97,
        5,
        193,
        87,
        96,
        0,
        128,
        253,
        91,
        129,
        81,
        96,
        1,
        96,
        1,
        96,
        160,
        27,
        3,
        129,
        22,
        129,
        20,
        97,
        5,
        216,
        87,
        96,
        0,
        128,
        253,
        91,
        147,
        146,
        80,
        80,
        80,
        86,
        91,
        96,
        0,
        128,
        96,
        64,
        131,
        133,
        3,
        18,
        21,
        97,
        5,
        242,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        80,
        128,
        81,
        96,
        32,
        144,
        145,
        1,
        81,
        144,
        146,
        144,
        145,
        80,
        86,
        91,
        96,
        0,
        129,
        81,
        96,
        0,
        91,
        129,
        129,
        16,
        21,
        97,
        6,
        36,
        87,
        96,
        32,
        129,
        133,
        1,
        129,
        1,
        81,
        134,
        131,
        1,
        82,
        1,
        97,
        6,
        10,
        86,
        91,
        80,
        96,
        0,
        147,
        1,
        146,
        131,
        82,
        80,
        144,
        145,
        144,
        80,
        86,
        91,
        96,
        0,
        97,
        6,
        81,
        97,
        6,
        75,
        97,
        6,
        69,
        132,
        136,
        97,
        6,
        3,
        86,
        91,
        134,
        97,
        6,
        3,
        86,
        91,
        132,
        97,
        6,
        3,
        86,
        91,
        149,
        148,
        80,
        80,
        80,
        80,
        80,
        86,
        91,
        96,
        128,
        81,
        96,
        160,
        81,
        96,
        192,
        81,
        96,
        224,
        81,
        97,
        1,
        0,
        81,
        97,
        1,
        32,
        81,
        97,
        1,
        64,
        81,
        97,
        1,
        96,
        81,
        97,
        1,
        128,
        81,
        97,
        1,
        160,
        81,
        96,
        63,
        97,
        6,
        181,
        96,
        0,
        57,
        96,
        0,
        80,
        80,
        96,
        0,
        80,
        80,
        96,
        0,
        80,
        80,
        96,
        0,
        80,
        80,
        96,
        0,
        80,
        80,
        96,
        0,
        80,
        80,
        96,
        0,
        80,
        80,
        96,
        0,
        80,
        80,
        96,
        0,
        80,
        80,
        96,
        0,
        80,
        80,
        96,
        63,
        96,
        0,
        243,
        254,
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
        253,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        145,
        195,
        162,
        81,
        99,
        49,
        60,
        196,
        57,
        126,
        36,
        216,
        86,
        60,
        80,
        63,
        128,
        141,
        238,
        156,
        136,
        121,
        78,
        66,
        186,
        15,
        39,
        49,
        216,
        150,
        13,
        160,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The bytecode of the contract.
    pub static VERIFIERS_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __BYTECODE,
    );
    #[rustfmt::skip]
    const __DEPLOYED_BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        96,
        0,
        128,
        253,
        254,
        162,
        100,
        105,
        112,
        102,
        115,
        88,
        34,
        18,
        32,
        145,
        195,
        162,
        81,
        99,
        49,
        60,
        196,
        57,
        126,
        36,
        216,
        86,
        60,
        80,
        63,
        128,
        141,
        238,
        156,
        136,
        121,
        78,
        66,
        186,
        15,
        39,
        49,
        216,
        150,
        13,
        160,
        100,
        115,
        111,
        108,
        99,
        67,
        0,
        8,
        19,
        0,
        51,
    ];
    ///The deployed bytecode of the contract.
    pub static VERIFIERS_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct Verifiers<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for Verifiers<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for Verifiers<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for Verifiers<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for Verifiers<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(Verifiers)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> Verifiers<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    VERIFIERS_ABI.clone(),
                    client,
                ),
            )
        }
        /// Constructs the general purpose `Deployer` instance based on the provided constructor arguments and sends it.
        /// Returns a new instance of a deployer that returns an instance of this contract after sending the transaction
        ///
        /// Notes:
        /// - If there are no constructor arguments, you should pass `()` as the argument.
        /// - The default poll duration is 7 seconds.
        /// - The default number of confirmations is 1 block.
        ///
        ///
        /// # Example
        ///
        /// Generate contract bindings with `abigen!` and deploy a new contract instance.
        ///
        /// *Note*: this requires a `bytecode` and `abi` object in the `greeter.json` artifact.
        ///
        /// ```ignore
        /// # async fn deploy<M: ethers::providers::Middleware>(client: ::std::sync::Arc<M>) {
        ///     abigen!(Greeter, "../greeter.json");
        ///
        ///    let greeter_contract = Greeter::deploy(client, "Hello world!".to_string()).unwrap().send().await.unwrap();
        ///    let msg = greeter_contract.greet().call().await.unwrap();
        /// # }
        /// ```
        pub fn deploy<T: ::ethers::core::abi::Tokenize>(
            client: ::std::sync::Arc<M>,
            constructor_args: T,
        ) -> ::core::result::Result<
            ::ethers::contract::builders::ContractDeployer<M, Self>,
            ::ethers::contract::ContractError<M>,
        > {
            let factory = ::ethers::contract::ContractFactory::new(
                VERIFIERS_ABI.clone(),
                VERIFIERS_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
        ///Gets the contract's `CounterIncremented` event
        pub fn counter_incremented_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            CounterIncrementedFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OrderCancelled` event
        pub fn order_cancelled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OrderCancelledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OrderFulfilled` event
        pub fn order_fulfilled_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OrderFulfilledFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OrderValidated` event
        pub fn order_validated_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OrderValidatedFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            VerifiersEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for Verifiers<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BadContractSignature` with signature `BadContractSignature()` and selector `0x4f7fb80d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "BadContractSignature", abi = "BadContractSignature()")]
    pub struct BadContractSignature;
    ///Custom Error type `BadFraction` with signature `BadFraction()` and selector `0x5a052b32`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "BadFraction", abi = "BadFraction()")]
    pub struct BadFraction;
    ///Custom Error type `BadReturnValueFromERC20OnTransfer` with signature `BadReturnValueFromERC20OnTransfer(address,address,address,uint256)` and selector `0x98891923`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "BadReturnValueFromERC20OnTransfer",
        abi = "BadReturnValueFromERC20OnTransfer(address,address,address,uint256)"
    )]
    pub struct BadReturnValueFromERC20OnTransfer {
        pub token: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `BadSignatureV` with signature `BadSignatureV(uint8)` and selector `0x1f003d0a`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "BadSignatureV", abi = "BadSignatureV(uint8)")]
    pub struct BadSignatureV {
        pub v: u8,
    }
    ///Custom Error type `ConsiderationNotMet` with signature `ConsiderationNotMet(uint256,uint256,uint256)` and selector `0xa5f54208`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ConsiderationNotMet",
        abi = "ConsiderationNotMet(uint256,uint256,uint256)"
    )]
    pub struct ConsiderationNotMet {
        pub order_index: ::ethers::core::types::U256,
        pub consideration_index: ::ethers::core::types::U256,
        pub shortfall_amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `ERC1155BatchTransferGenericFailure` with signature `ERC1155BatchTransferGenericFailure(address,address,address,uint256[],uint256[])` and selector `0xafc445e2`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "ERC1155BatchTransferGenericFailure",
        abi = "ERC1155BatchTransferGenericFailure(address,address,address,uint256[],uint256[])"
    )]
    pub struct ERC1155BatchTransferGenericFailure {
        pub token: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub identifiers: ::std::vec::Vec<::ethers::core::types::U256>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Custom Error type `EtherTransferGenericFailure` with signature `EtherTransferGenericFailure(address,uint256)` and selector `0x470c7c1d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "EtherTransferGenericFailure",
        abi = "EtherTransferGenericFailure(address,uint256)"
    )]
    pub struct EtherTransferGenericFailure {
        pub account: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `InsufficientEtherSupplied` with signature `InsufficientEtherSupplied()` and selector `0x1a783b8d`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InsufficientEtherSupplied", abi = "InsufficientEtherSupplied()")]
    pub struct InsufficientEtherSupplied;
    ///Custom Error type `Invalid1155BatchTransferEncoding` with signature `Invalid1155BatchTransferEncoding()` and selector `0xeba2084c`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "Invalid1155BatchTransferEncoding",
        abi = "Invalid1155BatchTransferEncoding()"
    )]
    pub struct Invalid1155BatchTransferEncoding;
    ///Custom Error type `InvalidBasicOrderParameterEncoding` with signature `InvalidBasicOrderParameterEncoding()` and selector `0x39f3e3fd`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InvalidBasicOrderParameterEncoding",
        abi = "InvalidBasicOrderParameterEncoding()"
    )]
    pub struct InvalidBasicOrderParameterEncoding;
    ///Custom Error type `InvalidCallToConduit` with signature `InvalidCallToConduit(address)` and selector `0xd13d53d4`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidCallToConduit", abi = "InvalidCallToConduit(address)")]
    pub struct InvalidCallToConduit {
        pub conduit: ::ethers::core::types::Address,
    }
    ///Custom Error type `InvalidCanceller` with signature `InvalidCanceller()` and selector `0x80ec7374`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidCanceller", abi = "InvalidCanceller()")]
    pub struct InvalidCanceller;
    ///Custom Error type `InvalidConduit` with signature `InvalidConduit(bytes32,address)` and selector `0x1cf99b26`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidConduit", abi = "InvalidConduit(bytes32,address)")]
    pub struct InvalidConduit {
        pub conduit_key: [u8; 32],
        pub conduit: ::ethers::core::types::Address,
    }
    ///Custom Error type `InvalidERC721TransferAmount` with signature `InvalidERC721TransferAmount()` and selector `0xefcc00b1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "InvalidERC721TransferAmount",
        abi = "InvalidERC721TransferAmount()"
    )]
    pub struct InvalidERC721TransferAmount;
    ///Custom Error type `InvalidMsgValue` with signature `InvalidMsgValue(uint256)` and selector `0xa61be9f0`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidMsgValue", abi = "InvalidMsgValue(uint256)")]
    pub struct InvalidMsgValue {
        pub value: ::ethers::core::types::U256,
    }
    ///Custom Error type `InvalidNativeOfferItem` with signature `InvalidNativeOfferItem()` and selector `0x12d3f5a3`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidNativeOfferItem", abi = "InvalidNativeOfferItem()")]
    pub struct InvalidNativeOfferItem;
    ///Custom Error type `InvalidSignature` with signature `InvalidSignature()` and selector `0x8baa579f`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidSignature", abi = "InvalidSignature()")]
    pub struct InvalidSignature;
    ///Custom Error type `InvalidSigner` with signature `InvalidSigner()` and selector `0x815e1d64`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidSigner", abi = "InvalidSigner()")]
    pub struct InvalidSigner;
    ///Custom Error type `InvalidTime` with signature `InvalidTime()` and selector `0x6f7eac26`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "InvalidTime", abi = "InvalidTime()")]
    pub struct InvalidTime;
    ///Custom Error type `MissingItemAmount` with signature `MissingItemAmount()` and selector `0x91b3e514`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "MissingItemAmount", abi = "MissingItemAmount()")]
    pub struct MissingItemAmount;
    ///Custom Error type `MissingOriginalConsiderationItems` with signature `MissingOriginalConsiderationItems()` and selector `0x466aa616`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "MissingOriginalConsiderationItems",
        abi = "MissingOriginalConsiderationItems()"
    )]
    pub struct MissingOriginalConsiderationItems;
    ///Custom Error type `NoContract` with signature `NoContract(address)` and selector `0x5f15d672`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NoContract", abi = "NoContract(address)")]
    pub struct NoContract {
        pub account: ::ethers::core::types::Address,
    }
    ///Custom Error type `NoReentrantCalls` with signature `NoReentrantCalls()` and selector `0x7fa8a987`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "NoReentrantCalls", abi = "NoReentrantCalls()")]
    pub struct NoReentrantCalls;
    ///Custom Error type `NoSpecifiedOrdersAvailable` with signature `NoSpecifiedOrdersAvailable()` and selector `0xd5da9a1b`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "NoSpecifiedOrdersAvailable",
        abi = "NoSpecifiedOrdersAvailable()"
    )]
    pub struct NoSpecifiedOrdersAvailable;
    ///Custom Error type `OrderAlreadyFilled` with signature `OrderAlreadyFilled(bytes32)` and selector `0x10fda3e1`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OrderAlreadyFilled", abi = "OrderAlreadyFilled(bytes32)")]
    pub struct OrderAlreadyFilled {
        pub order_hash: [u8; 32],
    }
    ///Custom Error type `OrderIsCancelled` with signature `OrderIsCancelled(bytes32)` and selector `0x1a515574`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OrderIsCancelled", abi = "OrderIsCancelled(bytes32)")]
    pub struct OrderIsCancelled {
        pub order_hash: [u8; 32],
    }
    ///Custom Error type `OrderPartiallyFilled` with signature `OrderPartiallyFilled(bytes32)` and selector `0xee9e0e63`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "OrderPartiallyFilled", abi = "OrderPartiallyFilled(bytes32)")]
    pub struct OrderPartiallyFilled {
        pub order_hash: [u8; 32],
    }
    ///Custom Error type `PartialFillsNotEnabledForOrder` with signature `PartialFillsNotEnabledForOrder()` and selector `0xa11b63ff`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "PartialFillsNotEnabledForOrder",
        abi = "PartialFillsNotEnabledForOrder()"
    )]
    pub struct PartialFillsNotEnabledForOrder;
    ///Custom Error type `TokenTransferGenericFailure` with signature `TokenTransferGenericFailure(address,address,address,uint256,uint256)` and selector `0xf486bc87`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(
        name = "TokenTransferGenericFailure",
        abi = "TokenTransferGenericFailure(address,address,address,uint256,uint256)"
    )]
    pub struct TokenTransferGenericFailure {
        pub token: ::ethers::core::types::Address,
        pub from: ::ethers::core::types::Address,
        pub to: ::ethers::core::types::Address,
        pub identifier: ::ethers::core::types::U256,
        pub amount: ::ethers::core::types::U256,
    }
    ///Custom Error type `UnusedItemParameters` with signature `UnusedItemParameters()` and selector `0x6ab37ce7`
    #[derive(
        Clone,
        ::ethers::contract::EthError,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[etherror(name = "UnusedItemParameters", abi = "UnusedItemParameters()")]
    pub struct UnusedItemParameters;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VerifiersErrors {
        BadContractSignature(BadContractSignature),
        BadFraction(BadFraction),
        BadReturnValueFromERC20OnTransfer(BadReturnValueFromERC20OnTransfer),
        BadSignatureV(BadSignatureV),
        ConsiderationNotMet(ConsiderationNotMet),
        ERC1155BatchTransferGenericFailure(ERC1155BatchTransferGenericFailure),
        EtherTransferGenericFailure(EtherTransferGenericFailure),
        InsufficientEtherSupplied(InsufficientEtherSupplied),
        Invalid1155BatchTransferEncoding(Invalid1155BatchTransferEncoding),
        InvalidBasicOrderParameterEncoding(InvalidBasicOrderParameterEncoding),
        InvalidCallToConduit(InvalidCallToConduit),
        InvalidCanceller(InvalidCanceller),
        InvalidConduit(InvalidConduit),
        InvalidERC721TransferAmount(InvalidERC721TransferAmount),
        InvalidMsgValue(InvalidMsgValue),
        InvalidNativeOfferItem(InvalidNativeOfferItem),
        InvalidSignature(InvalidSignature),
        InvalidSigner(InvalidSigner),
        InvalidTime(InvalidTime),
        MissingItemAmount(MissingItemAmount),
        MissingOriginalConsiderationItems(MissingOriginalConsiderationItems),
        NoContract(NoContract),
        NoReentrantCalls(NoReentrantCalls),
        NoSpecifiedOrdersAvailable(NoSpecifiedOrdersAvailable),
        OrderAlreadyFilled(OrderAlreadyFilled),
        OrderIsCancelled(OrderIsCancelled),
        OrderPartiallyFilled(OrderPartiallyFilled),
        PartialFillsNotEnabledForOrder(PartialFillsNotEnabledForOrder),
        TokenTransferGenericFailure(TokenTransferGenericFailure),
        UnusedItemParameters(UnusedItemParameters),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for VerifiersErrors {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <::std::string::String as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::RevertString(decoded));
            }
            if let Ok(decoded)
                = <BadContractSignature as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BadContractSignature(decoded));
            }
            if let Ok(decoded)
                = <BadFraction as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BadFraction(decoded));
            }
            if let Ok(decoded)
                = <BadReturnValueFromERC20OnTransfer as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BadReturnValueFromERC20OnTransfer(decoded));
            }
            if let Ok(decoded)
                = <BadSignatureV as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BadSignatureV(decoded));
            }
            if let Ok(decoded)
                = <ConsiderationNotMet as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ConsiderationNotMet(decoded));
            }
            if let Ok(decoded)
                = <ERC1155BatchTransferGenericFailure as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ERC1155BatchTransferGenericFailure(decoded));
            }
            if let Ok(decoded)
                = <EtherTransferGenericFailure as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::EtherTransferGenericFailure(decoded));
            }
            if let Ok(decoded)
                = <InsufficientEtherSupplied as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InsufficientEtherSupplied(decoded));
            }
            if let Ok(decoded)
                = <Invalid1155BatchTransferEncoding as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Invalid1155BatchTransferEncoding(decoded));
            }
            if let Ok(decoded)
                = <InvalidBasicOrderParameterEncoding as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidBasicOrderParameterEncoding(decoded));
            }
            if let Ok(decoded)
                = <InvalidCallToConduit as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidCallToConduit(decoded));
            }
            if let Ok(decoded)
                = <InvalidCanceller as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidCanceller(decoded));
            }
            if let Ok(decoded)
                = <InvalidConduit as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidConduit(decoded));
            }
            if let Ok(decoded)
                = <InvalidERC721TransferAmount as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidERC721TransferAmount(decoded));
            }
            if let Ok(decoded)
                = <InvalidMsgValue as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidMsgValue(decoded));
            }
            if let Ok(decoded)
                = <InvalidNativeOfferItem as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidNativeOfferItem(decoded));
            }
            if let Ok(decoded)
                = <InvalidSignature as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSignature(decoded));
            }
            if let Ok(decoded)
                = <InvalidSigner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSigner(decoded));
            }
            if let Ok(decoded)
                = <InvalidTime as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidTime(decoded));
            }
            if let Ok(decoded)
                = <MissingItemAmount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MissingItemAmount(decoded));
            }
            if let Ok(decoded)
                = <MissingOriginalConsiderationItems as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MissingOriginalConsiderationItems(decoded));
            }
            if let Ok(decoded)
                = <NoContract as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoContract(decoded));
            }
            if let Ok(decoded)
                = <NoReentrantCalls as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoReentrantCalls(decoded));
            }
            if let Ok(decoded)
                = <NoSpecifiedOrdersAvailable as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::NoSpecifiedOrdersAvailable(decoded));
            }
            if let Ok(decoded)
                = <OrderAlreadyFilled as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OrderAlreadyFilled(decoded));
            }
            if let Ok(decoded)
                = <OrderIsCancelled as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::OrderIsCancelled(decoded));
            }
            if let Ok(decoded)
                = <OrderPartiallyFilled as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OrderPartiallyFilled(decoded));
            }
            if let Ok(decoded)
                = <PartialFillsNotEnabledForOrder as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::PartialFillsNotEnabledForOrder(decoded));
            }
            if let Ok(decoded)
                = <TokenTransferGenericFailure as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TokenTransferGenericFailure(decoded));
            }
            if let Ok(decoded)
                = <UnusedItemParameters as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UnusedItemParameters(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for VerifiersErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BadContractSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BadFraction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BadReturnValueFromERC20OnTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BadSignatureV(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConsiderationNotMet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155BatchTransferGenericFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EtherTransferGenericFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientEtherSupplied(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invalid1155BatchTransferEncoding(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidBasicOrderParameterEncoding(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCallToConduit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidCanceller(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidConduit(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidERC721TransferAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidMsgValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidNativeOfferItem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MissingItemAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MissingOriginalConsiderationItems(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoContract(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoReentrantCalls(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoSpecifiedOrdersAvailable(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OrderAlreadyFilled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OrderIsCancelled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OrderPartiallyFilled(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PartialFillsNotEnabledForOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TokenTransferGenericFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnusedItemParameters(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for VerifiersErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BadContractSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BadFraction as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <BadReturnValueFromERC20OnTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BadSignatureV as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ConsiderationNotMet as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1155BatchTransferGenericFailure as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <EtherTransferGenericFailure as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InsufficientEtherSupplied as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Invalid1155BatchTransferEncoding as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidBasicOrderParameterEncoding as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCallToConduit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidCanceller as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidConduit as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidERC721TransferAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidMsgValue as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidNativeOfferItem as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidSigner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTime as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <MissingItemAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MissingOriginalConsiderationItems as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoContract as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <NoReentrantCalls as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoSpecifiedOrdersAvailable as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OrderAlreadyFilled as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OrderIsCancelled as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OrderPartiallyFilled as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <PartialFillsNotEnabledForOrder as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <TokenTransferGenericFailure as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnusedItemParameters as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for VerifiersErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BadContractSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BadFraction(element) => ::core::fmt::Display::fmt(element, f),
                Self::BadReturnValueFromERC20OnTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BadSignatureV(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConsiderationNotMet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1155BatchTransferGenericFailure(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EtherTransferGenericFailure(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientEtherSupplied(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Invalid1155BatchTransferEncoding(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidBasicOrderParameterEncoding(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidCallToConduit(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidCanceller(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidConduit(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidERC721TransferAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidMsgValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidNativeOfferItem(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::MissingItemAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::MissingOriginalConsiderationItems(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::NoContract(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoReentrantCalls(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoSpecifiedOrdersAvailable(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OrderAlreadyFilled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OrderIsCancelled(element) => ::core::fmt::Display::fmt(element, f),
                Self::OrderPartiallyFilled(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::PartialFillsNotEnabledForOrder(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenTransferGenericFailure(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnusedItemParameters(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for VerifiersErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BadContractSignature> for VerifiersErrors {
        fn from(value: BadContractSignature) -> Self {
            Self::BadContractSignature(value)
        }
    }
    impl ::core::convert::From<BadFraction> for VerifiersErrors {
        fn from(value: BadFraction) -> Self {
            Self::BadFraction(value)
        }
    }
    impl ::core::convert::From<BadReturnValueFromERC20OnTransfer> for VerifiersErrors {
        fn from(value: BadReturnValueFromERC20OnTransfer) -> Self {
            Self::BadReturnValueFromERC20OnTransfer(value)
        }
    }
    impl ::core::convert::From<BadSignatureV> for VerifiersErrors {
        fn from(value: BadSignatureV) -> Self {
            Self::BadSignatureV(value)
        }
    }
    impl ::core::convert::From<ConsiderationNotMet> for VerifiersErrors {
        fn from(value: ConsiderationNotMet) -> Self {
            Self::ConsiderationNotMet(value)
        }
    }
    impl ::core::convert::From<ERC1155BatchTransferGenericFailure> for VerifiersErrors {
        fn from(value: ERC1155BatchTransferGenericFailure) -> Self {
            Self::ERC1155BatchTransferGenericFailure(value)
        }
    }
    impl ::core::convert::From<EtherTransferGenericFailure> for VerifiersErrors {
        fn from(value: EtherTransferGenericFailure) -> Self {
            Self::EtherTransferGenericFailure(value)
        }
    }
    impl ::core::convert::From<InsufficientEtherSupplied> for VerifiersErrors {
        fn from(value: InsufficientEtherSupplied) -> Self {
            Self::InsufficientEtherSupplied(value)
        }
    }
    impl ::core::convert::From<Invalid1155BatchTransferEncoding> for VerifiersErrors {
        fn from(value: Invalid1155BatchTransferEncoding) -> Self {
            Self::Invalid1155BatchTransferEncoding(value)
        }
    }
    impl ::core::convert::From<InvalidBasicOrderParameterEncoding> for VerifiersErrors {
        fn from(value: InvalidBasicOrderParameterEncoding) -> Self {
            Self::InvalidBasicOrderParameterEncoding(value)
        }
    }
    impl ::core::convert::From<InvalidCallToConduit> for VerifiersErrors {
        fn from(value: InvalidCallToConduit) -> Self {
            Self::InvalidCallToConduit(value)
        }
    }
    impl ::core::convert::From<InvalidCanceller> for VerifiersErrors {
        fn from(value: InvalidCanceller) -> Self {
            Self::InvalidCanceller(value)
        }
    }
    impl ::core::convert::From<InvalidConduit> for VerifiersErrors {
        fn from(value: InvalidConduit) -> Self {
            Self::InvalidConduit(value)
        }
    }
    impl ::core::convert::From<InvalidERC721TransferAmount> for VerifiersErrors {
        fn from(value: InvalidERC721TransferAmount) -> Self {
            Self::InvalidERC721TransferAmount(value)
        }
    }
    impl ::core::convert::From<InvalidMsgValue> for VerifiersErrors {
        fn from(value: InvalidMsgValue) -> Self {
            Self::InvalidMsgValue(value)
        }
    }
    impl ::core::convert::From<InvalidNativeOfferItem> for VerifiersErrors {
        fn from(value: InvalidNativeOfferItem) -> Self {
            Self::InvalidNativeOfferItem(value)
        }
    }
    impl ::core::convert::From<InvalidSignature> for VerifiersErrors {
        fn from(value: InvalidSignature) -> Self {
            Self::InvalidSignature(value)
        }
    }
    impl ::core::convert::From<InvalidSigner> for VerifiersErrors {
        fn from(value: InvalidSigner) -> Self {
            Self::InvalidSigner(value)
        }
    }
    impl ::core::convert::From<InvalidTime> for VerifiersErrors {
        fn from(value: InvalidTime) -> Self {
            Self::InvalidTime(value)
        }
    }
    impl ::core::convert::From<MissingItemAmount> for VerifiersErrors {
        fn from(value: MissingItemAmount) -> Self {
            Self::MissingItemAmount(value)
        }
    }
    impl ::core::convert::From<MissingOriginalConsiderationItems> for VerifiersErrors {
        fn from(value: MissingOriginalConsiderationItems) -> Self {
            Self::MissingOriginalConsiderationItems(value)
        }
    }
    impl ::core::convert::From<NoContract> for VerifiersErrors {
        fn from(value: NoContract) -> Self {
            Self::NoContract(value)
        }
    }
    impl ::core::convert::From<NoReentrantCalls> for VerifiersErrors {
        fn from(value: NoReentrantCalls) -> Self {
            Self::NoReentrantCalls(value)
        }
    }
    impl ::core::convert::From<NoSpecifiedOrdersAvailable> for VerifiersErrors {
        fn from(value: NoSpecifiedOrdersAvailable) -> Self {
            Self::NoSpecifiedOrdersAvailable(value)
        }
    }
    impl ::core::convert::From<OrderAlreadyFilled> for VerifiersErrors {
        fn from(value: OrderAlreadyFilled) -> Self {
            Self::OrderAlreadyFilled(value)
        }
    }
    impl ::core::convert::From<OrderIsCancelled> for VerifiersErrors {
        fn from(value: OrderIsCancelled) -> Self {
            Self::OrderIsCancelled(value)
        }
    }
    impl ::core::convert::From<OrderPartiallyFilled> for VerifiersErrors {
        fn from(value: OrderPartiallyFilled) -> Self {
            Self::OrderPartiallyFilled(value)
        }
    }
    impl ::core::convert::From<PartialFillsNotEnabledForOrder> for VerifiersErrors {
        fn from(value: PartialFillsNotEnabledForOrder) -> Self {
            Self::PartialFillsNotEnabledForOrder(value)
        }
    }
    impl ::core::convert::From<TokenTransferGenericFailure> for VerifiersErrors {
        fn from(value: TokenTransferGenericFailure) -> Self {
            Self::TokenTransferGenericFailure(value)
        }
    }
    impl ::core::convert::From<UnusedItemParameters> for VerifiersErrors {
        fn from(value: UnusedItemParameters) -> Self {
            Self::UnusedItemParameters(value)
        }
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "CounterIncremented", abi = "CounterIncremented(uint256,address)")]
    pub struct CounterIncrementedFilter {
        pub new_counter: ::ethers::core::types::U256,
        #[ethevent(indexed)]
        pub offerer: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "OrderCancelled", abi = "OrderCancelled(bytes32,address,address)")]
    pub struct OrderCancelledFilter {
        pub order_hash: [u8; 32],
        #[ethevent(indexed)]
        pub offerer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub zone: ::ethers::core::types::Address,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(
        name = "OrderFulfilled",
        abi = "OrderFulfilled(bytes32,address,address,address,(uint8,address,uint256,uint256)[],(uint8,address,uint256,uint256,address)[])"
    )]
    pub struct OrderFulfilledFilter {
        pub order_hash: [u8; 32],
        #[ethevent(indexed)]
        pub offerer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub zone: ::ethers::core::types::Address,
        pub recipient: ::ethers::core::types::Address,
        pub offer: ::std::vec::Vec<SpentItem>,
        pub consideration: ::std::vec::Vec<ReceivedItem>,
    }
    #[derive(
        Clone,
        ::ethers::contract::EthEvent,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethevent(name = "OrderValidated", abi = "OrderValidated(bytes32,address,address)")]
    pub struct OrderValidatedFilter {
        pub order_hash: [u8; 32],
        #[ethevent(indexed)]
        pub offerer: ::ethers::core::types::Address,
        #[ethevent(indexed)]
        pub zone: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum VerifiersEvents {
        CounterIncrementedFilter(CounterIncrementedFilter),
        OrderCancelledFilter(OrderCancelledFilter),
        OrderFulfilledFilter(OrderFulfilledFilter),
        OrderValidatedFilter(OrderValidatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for VerifiersEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CounterIncrementedFilter::decode_log(log) {
                return Ok(VerifiersEvents::CounterIncrementedFilter(decoded));
            }
            if let Ok(decoded) = OrderCancelledFilter::decode_log(log) {
                return Ok(VerifiersEvents::OrderCancelledFilter(decoded));
            }
            if let Ok(decoded) = OrderFulfilledFilter::decode_log(log) {
                return Ok(VerifiersEvents::OrderFulfilledFilter(decoded));
            }
            if let Ok(decoded) = OrderValidatedFilter::decode_log(log) {
                return Ok(VerifiersEvents::OrderValidatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for VerifiersEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CounterIncrementedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OrderCancelledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OrderFulfilledFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OrderValidatedFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<CounterIncrementedFilter> for VerifiersEvents {
        fn from(value: CounterIncrementedFilter) -> Self {
            Self::CounterIncrementedFilter(value)
        }
    }
    impl ::core::convert::From<OrderCancelledFilter> for VerifiersEvents {
        fn from(value: OrderCancelledFilter) -> Self {
            Self::OrderCancelledFilter(value)
        }
    }
    impl ::core::convert::From<OrderFulfilledFilter> for VerifiersEvents {
        fn from(value: OrderFulfilledFilter) -> Self {
            Self::OrderFulfilledFilter(value)
        }
    }
    impl ::core::convert::From<OrderValidatedFilter> for VerifiersEvents {
        fn from(value: OrderValidatedFilter) -> Self {
            Self::OrderValidatedFilter(value)
        }
    }
}
