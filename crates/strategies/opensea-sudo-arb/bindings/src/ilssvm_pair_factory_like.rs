pub use ilssvm_pair_factory_like::*;
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
pub mod ilssvm_pair_factory_like {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"target\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"callAllowed\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"potentialPair\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"enum ILSSVMPairFactoryLike.PairVariant\",\"name\":\"variant\",\"type\":\"uint8\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"isPair\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"protocolFeeMultiplier\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"protocolFeeRecipient\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"contract LSSVMRouter\",\"name\":\"router\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"routerStatus\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"allowed\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"wasEverAllowed\",\"type\":\"bool\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static ILSSVMPAIRFACTORYLIKE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct ILSSVMPairFactoryLike<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ILSSVMPairFactoryLike<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ILSSVMPairFactoryLike<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ILSSVMPairFactoryLike<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ILSSVMPairFactoryLike<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ILSSVMPairFactoryLike))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ILSSVMPairFactoryLike<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ILSSVMPAIRFACTORYLIKE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `callAllowed` (0x1fba95e8) function
        pub fn call_allowed(
            &self,
            target: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([31, 186, 149, 232], target)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `isPair` (0x08f25a8f) function
        pub fn is_pair(
            &self,
            potential_pair: ::ethers::core::types::Address,
            variant: u8,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([8, 242, 90, 143], (potential_pair, variant))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolFeeMultiplier` (0x1ce4c78b) function
        pub fn protocol_fee_multiplier(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([28, 228, 199, 139], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `protocolFeeRecipient` (0x64df049e) function
        pub fn protocol_fee_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([100, 223, 4, 158], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `routerStatus` (0xa93ec68b) function
        pub fn router_status(
            &self,
            router: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, (bool, bool)> {
            self.0
                .method_hash([169, 62, 198, 139], router)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ILSSVMPairFactoryLike<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `callAllowed` function with signature `callAllowed(address)` and selector `0x1fba95e8`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "callAllowed", abi = "callAllowed(address)")]
    pub struct CallAllowedCall {
        pub target: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `isPair` function with signature `isPair(address,uint8)` and selector `0x08f25a8f`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "isPair", abi = "isPair(address,uint8)")]
    pub struct IsPairCall {
        pub potential_pair: ::ethers::core::types::Address,
        pub variant: u8,
    }
    ///Container type for all input parameters for the `protocolFeeMultiplier` function with signature `protocolFeeMultiplier()` and selector `0x1ce4c78b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "protocolFeeMultiplier", abi = "protocolFeeMultiplier()")]
    pub struct ProtocolFeeMultiplierCall;
    ///Container type for all input parameters for the `protocolFeeRecipient` function with signature `protocolFeeRecipient()` and selector `0x64df049e`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "protocolFeeRecipient", abi = "protocolFeeRecipient()")]
    pub struct ProtocolFeeRecipientCall;
    ///Container type for all input parameters for the `routerStatus` function with signature `routerStatus(address)` and selector `0xa93ec68b`
    #[derive(
        Clone,
        ::ethers::contract::EthCall,
        ::ethers::contract::EthDisplay,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "routerStatus", abi = "routerStatus(address)")]
    pub struct RouterStatusCall {
        pub router: ::ethers::core::types::Address,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ILSSVMPairFactoryLikeCalls {
        CallAllowed(CallAllowedCall),
        IsPair(IsPairCall),
        ProtocolFeeMultiplier(ProtocolFeeMultiplierCall),
        ProtocolFeeRecipient(ProtocolFeeRecipientCall),
        RouterStatus(RouterStatusCall),
    }
    impl ::ethers::core::abi::AbiDecode for ILSSVMPairFactoryLikeCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CallAllowedCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::CallAllowed(decoded));
            }
            if let Ok(decoded)
                = <IsPairCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::IsPair(decoded));
            }
            if let Ok(decoded)
                = <ProtocolFeeMultiplierCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProtocolFeeMultiplier(decoded));
            }
            if let Ok(decoded)
                = <ProtocolFeeRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ProtocolFeeRecipient(decoded));
            }
            if let Ok(decoded)
                = <RouterStatusCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::RouterStatus(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ILSSVMPairFactoryLikeCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::CallAllowed(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IsPair(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ProtocolFeeMultiplier(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ProtocolFeeRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RouterStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ILSSVMPairFactoryLikeCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::CallAllowed(element) => ::core::fmt::Display::fmt(element, f),
                Self::IsPair(element) => ::core::fmt::Display::fmt(element, f),
                Self::ProtocolFeeMultiplier(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ProtocolFeeRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::RouterStatus(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CallAllowedCall> for ILSSVMPairFactoryLikeCalls {
        fn from(value: CallAllowedCall) -> Self {
            Self::CallAllowed(value)
        }
    }
    impl ::core::convert::From<IsPairCall> for ILSSVMPairFactoryLikeCalls {
        fn from(value: IsPairCall) -> Self {
            Self::IsPair(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeMultiplierCall>
    for ILSSVMPairFactoryLikeCalls {
        fn from(value: ProtocolFeeMultiplierCall) -> Self {
            Self::ProtocolFeeMultiplier(value)
        }
    }
    impl ::core::convert::From<ProtocolFeeRecipientCall> for ILSSVMPairFactoryLikeCalls {
        fn from(value: ProtocolFeeRecipientCall) -> Self {
            Self::ProtocolFeeRecipient(value)
        }
    }
    impl ::core::convert::From<RouterStatusCall> for ILSSVMPairFactoryLikeCalls {
        fn from(value: RouterStatusCall) -> Self {
            Self::RouterStatus(value)
        }
    }
    ///Container type for all return fields from the `callAllowed` function with signature `callAllowed(address)` and selector `0x1fba95e8`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct CallAllowedReturn(pub bool);
    ///Container type for all return fields from the `isPair` function with signature `isPair(address,uint8)` and selector `0x08f25a8f`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct IsPairReturn(pub bool);
    ///Container type for all return fields from the `protocolFeeMultiplier` function with signature `protocolFeeMultiplier()` and selector `0x1ce4c78b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProtocolFeeMultiplierReturn(pub ::ethers::core::types::U256);
    ///Container type for all return fields from the `protocolFeeRecipient` function with signature `protocolFeeRecipient()` and selector `0x64df049e`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct ProtocolFeeRecipientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `routerStatus` function with signature `routerStatus(address)` and selector `0xa93ec68b`
    #[derive(
        Clone,
        ::ethers::contract::EthAbiType,
        ::ethers::contract::EthAbiCodec,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub struct RouterStatusReturn {
        pub allowed: bool,
        pub was_ever_allowed: bool,
    }
}
