pub use token_transferrer::*;
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
pub mod token_transferrer {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"BadReturnValueFromERC20OnTransfer\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"identifiers\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}],\"type\":\"error\",\"name\":\"ERC1155BatchTransferGenericFailure\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Invalid1155BatchTransferEncoding\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidERC721TransferAmount\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MissingItemAmount\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"NoContract\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"from\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"to\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"TokenTransferGenericFailure\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"UnusedItemParameters\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static TOKENTRANSFERRER_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    #[rustfmt::skip]
    const __BYTECODE: &[u8] = &[
        96,
        128,
        96,
        64,
        82,
        52,
        128,
        21,
        96,
        15,
        87,
        96,
        0,
        128,
        253,
        91,
        80,
        96,
        63,
        128,
        96,
        29,
        96,
        0,
        57,
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
        149,
        43,
        213,
        41,
        102,
        109,
        174,
        10,
        167,
        9,
        250,
        235,
        248,
        200,
        136,
        150,
        78,
        34,
        155,
        106,
        57,
        51,
        254,
        255,
        192,
        232,
        39,
        192,
        152,
        125,
        114,
        16,
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
    pub static TOKENTRANSFERRER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
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
        149,
        43,
        213,
        41,
        102,
        109,
        174,
        10,
        167,
        9,
        250,
        235,
        248,
        200,
        136,
        150,
        78,
        34,
        155,
        106,
        57,
        51,
        254,
        255,
        192,
        232,
        39,
        192,
        152,
        125,
        114,
        16,
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
    pub static TOKENTRANSFERRER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct TokenTransferrer<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for TokenTransferrer<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for TokenTransferrer<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for TokenTransferrer<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for TokenTransferrer<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(TokenTransferrer)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> TokenTransferrer<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    TOKENTRANSFERRER_ABI.clone(),
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
                TOKENTRANSFERRER_ABI.clone(),
                TOKENTRANSFERRER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for TokenTransferrer<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    pub enum TokenTransferrerErrors {
        BadReturnValueFromERC20OnTransfer(BadReturnValueFromERC20OnTransfer),
        ERC1155BatchTransferGenericFailure(ERC1155BatchTransferGenericFailure),
        Invalid1155BatchTransferEncoding(Invalid1155BatchTransferEncoding),
        InvalidERC721TransferAmount(InvalidERC721TransferAmount),
        MissingItemAmount(MissingItemAmount),
        NoContract(NoContract),
        TokenTransferGenericFailure(TokenTransferGenericFailure),
        UnusedItemParameters(UnusedItemParameters),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for TokenTransferrerErrors {
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
                = <BadReturnValueFromERC20OnTransfer as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::BadReturnValueFromERC20OnTransfer(decoded));
            }
            if let Ok(decoded)
                = <ERC1155BatchTransferGenericFailure as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ERC1155BatchTransferGenericFailure(decoded));
            }
            if let Ok(decoded)
                = <Invalid1155BatchTransferEncoding as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Invalid1155BatchTransferEncoding(decoded));
            }
            if let Ok(decoded)
                = <InvalidERC721TransferAmount as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidERC721TransferAmount(decoded));
            }
            if let Ok(decoded)
                = <MissingItemAmount as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MissingItemAmount(decoded));
            }
            if let Ok(decoded)
                = <NoContract as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::NoContract(decoded));
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
    impl ::ethers::core::abi::AbiEncode for TokenTransferrerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BadReturnValueFromERC20OnTransfer(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ERC1155BatchTransferGenericFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Invalid1155BatchTransferEncoding(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidERC721TransferAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MissingItemAmount(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::NoContract(element) => {
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
    impl ::ethers::contract::ContractRevert for TokenTransferrerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BadReturnValueFromERC20OnTransfer as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <ERC1155BatchTransferGenericFailure as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Invalid1155BatchTransferEncoding as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidERC721TransferAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MissingItemAmount as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <NoContract as ::ethers::contract::EthError>::selector() => true,
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
    impl ::core::fmt::Display for TokenTransferrerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BadReturnValueFromERC20OnTransfer(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ERC1155BatchTransferGenericFailure(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Invalid1155BatchTransferEncoding(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidERC721TransferAmount(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MissingItemAmount(element) => ::core::fmt::Display::fmt(element, f),
                Self::NoContract(element) => ::core::fmt::Display::fmt(element, f),
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
    impl ::core::convert::From<::std::string::String> for TokenTransferrerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BadReturnValueFromERC20OnTransfer>
    for TokenTransferrerErrors {
        fn from(value: BadReturnValueFromERC20OnTransfer) -> Self {
            Self::BadReturnValueFromERC20OnTransfer(value)
        }
    }
    impl ::core::convert::From<ERC1155BatchTransferGenericFailure>
    for TokenTransferrerErrors {
        fn from(value: ERC1155BatchTransferGenericFailure) -> Self {
            Self::ERC1155BatchTransferGenericFailure(value)
        }
    }
    impl ::core::convert::From<Invalid1155BatchTransferEncoding>
    for TokenTransferrerErrors {
        fn from(value: Invalid1155BatchTransferEncoding) -> Self {
            Self::Invalid1155BatchTransferEncoding(value)
        }
    }
    impl ::core::convert::From<InvalidERC721TransferAmount> for TokenTransferrerErrors {
        fn from(value: InvalidERC721TransferAmount) -> Self {
            Self::InvalidERC721TransferAmount(value)
        }
    }
    impl ::core::convert::From<MissingItemAmount> for TokenTransferrerErrors {
        fn from(value: MissingItemAmount) -> Self {
            Self::MissingItemAmount(value)
        }
    }
    impl ::core::convert::From<NoContract> for TokenTransferrerErrors {
        fn from(value: NoContract) -> Self {
            Self::NoContract(value)
        }
    }
    impl ::core::convert::From<TokenTransferGenericFailure> for TokenTransferrerErrors {
        fn from(value: TokenTransferGenericFailure) -> Self {
            Self::TokenTransferGenericFailure(value)
        }
    }
    impl ::core::convert::From<UnusedItemParameters> for TokenTransferrerErrors {
        fn from(value: UnusedItemParameters) -> Self {
            Self::UnusedItemParameters(value)
        }
    }
}
