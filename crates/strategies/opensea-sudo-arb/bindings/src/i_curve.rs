pub use i_curve::*;
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
pub mod i_curve {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"spotPrice\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"delta\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"numItems\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeMultiplier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protocolFeeMultiplier\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBuyInfo\",\"outputs\":[{\"internalType\":\"enum CurveErrorCodes.Error\",\"name\":\"error\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"newSpotPrice\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"newDelta\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"inputValue\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protocolFee\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"spotPrice\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"delta\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"numItems\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"feeMultiplier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protocolFeeMultiplier\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSellInfo\",\"outputs\":[{\"internalType\":\"enum CurveErrorCodes.Error\",\"name\":\"error\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"newSpotPrice\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"newDelta\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"outputValue\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protocolFee\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"delta\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"validateDelta\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"valid\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"newSpotPrice\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"validateSpotPrice\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"valid\",\"type\":\"bool\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static ICURVE_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct ICurve<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ICurve<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ICurve<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ICurve<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ICurve<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ICurve)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ICurve<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    ICURVE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `getBuyInfo` (0x7ca542ac) function
        pub fn get_buy_info(
            &self,
            spot_price: u128,
            delta: u128,
            num_items: ::ethers::core::types::U256,
            fee_multiplier: ::ethers::core::types::U256,
            protocol_fee_multiplier: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u8, u128, u128, ::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [124, 165, 66, 172],
                    (
                        spot_price,
                        delta,
                        num_items,
                        fee_multiplier,
                        protocol_fee_multiplier,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSellInfo` (0x097cc63d) function
        pub fn get_sell_info(
            &self,
            spot_price: u128,
            delta: u128,
            num_items: ::ethers::core::types::U256,
            fee_multiplier: ::ethers::core::types::U256,
            protocol_fee_multiplier: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (u8, u128, u128, ::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash(
                    [9, 124, 198, 61],
                    (
                        spot_price,
                        delta,
                        num_items,
                        fee_multiplier,
                        protocol_fee_multiplier,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateDelta` (0x0ae67ccc) function
        pub fn validate_delta(
            &self,
            delta: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([10, 230, 124, 204], delta)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validateSpotPrice` (0xa1bbb2e8) function
        pub fn validate_spot_price(
            &self,
            new_spot_price: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([161, 187, 178, 232], new_spot_price)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ICurve<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `getBuyInfo` function with signature `getBuyInfo(uint128,uint128,uint256,uint256,uint256)` and selector `0x7ca542ac`
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
    #[ethcall(
        name = "getBuyInfo",
        abi = "getBuyInfo(uint128,uint128,uint256,uint256,uint256)"
    )]
    pub struct GetBuyInfoCall {
        pub spot_price: u128,
        pub delta: u128,
        pub num_items: ::ethers::core::types::U256,
        pub fee_multiplier: ::ethers::core::types::U256,
        pub protocol_fee_multiplier: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getSellInfo` function with signature `getSellInfo(uint128,uint128,uint256,uint256,uint256)` and selector `0x097cc63d`
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
    #[ethcall(
        name = "getSellInfo",
        abi = "getSellInfo(uint128,uint128,uint256,uint256,uint256)"
    )]
    pub struct GetSellInfoCall {
        pub spot_price: u128,
        pub delta: u128,
        pub num_items: ::ethers::core::types::U256,
        pub fee_multiplier: ::ethers::core::types::U256,
        pub protocol_fee_multiplier: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `validateDelta` function with signature `validateDelta(uint128)` and selector `0x0ae67ccc`
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
    #[ethcall(name = "validateDelta", abi = "validateDelta(uint128)")]
    pub struct ValidateDeltaCall {
        pub delta: u128,
    }
    ///Container type for all input parameters for the `validateSpotPrice` function with signature `validateSpotPrice(uint128)` and selector `0xa1bbb2e8`
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
    #[ethcall(name = "validateSpotPrice", abi = "validateSpotPrice(uint128)")]
    pub struct ValidateSpotPriceCall {
        pub new_spot_price: u128,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum ICurveCalls {
        GetBuyInfo(GetBuyInfoCall),
        GetSellInfo(GetSellInfoCall),
        ValidateDelta(ValidateDeltaCall),
        ValidateSpotPrice(ValidateSpotPriceCall),
    }
    impl ::ethers::core::abi::AbiDecode for ICurveCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <GetBuyInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBuyInfo(decoded));
            }
            if let Ok(decoded)
                = <GetSellInfoCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSellInfo(decoded));
            }
            if let Ok(decoded)
                = <ValidateDeltaCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ValidateDelta(decoded));
            }
            if let Ok(decoded)
                = <ValidateSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ValidateSpotPrice(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ICurveCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::GetBuyInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSellInfo(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ValidateSpotPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ICurveCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::GetBuyInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSellInfo(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateDelta(element) => ::core::fmt::Display::fmt(element, f),
                Self::ValidateSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<GetBuyInfoCall> for ICurveCalls {
        fn from(value: GetBuyInfoCall) -> Self {
            Self::GetBuyInfo(value)
        }
    }
    impl ::core::convert::From<GetSellInfoCall> for ICurveCalls {
        fn from(value: GetSellInfoCall) -> Self {
            Self::GetSellInfo(value)
        }
    }
    impl ::core::convert::From<ValidateDeltaCall> for ICurveCalls {
        fn from(value: ValidateDeltaCall) -> Self {
            Self::ValidateDelta(value)
        }
    }
    impl ::core::convert::From<ValidateSpotPriceCall> for ICurveCalls {
        fn from(value: ValidateSpotPriceCall) -> Self {
            Self::ValidateSpotPrice(value)
        }
    }
    ///Container type for all return fields from the `getBuyInfo` function with signature `getBuyInfo(uint128,uint128,uint256,uint256,uint256)` and selector `0x7ca542ac`
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
    pub struct GetBuyInfoReturn {
        pub error: u8,
        pub new_spot_price: u128,
        pub new_delta: u128,
        pub input_value: ::ethers::core::types::U256,
        pub protocol_fee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getSellInfo` function with signature `getSellInfo(uint128,uint128,uint256,uint256,uint256)` and selector `0x097cc63d`
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
    pub struct GetSellInfoReturn {
        pub error: u8,
        pub new_spot_price: u128,
        pub new_delta: u128,
        pub output_value: ::ethers::core::types::U256,
        pub protocol_fee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `validateDelta` function with signature `validateDelta(uint128)` and selector `0x0ae67ccc`
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
    pub struct ValidateDeltaReturn {
        pub valid: bool,
    }
    ///Container type for all return fields from the `validateSpotPrice` function with signature `validateSpotPrice(uint128)` and selector `0xa1bbb2e8`
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
    pub struct ValidateSpotPriceReturn {
        pub valid: bool,
    }
}
