pub use criteria_resolution::*;
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
pub mod criteria_resolution {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"ConsiderationCriteriaResolverOutOfRange\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CriteriaNotEnabledForItem\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidProof\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OfferCriteriaResolverOutOfRange\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OrderCriteriaResolverOutOfRange\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"UnresolvedConsiderationCriteria\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"UnresolvedOfferCriteria\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static CRITERIARESOLUTION_ABI: ::ethers::contract::Lazy<
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
        53,
        79,
        63,
        147,
        177,
        211,
        213,
        214,
        94,
        237,
        132,
        192,
        214,
        28,
        146,
        218,
        203,
        136,
        44,
        50,
        243,
        122,
        41,
        239,
        218,
        194,
        183,
        17,
        138,
        111,
        135,
        127,
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
    pub static CRITERIARESOLUTION_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
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
        53,
        79,
        63,
        147,
        177,
        211,
        213,
        214,
        94,
        237,
        132,
        192,
        214,
        28,
        146,
        218,
        203,
        136,
        44,
        50,
        243,
        122,
        41,
        239,
        218,
        194,
        183,
        17,
        138,
        111,
        135,
        127,
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
    pub static CRITERIARESOLUTION_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CriteriaResolution<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CriteriaResolution<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CriteriaResolution<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CriteriaResolution<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CriteriaResolution<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(CriteriaResolution)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CriteriaResolution<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CRITERIARESOLUTION_ABI.clone(),
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
                CRITERIARESOLUTION_ABI.clone(),
                CRITERIARESOLUTION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CriteriaResolution<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `ConsiderationCriteriaResolverOutOfRange` with signature `ConsiderationCriteriaResolverOutOfRange()` and selector `0x6088d7de`
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
        name = "ConsiderationCriteriaResolverOutOfRange",
        abi = "ConsiderationCriteriaResolverOutOfRange()"
    )]
    pub struct ConsiderationCriteriaResolverOutOfRange;
    ///Custom Error type `CriteriaNotEnabledForItem` with signature `CriteriaNotEnabledForItem()` and selector `0x94eb6af6`
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
    #[etherror(name = "CriteriaNotEnabledForItem", abi = "CriteriaNotEnabledForItem()")]
    pub struct CriteriaNotEnabledForItem;
    ///Custom Error type `InvalidProof` with signature `InvalidProof()` and selector `0x09bde339`
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
    #[etherror(name = "InvalidProof", abi = "InvalidProof()")]
    pub struct InvalidProof;
    ///Custom Error type `OfferCriteriaResolverOutOfRange` with signature `OfferCriteriaResolverOutOfRange()` and selector `0xbfb3f8ce`
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
        name = "OfferCriteriaResolverOutOfRange",
        abi = "OfferCriteriaResolverOutOfRange()"
    )]
    pub struct OfferCriteriaResolverOutOfRange;
    ///Custom Error type `OrderCriteriaResolverOutOfRange` with signature `OrderCriteriaResolverOutOfRange()` and selector `0x869586c4`
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
        name = "OrderCriteriaResolverOutOfRange",
        abi = "OrderCriteriaResolverOutOfRange()"
    )]
    pub struct OrderCriteriaResolverOutOfRange;
    ///Custom Error type `UnresolvedConsiderationCriteria` with signature `UnresolvedConsiderationCriteria()` and selector `0xff75a340`
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
        name = "UnresolvedConsiderationCriteria",
        abi = "UnresolvedConsiderationCriteria()"
    )]
    pub struct UnresolvedConsiderationCriteria;
    ///Custom Error type `UnresolvedOfferCriteria` with signature `UnresolvedOfferCriteria()` and selector `0xa6cfc673`
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
    #[etherror(name = "UnresolvedOfferCriteria", abi = "UnresolvedOfferCriteria()")]
    pub struct UnresolvedOfferCriteria;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CriteriaResolutionErrors {
        ConsiderationCriteriaResolverOutOfRange(ConsiderationCriteriaResolverOutOfRange),
        CriteriaNotEnabledForItem(CriteriaNotEnabledForItem),
        InvalidProof(InvalidProof),
        OfferCriteriaResolverOutOfRange(OfferCriteriaResolverOutOfRange),
        OrderCriteriaResolverOutOfRange(OrderCriteriaResolverOutOfRange),
        UnresolvedConsiderationCriteria(UnresolvedConsiderationCriteria),
        UnresolvedOfferCriteria(UnresolvedOfferCriteria),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for CriteriaResolutionErrors {
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
                = <ConsiderationCriteriaResolverOutOfRange as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ConsiderationCriteriaResolverOutOfRange(decoded));
            }
            if let Ok(decoded)
                = <CriteriaNotEnabledForItem as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::CriteriaNotEnabledForItem(decoded));
            }
            if let Ok(decoded)
                = <InvalidProof as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidProof(decoded));
            }
            if let Ok(decoded)
                = <OfferCriteriaResolverOutOfRange as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OfferCriteriaResolverOutOfRange(decoded));
            }
            if let Ok(decoded)
                = <OrderCriteriaResolverOutOfRange as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OrderCriteriaResolverOutOfRange(decoded));
            }
            if let Ok(decoded)
                = <UnresolvedConsiderationCriteria as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UnresolvedConsiderationCriteria(decoded));
            }
            if let Ok(decoded)
                = <UnresolvedOfferCriteria as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::UnresolvedOfferCriteria(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CriteriaResolutionErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::ConsiderationCriteriaResolverOutOfRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::CriteriaNotEnabledForItem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidProof(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OfferCriteriaResolverOutOfRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OrderCriteriaResolverOutOfRange(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnresolvedConsiderationCriteria(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::UnresolvedOfferCriteria(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for CriteriaResolutionErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <ConsiderationCriteriaResolverOutOfRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <CriteriaNotEnabledForItem as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidProof as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <OfferCriteriaResolverOutOfRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <OrderCriteriaResolverOutOfRange as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnresolvedConsiderationCriteria as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <UnresolvedOfferCriteria as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for CriteriaResolutionErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::ConsiderationCriteriaResolverOutOfRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::CriteriaNotEnabledForItem(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidProof(element) => ::core::fmt::Display::fmt(element, f),
                Self::OfferCriteriaResolverOutOfRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OrderCriteriaResolverOutOfRange(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnresolvedConsiderationCriteria(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::UnresolvedOfferCriteria(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for CriteriaResolutionErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ConsiderationCriteriaResolverOutOfRange>
    for CriteriaResolutionErrors {
        fn from(value: ConsiderationCriteriaResolverOutOfRange) -> Self {
            Self::ConsiderationCriteriaResolverOutOfRange(value)
        }
    }
    impl ::core::convert::From<CriteriaNotEnabledForItem> for CriteriaResolutionErrors {
        fn from(value: CriteriaNotEnabledForItem) -> Self {
            Self::CriteriaNotEnabledForItem(value)
        }
    }
    impl ::core::convert::From<InvalidProof> for CriteriaResolutionErrors {
        fn from(value: InvalidProof) -> Self {
            Self::InvalidProof(value)
        }
    }
    impl ::core::convert::From<OfferCriteriaResolverOutOfRange>
    for CriteriaResolutionErrors {
        fn from(value: OfferCriteriaResolverOutOfRange) -> Self {
            Self::OfferCriteriaResolverOutOfRange(value)
        }
    }
    impl ::core::convert::From<OrderCriteriaResolverOutOfRange>
    for CriteriaResolutionErrors {
        fn from(value: OrderCriteriaResolverOutOfRange) -> Self {
            Self::OrderCriteriaResolverOutOfRange(value)
        }
    }
    impl ::core::convert::From<UnresolvedConsiderationCriteria>
    for CriteriaResolutionErrors {
        fn from(value: UnresolvedConsiderationCriteria) -> Self {
            Self::UnresolvedConsiderationCriteria(value)
        }
    }
    impl ::core::convert::From<UnresolvedOfferCriteria> for CriteriaResolutionErrors {
        fn from(value: UnresolvedOfferCriteria) -> Self {
            Self::UnresolvedOfferCriteria(value)
        }
    }
}
