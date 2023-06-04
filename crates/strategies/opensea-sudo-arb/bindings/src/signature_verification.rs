pub use signature_verification::*;
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
pub mod signature_verification {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[],\"type\":\"error\",\"name\":\"BadContractSignature\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint8\",\"name\":\"v\",\"type\":\"uint8\",\"components\":[]}],\"type\":\"error\",\"name\":\"BadSignatureV\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSignature\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"InvalidSigner\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static SIGNATUREVERIFICATION_ABI: ::ethers::contract::Lazy<
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
        0,
        143,
        120,
        14,
        117,
        153,
        197,
        1,
        174,
        102,
        173,
        227,
        129,
        22,
        142,
        171,
        157,
        90,
        131,
        14,
        183,
        63,
        100,
        213,
        30,
        106,
        219,
        227,
        118,
        200,
        210,
        65,
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
    pub static SIGNATUREVERIFICATION_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
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
        0,
        143,
        120,
        14,
        117,
        153,
        197,
        1,
        174,
        102,
        173,
        227,
        129,
        22,
        142,
        171,
        157,
        90,
        131,
        14,
        183,
        63,
        100,
        213,
        30,
        106,
        219,
        227,
        118,
        200,
        210,
        65,
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
    pub static SIGNATUREVERIFICATION_DEPLOYED_BYTECODE: ::ethers::core::types::Bytes = ::ethers::core::types::Bytes::from_static(
        __DEPLOYED_BYTECODE,
    );
    pub struct SignatureVerification<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for SignatureVerification<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for SignatureVerification<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for SignatureVerification<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for SignatureVerification<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(SignatureVerification))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> SignatureVerification<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    SIGNATUREVERIFICATION_ABI.clone(),
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
                SIGNATUREVERIFICATION_ABI.clone(),
                SIGNATUREVERIFICATION_BYTECODE.clone().into(),
                client,
            );
            let deployer = factory.deploy(constructor_args)?;
            let deployer = ::ethers::contract::ContractDeployer::new(deployer);
            Ok(deployer)
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for SignatureVerification<M> {
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
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum SignatureVerificationErrors {
        BadContractSignature(BadContractSignature),
        BadSignatureV(BadSignatureV),
        InvalidSignature(InvalidSignature),
        InvalidSigner(InvalidSigner),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for SignatureVerificationErrors {
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
                = <BadSignatureV as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BadSignatureV(decoded));
            }
            if let Ok(decoded)
                = <InvalidSignature as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSignature(decoded));
            }
            if let Ok(decoded)
                = <InvalidSigner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::InvalidSigner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for SignatureVerificationErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BadContractSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BadSignatureV(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSignature(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::InvalidSigner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for SignatureVerificationErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BadContractSignature as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <BadSignatureV as ::ethers::contract::EthError>::selector() => {
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
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for SignatureVerificationErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BadContractSignature(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::BadSignatureV(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSignature(element) => ::core::fmt::Display::fmt(element, f),
                Self::InvalidSigner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for SignatureVerificationErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BadContractSignature> for SignatureVerificationErrors {
        fn from(value: BadContractSignature) -> Self {
            Self::BadContractSignature(value)
        }
    }
    impl ::core::convert::From<BadSignatureV> for SignatureVerificationErrors {
        fn from(value: BadSignatureV) -> Self {
            Self::BadSignatureV(value)
        }
    }
    impl ::core::convert::From<InvalidSignature> for SignatureVerificationErrors {
        fn from(value: InvalidSignature) -> Self {
            Self::InvalidSignature(value)
        }
    }
    impl ::core::convert::From<InvalidSigner> for SignatureVerificationErrors {
        fn from(value: InvalidSigner) -> Self {
            Self::InvalidSigner(value)
        }
    }
}
