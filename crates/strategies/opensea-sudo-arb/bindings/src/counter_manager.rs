pub use counter_manager::*;
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
pub mod counter_manager {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"BadFraction\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"considerationIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"shortfallAmount\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"ConsiderationNotMet\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"account\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"EtherTransferGenericFailure\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InsufficientEtherSupplied\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidBasicOrderParameterEncoding\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidCallToConduit\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidCanceller\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"conduit\",\"type\":\"address\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidConduit\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"value\",\"type\":\"uint256\",\"components\":[]}],\"type\":\"error\",\"name\":\"InvalidMsgValue\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidNativeOfferItem\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidTime\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"MissingOriginalConsiderationItems\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NoReentrantCalls\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"NoSpecifiedOrdersAvailable\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"OrderAlreadyFilled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"OrderIsCancelled\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[]}],\"type\":\"error\",\"name\":\"OrderPartiallyFilled\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"PartialFillsNotEnabledForOrder\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"newCounter\",\"type\":\"uint256\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"CounterIncremented\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OrderCancelled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[],\"indexed\":false},{\"internalType\":\"struct SpentItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"indexed\":false},{\"internalType\":\"struct ReceivedItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"indexed\":false}],\"type\":\"event\",\"name\":\"OrderFulfilled\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[],\"indexed\":false},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[],\"indexed\":true},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OrderValidated\",\"outputs\":[],\"anonymous\":false}]";
    ///The parsed JSON ABI of the contract.
    pub static COUNTERMANAGER_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
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
        1,
        96,
        0,
        85,
        96,
        63,
        128,
        96,
        34,
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
        54,
        50,
        103,
        85,
        214,
        18,
        13,
        191,
        27,
        1,
        121,
        111,
        194,
        15,
        108,
        30,
        7,
        3,
        26,
        199,
        82,
        120,
        255,
        0,
        187,
        210,
        82,
        57,
        101,
        23,
        49,
        90,
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
    pub static COUNTERMANAGER_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
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
        54,
        50,
        103,
        85,
        214,
        18,
        13,
        191,
        27,
        1,
        121,
        111,
        194,
        15,
        108,
        30,
        7,
        3,
        26,
        199,
        82,
        120,
        255,
        0,
        187,
        210,
        82,
        57,
        101,
        23,
        49,
        90,
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
    pub static COUNTERMANAGER_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct CounterManager<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for CounterManager<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for CounterManager<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for CounterManager<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for CounterManager<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(CounterManager)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> CounterManager<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    COUNTERMANAGER_ABI.clone(),
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
                COUNTERMANAGER_ABI.clone(),
                COUNTERMANAGER_BYTECODE.clone().into(),
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
            CounterManagerEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for CounterManager<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum CounterManagerErrors {
        BadFraction(BadFraction),
        ConsiderationNotMet(ConsiderationNotMet),
        EtherTransferGenericFailure(EtherTransferGenericFailure),
        InsufficientEtherSupplied(InsufficientEtherSupplied),
        InvalidBasicOrderParameterEncoding(InvalidBasicOrderParameterEncoding),
        InvalidCallToConduit(InvalidCallToConduit),
        InvalidCanceller(InvalidCanceller),
        InvalidConduit(InvalidConduit),
        InvalidMsgValue(InvalidMsgValue),
        InvalidNativeOfferItem(InvalidNativeOfferItem),
        InvalidTime(InvalidTime),
        MissingOriginalConsiderationItems(MissingOriginalConsiderationItems),
        NoReentrantCalls(NoReentrantCalls),
        NoSpecifiedOrdersAvailable(NoSpecifiedOrdersAvailable),
        OrderAlreadyFilled(OrderAlreadyFilled),
        OrderIsCancelled(OrderIsCancelled),
        OrderPartiallyFilled(OrderPartiallyFilled),
        PartialFillsNotEnabledForOrder(PartialFillsNotEnabledForOrder),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for CounterManagerErrors {
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
                = <BadFraction as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BadFraction(decoded));
            }
            if let Ok(decoded)
                = <ConsiderationNotMet as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ConsiderationNotMet(decoded));
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
                = <InvalidTime as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidTime(decoded));
            }
            if let Ok(decoded)
                = <MissingOriginalConsiderationItems as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MissingOriginalConsiderationItems(decoded));
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
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for CounterManagerErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BadFraction(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ConsiderationNotMet(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::EtherTransferGenericFailure(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InsufficientEtherSupplied(element) => {
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
                Self::InvalidMsgValue(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidNativeOfferItem(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidTime(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MissingOriginalConsiderationItems(element) => {
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
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for CounterManagerErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BadFraction as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <ConsiderationNotMet as ::ethers::contract::EthError>::selector() => {
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
                    == <InvalidMsgValue as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidNativeOfferItem as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <InvalidTime as ::ethers::contract::EthError>::selector() => true,
                _ if selector
                    == <MissingOriginalConsiderationItems as ::ethers::contract::EthError>::selector() => {
                    true
                }
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for CounterManagerErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BadFraction(element) => ::core::fmt::Display::fmt(element, f),
                Self::ConsiderationNotMet(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::EtherTransferGenericFailure(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InsufficientEtherSupplied(element) => {
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
                Self::InvalidMsgValue(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidNativeOfferItem(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::InvalidTime(element) => ::core::fmt::Display::fmt(element, f),
                Self::MissingOriginalConsiderationItems(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
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
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for CounterManagerErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BadFraction> for CounterManagerErrors {
        fn from(value: BadFraction) -> Self {
            Self::BadFraction(value)
        }
    }
    impl ::core::convert::From<ConsiderationNotMet> for CounterManagerErrors {
        fn from(value: ConsiderationNotMet) -> Self {
            Self::ConsiderationNotMet(value)
        }
    }
    impl ::core::convert::From<EtherTransferGenericFailure> for CounterManagerErrors {
        fn from(value: EtherTransferGenericFailure) -> Self {
            Self::EtherTransferGenericFailure(value)
        }
    }
    impl ::core::convert::From<InsufficientEtherSupplied> for CounterManagerErrors {
        fn from(value: InsufficientEtherSupplied) -> Self {
            Self::InsufficientEtherSupplied(value)
        }
    }
    impl ::core::convert::From<InvalidBasicOrderParameterEncoding>
    for CounterManagerErrors {
        fn from(value: InvalidBasicOrderParameterEncoding) -> Self {
            Self::InvalidBasicOrderParameterEncoding(value)
        }
    }
    impl ::core::convert::From<InvalidCallToConduit> for CounterManagerErrors {
        fn from(value: InvalidCallToConduit) -> Self {
            Self::InvalidCallToConduit(value)
        }
    }
    impl ::core::convert::From<InvalidCanceller> for CounterManagerErrors {
        fn from(value: InvalidCanceller) -> Self {
            Self::InvalidCanceller(value)
        }
    }
    impl ::core::convert::From<InvalidConduit> for CounterManagerErrors {
        fn from(value: InvalidConduit) -> Self {
            Self::InvalidConduit(value)
        }
    }
    impl ::core::convert::From<InvalidMsgValue> for CounterManagerErrors {
        fn from(value: InvalidMsgValue) -> Self {
            Self::InvalidMsgValue(value)
        }
    }
    impl ::core::convert::From<InvalidNativeOfferItem> for CounterManagerErrors {
        fn from(value: InvalidNativeOfferItem) -> Self {
            Self::InvalidNativeOfferItem(value)
        }
    }
    impl ::core::convert::From<InvalidTime> for CounterManagerErrors {
        fn from(value: InvalidTime) -> Self {
            Self::InvalidTime(value)
        }
    }
    impl ::core::convert::From<MissingOriginalConsiderationItems>
    for CounterManagerErrors {
        fn from(value: MissingOriginalConsiderationItems) -> Self {
            Self::MissingOriginalConsiderationItems(value)
        }
    }
    impl ::core::convert::From<NoReentrantCalls> for CounterManagerErrors {
        fn from(value: NoReentrantCalls) -> Self {
            Self::NoReentrantCalls(value)
        }
    }
    impl ::core::convert::From<NoSpecifiedOrdersAvailable> for CounterManagerErrors {
        fn from(value: NoSpecifiedOrdersAvailable) -> Self {
            Self::NoSpecifiedOrdersAvailable(value)
        }
    }
    impl ::core::convert::From<OrderAlreadyFilled> for CounterManagerErrors {
        fn from(value: OrderAlreadyFilled) -> Self {
            Self::OrderAlreadyFilled(value)
        }
    }
    impl ::core::convert::From<OrderIsCancelled> for CounterManagerErrors {
        fn from(value: OrderIsCancelled) -> Self {
            Self::OrderIsCancelled(value)
        }
    }
    impl ::core::convert::From<OrderPartiallyFilled> for CounterManagerErrors {
        fn from(value: OrderPartiallyFilled) -> Self {
            Self::OrderPartiallyFilled(value)
        }
    }
    impl ::core::convert::From<PartialFillsNotEnabledForOrder> for CounterManagerErrors {
        fn from(value: PartialFillsNotEnabledForOrder) -> Self {
            Self::PartialFillsNotEnabledForOrder(value)
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
    pub enum CounterManagerEvents {
        CounterIncrementedFilter(CounterIncrementedFilter),
        OrderCancelledFilter(OrderCancelledFilter),
        OrderFulfilledFilter(OrderFulfilledFilter),
        OrderValidatedFilter(OrderValidatedFilter),
    }
    impl ::ethers::contract::EthLogDecode for CounterManagerEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = CounterIncrementedFilter::decode_log(log) {
                return Ok(CounterManagerEvents::CounterIncrementedFilter(decoded));
            }
            if let Ok(decoded) = OrderCancelledFilter::decode_log(log) {
                return Ok(CounterManagerEvents::OrderCancelledFilter(decoded));
            }
            if let Ok(decoded) = OrderFulfilledFilter::decode_log(log) {
                return Ok(CounterManagerEvents::OrderFulfilledFilter(decoded));
            }
            if let Ok(decoded) = OrderValidatedFilter::decode_log(log) {
                return Ok(CounterManagerEvents::OrderValidatedFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for CounterManagerEvents {
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
    impl ::core::convert::From<CounterIncrementedFilter> for CounterManagerEvents {
        fn from(value: CounterIncrementedFilter) -> Self {
            Self::CounterIncrementedFilter(value)
        }
    }
    impl ::core::convert::From<OrderCancelledFilter> for CounterManagerEvents {
        fn from(value: OrderCancelledFilter) -> Self {
            Self::OrderCancelledFilter(value)
        }
    }
    impl ::core::convert::From<OrderFulfilledFilter> for CounterManagerEvents {
        fn from(value: OrderFulfilledFilter) -> Self {
            Self::OrderFulfilledFilter(value)
        }
    }
    impl ::core::convert::From<OrderValidatedFilter> for CounterManagerEvents {
        fn from(value: OrderValidatedFilter) -> Self {
            Self::OrderValidatedFilter(value)
        }
    }
}
