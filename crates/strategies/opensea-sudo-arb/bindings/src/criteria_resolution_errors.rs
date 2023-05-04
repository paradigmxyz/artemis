pub use criteria_resolution_errors::*;
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
pub mod criteria_resolution_errors {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"ConsiderationCriteriaResolverOutOfRange\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"CriteriaNotEnabledForItem\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidProof\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OfferCriteriaResolverOutOfRange\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"OrderCriteriaResolverOutOfRange\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"UnresolvedConsiderationCriteria\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"UnresolvedOfferCriteria\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static CRITERIARESOLUTIONERRORS_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct CriteriaResolutionErrors<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CriteriaResolutionErrors<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CriteriaResolutionErrors<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CriteriaResolutionErrors<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CriteriaResolutionErrors<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(CriteriaResolutionErrors))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CriteriaResolutionErrors<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CRITERIARESOLUTIONERRORS_ABI.clone(),
                    client,
                ),
            )
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CriteriaResolutionErrors<M> {
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
    pub enum CriteriaResolutionErrorsErrors {
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
    impl ::ethers::core::abi::AbiDecode for CriteriaResolutionErrorsErrors {
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
    impl ::ethers::core::abi::AbiEncode for CriteriaResolutionErrorsErrors {
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
    impl ::ethers::contract::ContractRevert for CriteriaResolutionErrorsErrors {
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
    impl ::core::fmt::Display for CriteriaResolutionErrorsErrors {
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
    impl ::core::convert::From<::std::string::String>
    for CriteriaResolutionErrorsErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<ConsiderationCriteriaResolverOutOfRange>
    for CriteriaResolutionErrorsErrors {
        fn from(value: ConsiderationCriteriaResolverOutOfRange) -> Self {
            Self::ConsiderationCriteriaResolverOutOfRange(value)
        }
    }
    impl ::core::convert::From<CriteriaNotEnabledForItem>
    for CriteriaResolutionErrorsErrors {
        fn from(value: CriteriaNotEnabledForItem) -> Self {
            Self::CriteriaNotEnabledForItem(value)
        }
    }
    impl ::core::convert::From<InvalidProof> for CriteriaResolutionErrorsErrors {
        fn from(value: InvalidProof) -> Self {
            Self::InvalidProof(value)
        }
    }
    impl ::core::convert::From<OfferCriteriaResolverOutOfRange>
    for CriteriaResolutionErrorsErrors {
        fn from(value: OfferCriteriaResolverOutOfRange) -> Self {
            Self::OfferCriteriaResolverOutOfRange(value)
        }
    }
    impl ::core::convert::From<OrderCriteriaResolverOutOfRange>
    for CriteriaResolutionErrorsErrors {
        fn from(value: OrderCriteriaResolverOutOfRange) -> Self {
            Self::OrderCriteriaResolverOutOfRange(value)
        }
    }
    impl ::core::convert::From<UnresolvedConsiderationCriteria>
    for CriteriaResolutionErrorsErrors {
        fn from(value: UnresolvedConsiderationCriteria) -> Self {
            Self::UnresolvedConsiderationCriteria(value)
        }
    }
    impl ::core::convert::From<UnresolvedOfferCriteria>
    for CriteriaResolutionErrorsErrors {
        fn from(value: UnresolvedOfferCriteria) -> Self {
            Self::UnresolvedOfferCriteria(value)
        }
    }
}
