pub use fulfillment_applier::*;
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
pub mod fulfillment_applier {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidFulfillmentComponentData\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MismatchedFulfillmentOfferAndConsiderationComponents\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"enum Side\",\"name\":\"side\",\"type\":\"uint8\",\"components\":[]}],\"type\":\"error\",\"name\":\"MissingFulfillmentComponentOnAggregation\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OfferAndConsiderationRequiredOnFulfillment\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static FULFILLMENTAPPLIER_ABI: ::ethers::contract::Lazy<
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
        220,
        35,
        241,
        177,
        140,
        29,
        78,
        120,
        135,
        162,
        33,
        138,
        122,
        177,
        245,
        57,
        32,
        198,
        211,
        58,
        73,
        188,
        61,
        87,
        208,
        120,
        29,
        254,
        179,
        121,
        31,
        240,
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
    pub static FULFILLMENTAPPLIER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
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
        220,
        35,
        241,
        177,
        140,
        29,
        78,
        120,
        135,
        162,
        33,
        138,
        122,
        177,
        245,
        57,
        32,
        198,
        211,
        58,
        73,
        188,
        61,
        87,
        208,
        120,
        29,
        254,
        179,
        121,
        31,
        240,
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
    pub static FULFILLMENTAPPLIER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct FulfillmentApplier<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for FulfillmentApplier<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for FulfillmentApplier<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for FulfillmentApplier<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for FulfillmentApplier<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(FulfillmentApplier)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> FulfillmentApplier<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    FULFILLMENTAPPLIER_ABI.clone(),
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
                FULFILLMENTAPPLIER_ABI.clone(),
                FULFILLMENTAPPLIER_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for FulfillmentApplier<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `InvalidFulfillmentComponentData` with signature `InvalidFulfillmentComponentData()` and selector `0x7fda7279`
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
        name = "InvalidFulfillmentComponentData",
        abi = "InvalidFulfillmentComponentData()"
    )]
    pub struct InvalidFulfillmentComponentData;
    ///Custom Error type `MismatchedFulfillmentOfferAndConsiderationComponents` with signature `MismatchedFulfillmentOfferAndConsiderationComponents()` and selector `0x09cfb455`
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
        name = "MismatchedFulfillmentOfferAndConsiderationComponents",
        abi = "MismatchedFulfillmentOfferAndConsiderationComponents()"
    )]
    pub struct MismatchedFulfillmentOfferAndConsiderationComponents;
    ///Custom Error type `MissingFulfillmentComponentOnAggregation` with signature `MissingFulfillmentComponentOnAggregation(uint8)` and selector `0x375c24c1`
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
        name = "MissingFulfillmentComponentOnAggregation",
        abi = "MissingFulfillmentComponentOnAggregation(uint8)"
    )]
    pub struct MissingFulfillmentComponentOnAggregation {
        pub side: u8,
    }
    ///Custom Error type `OfferAndConsiderationRequiredOnFulfillment` with signature `OfferAndConsiderationRequiredOnFulfillment()` and selector `0x98e9db6e`
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
        name = "OfferAndConsiderationRequiredOnFulfillment",
        abi = "OfferAndConsiderationRequiredOnFulfillment()"
    )]
    pub struct OfferAndConsiderationRequiredOnFulfillment;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum FulfillmentApplierErrors {
        InvalidFulfillmentComponentData(InvalidFulfillmentComponentData),
        MismatchedFulfillmentOfferAndConsiderationComponents(
            MismatchedFulfillmentOfferAndConsiderationComponents,
        ),
        MissingFulfillmentComponentOnAggregation(
            MissingFulfillmentComponentOnAggregation,
        ),
        OfferAndConsiderationRequiredOnFulfillment(
            OfferAndConsiderationRequiredOnFulfillment,
        ),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for FulfillmentApplierErrors {
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
                = <InvalidFulfillmentComponentData as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::InvalidFulfillmentComponentData(decoded));
            }
            if let Ok(decoded)
                = <MismatchedFulfillmentOfferAndConsiderationComponents as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(
                    Self::MismatchedFulfillmentOfferAndConsiderationComponents(decoded),
                );
            }
            if let Ok(decoded)
                = <MissingFulfillmentComponentOnAggregation as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MissingFulfillmentComponentOnAggregation(decoded));
            }
            if let Ok(decoded)
                = <OfferAndConsiderationRequiredOnFulfillment as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OfferAndConsiderationRequiredOnFulfillment(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for FulfillmentApplierErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::InvalidFulfillmentComponentData(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MismatchedFulfillmentOfferAndConsiderationComponents(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MissingFulfillmentComponentOnAggregation(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OfferAndConsiderationRequiredOnFulfillment(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for FulfillmentApplierErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <InvalidFulfillmentComponentData as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MismatchedFulfillmentOfferAndConsiderationComponents as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <MissingFulfillmentComponentOnAggregation as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OfferAndConsiderationRequiredOnFulfillment as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for FulfillmentApplierErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::InvalidFulfillmentComponentData(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MismatchedFulfillmentOfferAndConsiderationComponents(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MissingFulfillmentComponentOnAggregation(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OfferAndConsiderationRequiredOnFulfillment(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for FulfillmentApplierErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<InvalidFulfillmentComponentData>
    for FulfillmentApplierErrors {
        fn from(value: InvalidFulfillmentComponentData) -> Self {
            Self::InvalidFulfillmentComponentData(value)
        }
    }
    impl ::core::convert::From<MismatchedFulfillmentOfferAndConsiderationComponents>
    for FulfillmentApplierErrors {
        fn from(value: MismatchedFulfillmentOfferAndConsiderationComponents) -> Self {
            Self::MismatchedFulfillmentOfferAndConsiderationComponents(value)
        }
    }
    impl ::core::convert::From<MissingFulfillmentComponentOnAggregation>
    for FulfillmentApplierErrors {
        fn from(value: MissingFulfillmentComponentOnAggregation) -> Self {
            Self::MissingFulfillmentComponentOnAggregation(value)
        }
    }
    impl ::core::convert::From<OfferAndConsiderationRequiredOnFulfillment>
    for FulfillmentApplierErrors {
        fn from(value: OfferAndConsiderationRequiredOnFulfillment) -> Self {
            Self::OfferAndConsiderationRequiredOnFulfillment(value)
        }
    }
}
