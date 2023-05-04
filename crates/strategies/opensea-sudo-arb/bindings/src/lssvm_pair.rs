pub use lssvm_pair::*;
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
pub mod lssvm_pair {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"enum CurveErrorCodes.Error\",\"name\":\"error\",\"type\":\"uint8\",\"components\":[]}],\"type\":\"error\",\"name\":\"BondingCurveError\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Ownable_NewOwnerZeroAddress\",\"outputs\":[]},{\"inputs\":[],\"type\":\"error\",\"name\":\"Ownable_NotOwner\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"a\",\"type\":\"address\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"AssetRecipientChange\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"newDelta\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"DeltaUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint96\",\"name\":\"newFee\",\"type\":\"uint96\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"FeeUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"NFTWithdrawal\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[],\"indexed\":true}],\"type\":\"event\",\"name\":\"OwnershipTransferred\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"newSpotPrice\",\"type\":\"uint128\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"SpotPriceUpdate\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"SwapNFTInPair\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"type\":\"event\",\"name\":\"SwapNFTOutPair\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenDeposit\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[],\"indexed\":false}],\"type\":\"event\",\"name\":\"TokenWithdrawal\",\"outputs\":[],\"anonymous\":false},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"assetRecipient\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"bondingCurve\",\"outputs\":[{\"internalType\":\"contract ICurve\",\"name\":\"_bondingCurve\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"target\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"data\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"call\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"address payable\",\"name\":\"newRecipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeAssetRecipient\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"newDelta\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeDelta\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint96\",\"name\":\"newFee\",\"type\":\"uint96\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeFee\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"uint128\",\"name\":\"newSpotPrice\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"changeSpotPrice\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"delta\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"factory\",\"outputs\":[{\"internalType\":\"contract ILSSVMPairFactoryLike\",\"name\":\"_factory\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"fee\",\"outputs\":[{\"internalType\":\"uint96\",\"name\":\"\",\"type\":\"uint96\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAllHeldIds\",\"outputs\":[{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getAssetRecipient\",\"outputs\":[{\"internalType\":\"address payable\",\"name\":\"_assetRecipient\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numNFTs\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getBuyNFTQuote\",\"outputs\":[{\"internalType\":\"enum CurveErrorCodes.Error\",\"name\":\"error\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newSpotPrice\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newDelta\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"inputAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protocolFee\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numNFTs\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getSellNFTQuote\",\"outputs\":[{\"internalType\":\"enum CurveErrorCodes.Error\",\"name\":\"error\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newSpotPrice\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"newDelta\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"outputAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"protocolFee\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"_owner\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"_assetRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"_delta\",\"type\":\"uint128\",\"components\":[]},{\"internalType\":\"uint96\",\"name\":\"_fee\",\"type\":\"uint96\",\"components\":[]},{\"internalType\":\"uint128\",\"name\":\"_spotPrice\",\"type\":\"uint128\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"initialize\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"bytes[]\",\"name\":\"calls\",\"type\":\"bytes[]\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"revertOnFail\",\"type\":\"bool\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"multicall\",\"outputs\":[]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"nft\",\"outputs\":[{\"internalType\":\"contract IERC721\",\"name\":\"_nft\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC1155BatchReceived\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"\",\"type\":\"bytes\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"onERC1155Received\",\"outputs\":[{\"internalType\":\"bytes4\",\"name\":\"\",\"type\":\"bytes4\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"owner\",\"outputs\":[{\"internalType\":\"address\",\"name\":\"\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"pairVariant\",\"outputs\":[{\"internalType\":\"enum ILSSVMPairFactoryLike.PairVariant\",\"name\":\"\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"pure\",\"type\":\"function\",\"name\":\"poolType\",\"outputs\":[{\"internalType\":\"enum LSSVMPair.PoolType\",\"name\":\"_poolType\",\"type\":\"uint8\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"spotPrice\",\"outputs\":[{\"internalType\":\"uint128\",\"name\":\"\",\"type\":\"uint128\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"interfaceId\",\"type\":\"bytes4\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"supportsInterface\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"nftIds\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"minExpectedTokenOutput\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"tokenRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isRouter\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"routerCaller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"swapNFTsForToken\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"outputAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"numNFTs\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxExpectedTokenInput\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"nftRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isRouter\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"routerCaller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapTokenForAnyNFTs\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"inputAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"uint256[]\",\"name\":\"nftIds\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maxExpectedTokenInput\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"nftRecipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isRouter\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"routerCaller\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"swapTokenForSpecificNFTs\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"inputAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"newOwner\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"transferOwnership\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IERC1155\",\"name\":\"a\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"ids\",\"type\":\"uint256[]\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"amounts\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawERC1155\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract ERC20\",\"name\":\"a\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawERC20\",\"outputs\":[]},{\"inputs\":[{\"internalType\":\"contract IERC721\",\"name\":\"a\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256[]\",\"name\":\"nftIds\",\"type\":\"uint256[]\",\"components\":[]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"withdrawERC721\",\"outputs\":[]}]";
    ///The parsed JSON ABI of the contract.
    pub static LSSVMPAIR_ABI: ::ethers::contract::Lazy<::ethers::core::abi::Abi> = ::ethers::contract::Lazy::new(||
    ::ethers::core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct LSSVMPair<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for LSSVMPair<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for LSSVMPair<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for LSSVMPair<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for LSSVMPair<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(LSSVMPair)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> LSSVMPair<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    LSSVMPAIR_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `assetRecipient` (0x3bfa67fe) function
        pub fn asset_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([59, 250, 103, 254], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `bondingCurve` (0xeff1d50e) function
        pub fn bonding_curve(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([239, 241, 213, 14], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `call` (0x1b8b921d) function
        pub fn call(
            &self,
            target: ::ethers::core::types::Address,
            data: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 139, 146, 29], (target, data))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeAssetRecipient` (0xf4629549) function
        pub fn change_asset_recipient(
            &self,
            new_recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 98, 149, 73], new_recipient)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeDelta` (0x6809f664) function
        pub fn change_delta(
            &self,
            new_delta: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 9, 246, 100], new_delta)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeFee` (0x6b7a2200) function
        pub fn change_fee(
            &self,
            new_fee: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 122, 34, 0], new_fee)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `changeSpotPrice` (0xd8a1890c) function
        pub fn change_spot_price(
            &self,
            new_spot_price: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 161, 137, 12], new_spot_price)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `delta` (0x12b495a8) function
        pub fn delta(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([18, 180, 149, 168], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `factory` (0xc45a0155) function
        pub fn factory(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([196, 90, 1, 85], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fee` (0xddca3f43) function
        pub fn fee(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([221, 202, 63, 67], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAllHeldIds` (0x2f4fefaf) function
        pub fn get_all_held_ids(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::std::vec::Vec<::ethers::core::types::U256>,
        > {
            self.0
                .method_hash([47, 79, 239, 175], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getAssetRecipient` (0x79eac6c2) function
        pub fn get_asset_recipient(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([121, 234, 198, 194], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getBuyNFTQuote` (0xa5cb2b91) function
        pub fn get_buy_nft_quote(
            &self,
            num_nf_ts: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u8,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([165, 203, 43, 145], num_nf_ts)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getSellNFTQuote` (0x0c295e56) function
        pub fn get_sell_nft_quote(
            &self,
            num_nf_ts: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (
                u8,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
                ::ethers::core::types::U256,
            ),
        > {
            self.0
                .method_hash([12, 41, 94, 86], num_nf_ts)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `initialize` (0xfd17aef9) function
        pub fn initialize(
            &self,
            owner: ::ethers::core::types::Address,
            asset_recipient: ::ethers::core::types::Address,
            delta: u128,
            fee: u128,
            spot_price: u128,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash(
                    [253, 23, 174, 249],
                    (owner, asset_recipient, delta, fee, spot_price),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `multicall` (0x1e9701d4) function
        pub fn multicall(
            &self,
            calls: ::std::vec::Vec<::ethers::core::types::Bytes>,
            revert_on_fail: bool,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 151, 1, 212], (calls, revert_on_fail))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `nft` (0x47ccca02) function
        pub fn nft(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([71, 204, 202, 2], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155BatchReceived` (0xbc197c81) function
        pub fn on_erc1155_batch_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::std::vec::Vec<::ethers::core::types::U256>,
            p3: ::std::vec::Vec<::ethers::core::types::U256>,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([188, 25, 124, 129], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `onERC1155Received` (0xf23a6e61) function
        pub fn on_erc1155_received(
            &self,
            p0: ::ethers::core::types::Address,
            p1: ::ethers::core::types::Address,
            p2: ::ethers::core::types::U256,
            p3: ::ethers::core::types::U256,
            p4: ::ethers::core::types::Bytes,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 4]> {
            self.0
                .method_hash([242, 58, 110, 97], (p0, p1, p2, p3, p4))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `owner` (0x8da5cb5b) function
        pub fn owner(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            ::ethers::core::types::Address,
        > {
            self.0
                .method_hash([141, 165, 203, 91], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `pairVariant` (0x3053fc58) function
        pub fn pair_variant(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([48, 83, 252, 88], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `poolType` (0xb1dd61b6) function
        pub fn pool_type(&self) -> ::ethers::contract::builders::ContractCall<M, u8> {
            self.0
                .method_hash([177, 221, 97, 182], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `spotPrice` (0x398482d8) function
        pub fn spot_price(&self) -> ::ethers::contract::builders::ContractCall<M, u128> {
            self.0
                .method_hash([57, 132, 130, 216], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `supportsInterface` (0x01ffc9a7) function
        pub fn supports_interface(
            &self,
            interface_id: [u8; 4],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([1, 255, 201, 167], interface_id)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapNFTsForToken` (0xb1d3f1c1) function
        pub fn swap_nf_ts_for_token(
            &self,
            nft_ids: ::std::vec::Vec<::ethers::core::types::U256>,
            min_expected_token_output: ::ethers::core::types::U256,
            token_recipient: ::ethers::core::types::Address,
            is_router: bool,
            router_caller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [177, 211, 241, 193],
                    (
                        nft_ids,
                        min_expected_token_output,
                        token_recipient,
                        is_router,
                        router_caller,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapTokenForAnyNFTs` (0x28b8aee1) function
        pub fn swap_token_for_any_nf_ts(
            &self,
            num_nf_ts: ::ethers::core::types::U256,
            max_expected_token_input: ::ethers::core::types::U256,
            nft_recipient: ::ethers::core::types::Address,
            is_router: bool,
            router_caller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [40, 184, 174, 225],
                    (
                        num_nf_ts,
                        max_expected_token_input,
                        nft_recipient,
                        is_router,
                        router_caller,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `swapTokenForSpecificNFTs` (0x6d8b99f7) function
        pub fn swap_token_for_specific_nf_ts(
            &self,
            nft_ids: ::std::vec::Vec<::ethers::core::types::U256>,
            max_expected_token_input: ::ethers::core::types::U256,
            nft_recipient: ::ethers::core::types::Address,
            is_router: bool,
            router_caller: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash(
                    [109, 139, 153, 247],
                    (
                        nft_ids,
                        max_expected_token_input,
                        nft_recipient,
                        is_router,
                        router_caller,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `transferOwnership` (0xf2fde38b) function
        pub fn transfer_ownership(
            &self,
            new_owner: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 253, 227, 139], new_owner)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawERC1155` (0xa5ceac99) function
        pub fn withdraw_erc1155(
            &self,
            a: ::ethers::core::types::Address,
            ids: ::std::vec::Vec<::ethers::core::types::U256>,
            amounts: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 206, 172, 153], (a, ids, amounts))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawERC20` (0xa1db9782) function
        pub fn withdraw_erc20(
            &self,
            a: ::ethers::core::types::Address,
            amount: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 219, 151, 130], (a, amount))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `withdrawERC721` (0x13edab81) function
        pub fn withdraw_erc721(
            &self,
            a: ::ethers::core::types::Address,
            nft_ids: ::std::vec::Vec<::ethers::core::types::U256>,
        ) -> ::ethers::contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 237, 171, 129], (a, nft_ids))
                .expect("method not found (this should never happen)")
        }
        ///Gets the contract's `AssetRecipientChange` event
        pub fn asset_recipient_change_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            AssetRecipientChangeFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `DeltaUpdate` event
        pub fn delta_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            DeltaUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `FeeUpdate` event
        pub fn fee_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            FeeUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `NFTWithdrawal` event
        pub fn nft_withdrawal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            NftwithdrawalFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `OwnershipTransferred` event
        pub fn ownership_transferred_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            OwnershipTransferredFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SpotPriceUpdate` event
        pub fn spot_price_update_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SpotPriceUpdateFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SwapNFTInPair` event
        pub fn swap_nft_in_pair_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SwapNFTInPairFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `SwapNFTOutPair` event
        pub fn swap_nft_out_pair_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            SwapNFTOutPairFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenDeposit` event
        pub fn token_deposit_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenDepositFilter,
        > {
            self.0.event()
        }
        ///Gets the contract's `TokenWithdrawal` event
        pub fn token_withdrawal_filter(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            TokenWithdrawalFilter,
        > {
            self.0.event()
        }
        /// Returns an `Event` builder for all the events of this contract.
        pub fn events(
            &self,
        ) -> ::ethers::contract::builders::Event<
            ::std::sync::Arc<M>,
            M,
            LSSVMPairEvents,
        > {
            self.0.event_with_filter(::core::default::Default::default())
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for LSSVMPair<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Custom Error type `BondingCurveError` with signature `BondingCurveError(uint8)` and selector `0xe4f2c5ac`
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
    #[etherror(name = "BondingCurveError", abi = "BondingCurveError(uint8)")]
    pub struct BondingCurveError {
        pub error: u8,
    }
    ///Custom Error type `Ownable_NewOwnerZeroAddress` with signature `Ownable_NewOwnerZeroAddress()` and selector `0xedf1b1fc`
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
        name = "Ownable_NewOwnerZeroAddress",
        abi = "Ownable_NewOwnerZeroAddress()"
    )]
    pub struct Ownable_NewOwnerZeroAddress;
    ///Custom Error type `Ownable_NotOwner` with signature `Ownable_NotOwner()` and selector `0x5eee3ad1`
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
    #[etherror(name = "Ownable_NotOwner", abi = "Ownable_NotOwner()")]
    pub struct Ownable_NotOwner;
    ///Container type for all of the contract's custom errors
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LSSVMPairErrors {
        BondingCurveError(BondingCurveError),
        Ownable_NewOwnerZeroAddress(Ownable_NewOwnerZeroAddress),
        Ownable_NotOwner(Ownable_NotOwner),
        /// The standard solidity revert string, with selector
        /// Error(string) -- 0x08c379a0
        RevertString(::std::string::String),
    }
    impl ::ethers::core::abi::AbiDecode for LSSVMPairErrors {
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
                = <BondingCurveError as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BondingCurveError(decoded));
            }
            if let Ok(decoded)
                = <Ownable_NewOwnerZeroAddress as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::Ownable_NewOwnerZeroAddress(decoded));
            }
            if let Ok(decoded)
                = <Ownable_NotOwner as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Ownable_NotOwner(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LSSVMPairErrors {
        fn encode(self) -> ::std::vec::Vec<u8> {
            match self {
                Self::BondingCurveError(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ownable_NewOwnerZeroAddress(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Ownable_NotOwner(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::RevertString(s) => ::ethers::core::abi::AbiEncode::encode(s),
            }
        }
    }
    impl ::ethers::contract::ContractRevert for LSSVMPairErrors {
        fn valid_selector(selector: [u8; 4]) -> bool {
            match selector {
                [0x08, 0xc3, 0x79, 0xa0] => true,
                _ if selector
                    == <BondingCurveError as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Ownable_NewOwnerZeroAddress as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ if selector
                    == <Ownable_NotOwner as ::ethers::contract::EthError>::selector() => {
                    true
                }
                _ => false,
            }
        }
    }
    impl ::core::fmt::Display for LSSVMPairErrors {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::BondingCurveError(element) => ::core::fmt::Display::fmt(element, f),
                Self::Ownable_NewOwnerZeroAddress(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::Ownable_NotOwner(element) => ::core::fmt::Display::fmt(element, f),
                Self::RevertString(s) => ::core::fmt::Display::fmt(s, f),
            }
        }
    }
    impl ::core::convert::From<::std::string::String> for LSSVMPairErrors {
        fn from(value: String) -> Self {
            Self::RevertString(value)
        }
    }
    impl ::core::convert::From<BondingCurveError> for LSSVMPairErrors {
        fn from(value: BondingCurveError) -> Self {
            Self::BondingCurveError(value)
        }
    }
    impl ::core::convert::From<Ownable_NewOwnerZeroAddress> for LSSVMPairErrors {
        fn from(value: Ownable_NewOwnerZeroAddress) -> Self {
            Self::Ownable_NewOwnerZeroAddress(value)
        }
    }
    impl ::core::convert::From<Ownable_NotOwner> for LSSVMPairErrors {
        fn from(value: Ownable_NotOwner) -> Self {
            Self::Ownable_NotOwner(value)
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
    #[ethevent(name = "AssetRecipientChange", abi = "AssetRecipientChange(address)")]
    pub struct AssetRecipientChangeFilter {
        pub a: ::ethers::core::types::Address,
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
    #[ethevent(name = "DeltaUpdate", abi = "DeltaUpdate(uint128)")]
    pub struct DeltaUpdateFilter {
        pub new_delta: u128,
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
    #[ethevent(name = "FeeUpdate", abi = "FeeUpdate(uint96)")]
    pub struct FeeUpdateFilter {
        pub new_fee: u128,
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
    #[ethevent(name = "NFTWithdrawal", abi = "NFTWithdrawal()")]
    pub struct NftwithdrawalFilter;
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
    #[ethevent(name = "OwnershipTransferred", abi = "OwnershipTransferred(address)")]
    pub struct OwnershipTransferredFilter {
        #[ethevent(indexed)]
        pub new_owner: ::ethers::core::types::Address,
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
    #[ethevent(name = "SpotPriceUpdate", abi = "SpotPriceUpdate(uint128)")]
    pub struct SpotPriceUpdateFilter {
        pub new_spot_price: u128,
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
    #[ethevent(name = "SwapNFTInPair", abi = "SwapNFTInPair()")]
    pub struct SwapNFTInPairFilter;
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
    #[ethevent(name = "SwapNFTOutPair", abi = "SwapNFTOutPair()")]
    pub struct SwapNFTOutPairFilter;
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
    #[ethevent(name = "TokenDeposit", abi = "TokenDeposit(uint256)")]
    pub struct TokenDepositFilter {
        pub amount: ::ethers::core::types::U256,
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
    #[ethevent(name = "TokenWithdrawal", abi = "TokenWithdrawal(uint256)")]
    pub struct TokenWithdrawalFilter {
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all of the contract's events
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LSSVMPairEvents {
        AssetRecipientChangeFilter(AssetRecipientChangeFilter),
        DeltaUpdateFilter(DeltaUpdateFilter),
        FeeUpdateFilter(FeeUpdateFilter),
        NftwithdrawalFilter(NftwithdrawalFilter),
        OwnershipTransferredFilter(OwnershipTransferredFilter),
        SpotPriceUpdateFilter(SpotPriceUpdateFilter),
        SwapNFTInPairFilter(SwapNFTInPairFilter),
        SwapNFTOutPairFilter(SwapNFTOutPairFilter),
        TokenDepositFilter(TokenDepositFilter),
        TokenWithdrawalFilter(TokenWithdrawalFilter),
    }
    impl ::ethers::contract::EthLogDecode for LSSVMPairEvents {
        fn decode_log(
            log: &::ethers::core::abi::RawLog,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::Error> {
            if let Ok(decoded) = AssetRecipientChangeFilter::decode_log(log) {
                return Ok(LSSVMPairEvents::AssetRecipientChangeFilter(decoded));
            }
            if let Ok(decoded) = DeltaUpdateFilter::decode_log(log) {
                return Ok(LSSVMPairEvents::DeltaUpdateFilter(decoded));
            }
            if let Ok(decoded) = FeeUpdateFilter::decode_log(log) {
                return Ok(LSSVMPairEvents::FeeUpdateFilter(decoded));
            }
            if let Ok(decoded) = NftwithdrawalFilter::decode_log(log) {
                return Ok(LSSVMPairEvents::NftwithdrawalFilter(decoded));
            }
            if let Ok(decoded) = OwnershipTransferredFilter::decode_log(log) {
                return Ok(LSSVMPairEvents::OwnershipTransferredFilter(decoded));
            }
            if let Ok(decoded) = SpotPriceUpdateFilter::decode_log(log) {
                return Ok(LSSVMPairEvents::SpotPriceUpdateFilter(decoded));
            }
            if let Ok(decoded) = SwapNFTInPairFilter::decode_log(log) {
                return Ok(LSSVMPairEvents::SwapNFTInPairFilter(decoded));
            }
            if let Ok(decoded) = SwapNFTOutPairFilter::decode_log(log) {
                return Ok(LSSVMPairEvents::SwapNFTOutPairFilter(decoded));
            }
            if let Ok(decoded) = TokenDepositFilter::decode_log(log) {
                return Ok(LSSVMPairEvents::TokenDepositFilter(decoded));
            }
            if let Ok(decoded) = TokenWithdrawalFilter::decode_log(log) {
                return Ok(LSSVMPairEvents::TokenWithdrawalFilter(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData)
        }
    }
    impl ::core::fmt::Display for LSSVMPairEvents {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetRecipientChangeFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::DeltaUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::FeeUpdateFilter(element) => ::core::fmt::Display::fmt(element, f),
                Self::NftwithdrawalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OwnershipTransferredFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SpotPriceUpdateFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapNFTInPairFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapNFTOutPairFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenDepositFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TokenWithdrawalFilter(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
            }
        }
    }
    impl ::core::convert::From<AssetRecipientChangeFilter> for LSSVMPairEvents {
        fn from(value: AssetRecipientChangeFilter) -> Self {
            Self::AssetRecipientChangeFilter(value)
        }
    }
    impl ::core::convert::From<DeltaUpdateFilter> for LSSVMPairEvents {
        fn from(value: DeltaUpdateFilter) -> Self {
            Self::DeltaUpdateFilter(value)
        }
    }
    impl ::core::convert::From<FeeUpdateFilter> for LSSVMPairEvents {
        fn from(value: FeeUpdateFilter) -> Self {
            Self::FeeUpdateFilter(value)
        }
    }
    impl ::core::convert::From<NftwithdrawalFilter> for LSSVMPairEvents {
        fn from(value: NftwithdrawalFilter) -> Self {
            Self::NftwithdrawalFilter(value)
        }
    }
    impl ::core::convert::From<OwnershipTransferredFilter> for LSSVMPairEvents {
        fn from(value: OwnershipTransferredFilter) -> Self {
            Self::OwnershipTransferredFilter(value)
        }
    }
    impl ::core::convert::From<SpotPriceUpdateFilter> for LSSVMPairEvents {
        fn from(value: SpotPriceUpdateFilter) -> Self {
            Self::SpotPriceUpdateFilter(value)
        }
    }
    impl ::core::convert::From<SwapNFTInPairFilter> for LSSVMPairEvents {
        fn from(value: SwapNFTInPairFilter) -> Self {
            Self::SwapNFTInPairFilter(value)
        }
    }
    impl ::core::convert::From<SwapNFTOutPairFilter> for LSSVMPairEvents {
        fn from(value: SwapNFTOutPairFilter) -> Self {
            Self::SwapNFTOutPairFilter(value)
        }
    }
    impl ::core::convert::From<TokenDepositFilter> for LSSVMPairEvents {
        fn from(value: TokenDepositFilter) -> Self {
            Self::TokenDepositFilter(value)
        }
    }
    impl ::core::convert::From<TokenWithdrawalFilter> for LSSVMPairEvents {
        fn from(value: TokenWithdrawalFilter) -> Self {
            Self::TokenWithdrawalFilter(value)
        }
    }
    ///Container type for all input parameters for the `assetRecipient` function with signature `assetRecipient()` and selector `0x3bfa67fe`
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
    #[ethcall(name = "assetRecipient", abi = "assetRecipient()")]
    pub struct AssetRecipientCall;
    ///Container type for all input parameters for the `bondingCurve` function with signature `bondingCurve()` and selector `0xeff1d50e`
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
    #[ethcall(name = "bondingCurve", abi = "bondingCurve()")]
    pub struct BondingCurveCall;
    ///Container type for all input parameters for the `call` function with signature `call(address,bytes)` and selector `0x1b8b921d`
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
    #[ethcall(name = "call", abi = "call(address,bytes)")]
    pub struct CallCall {
        pub target: ::ethers::core::types::Address,
        pub data: ::ethers::core::types::Bytes,
    }
    ///Container type for all input parameters for the `changeAssetRecipient` function with signature `changeAssetRecipient(address)` and selector `0xf4629549`
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
    #[ethcall(name = "changeAssetRecipient", abi = "changeAssetRecipient(address)")]
    pub struct ChangeAssetRecipientCall {
        pub new_recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `changeDelta` function with signature `changeDelta(uint128)` and selector `0x6809f664`
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
    #[ethcall(name = "changeDelta", abi = "changeDelta(uint128)")]
    pub struct ChangeDeltaCall {
        pub new_delta: u128,
    }
    ///Container type for all input parameters for the `changeFee` function with signature `changeFee(uint96)` and selector `0x6b7a2200`
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
    #[ethcall(name = "changeFee", abi = "changeFee(uint96)")]
    pub struct ChangeFeeCall {
        pub new_fee: u128,
    }
    ///Container type for all input parameters for the `changeSpotPrice` function with signature `changeSpotPrice(uint128)` and selector `0xd8a1890c`
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
    #[ethcall(name = "changeSpotPrice", abi = "changeSpotPrice(uint128)")]
    pub struct ChangeSpotPriceCall {
        pub new_spot_price: u128,
    }
    ///Container type for all input parameters for the `delta` function with signature `delta()` and selector `0x12b495a8`
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
    #[ethcall(name = "delta", abi = "delta()")]
    pub struct DeltaCall;
    ///Container type for all input parameters for the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    #[ethcall(name = "factory", abi = "factory()")]
    pub struct FactoryCall;
    ///Container type for all input parameters for the `fee` function with signature `fee()` and selector `0xddca3f43`
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
    #[ethcall(name = "fee", abi = "fee()")]
    pub struct FeeCall;
    ///Container type for all input parameters for the `getAllHeldIds` function with signature `getAllHeldIds()` and selector `0x2f4fefaf`
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
    #[ethcall(name = "getAllHeldIds", abi = "getAllHeldIds()")]
    pub struct GetAllHeldIdsCall;
    ///Container type for all input parameters for the `getAssetRecipient` function with signature `getAssetRecipient()` and selector `0x79eac6c2`
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
    #[ethcall(name = "getAssetRecipient", abi = "getAssetRecipient()")]
    pub struct GetAssetRecipientCall;
    ///Container type for all input parameters for the `getBuyNFTQuote` function with signature `getBuyNFTQuote(uint256)` and selector `0xa5cb2b91`
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
    #[ethcall(name = "getBuyNFTQuote", abi = "getBuyNFTQuote(uint256)")]
    pub struct GetBuyNFTQuoteCall {
        pub num_nf_ts: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `getSellNFTQuote` function with signature `getSellNFTQuote(uint256)` and selector `0x0c295e56`
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
    #[ethcall(name = "getSellNFTQuote", abi = "getSellNFTQuote(uint256)")]
    pub struct GetSellNFTQuoteCall {
        pub num_nf_ts: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `initialize` function with signature `initialize(address,address,uint128,uint96,uint128)` and selector `0xfd17aef9`
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
        name = "initialize",
        abi = "initialize(address,address,uint128,uint96,uint128)"
    )]
    pub struct InitializeCall {
        pub owner: ::ethers::core::types::Address,
        pub asset_recipient: ::ethers::core::types::Address,
        pub delta: u128,
        pub fee: u128,
        pub spot_price: u128,
    }
    ///Container type for all input parameters for the `multicall` function with signature `multicall(bytes[],bool)` and selector `0x1e9701d4`
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
    #[ethcall(name = "multicall", abi = "multicall(bytes[],bool)")]
    pub struct MulticallCall {
        pub calls: ::std::vec::Vec<::ethers::core::types::Bytes>,
        pub revert_on_fail: bool,
    }
    ///Container type for all input parameters for the `nft` function with signature `nft()` and selector `0x47ccca02`
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
    #[ethcall(name = "nft", abi = "nft()")]
    pub struct NftCall;
    ///Container type for all input parameters for the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
        name = "onERC1155BatchReceived",
        abi = "onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)"
    )]
    pub struct OnERC1155BatchReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::std::vec::Vec<::ethers::core::types::U256>,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
        name = "onERC1155Received",
        abi = "onERC1155Received(address,address,uint256,uint256,bytes)"
    )]
    pub struct OnERC1155ReceivedCall(
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::Address,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::U256,
        pub ::ethers::core::types::Bytes,
    );
    ///Container type for all input parameters for the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    #[ethcall(name = "owner", abi = "owner()")]
    pub struct OwnerCall;
    ///Container type for all input parameters for the `pairVariant` function with signature `pairVariant()` and selector `0x3053fc58`
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
    #[ethcall(name = "pairVariant", abi = "pairVariant()")]
    pub struct PairVariantCall;
    ///Container type for all input parameters for the `poolType` function with signature `poolType()` and selector `0xb1dd61b6`
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
    #[ethcall(name = "poolType", abi = "poolType()")]
    pub struct PoolTypeCall;
    ///Container type for all input parameters for the `spotPrice` function with signature `spotPrice()` and selector `0x398482d8`
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
    #[ethcall(name = "spotPrice", abi = "spotPrice()")]
    pub struct SpotPriceCall;
    ///Container type for all input parameters for the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    #[ethcall(name = "supportsInterface", abi = "supportsInterface(bytes4)")]
    pub struct SupportsInterfaceCall {
        pub interface_id: [u8; 4],
    }
    ///Container type for all input parameters for the `swapNFTsForToken` function with signature `swapNFTsForToken(uint256[],uint256,address,bool,address)` and selector `0xb1d3f1c1`
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
        name = "swapNFTsForToken",
        abi = "swapNFTsForToken(uint256[],uint256,address,bool,address)"
    )]
    pub struct SwapNFTsForTokenCall {
        pub nft_ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub min_expected_token_output: ::ethers::core::types::U256,
        pub token_recipient: ::ethers::core::types::Address,
        pub is_router: bool,
        pub router_caller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `swapTokenForAnyNFTs` function with signature `swapTokenForAnyNFTs(uint256,uint256,address,bool,address)` and selector `0x28b8aee1`
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
        name = "swapTokenForAnyNFTs",
        abi = "swapTokenForAnyNFTs(uint256,uint256,address,bool,address)"
    )]
    pub struct SwapTokenForAnyNFTsCall {
        pub num_nf_ts: ::ethers::core::types::U256,
        pub max_expected_token_input: ::ethers::core::types::U256,
        pub nft_recipient: ::ethers::core::types::Address,
        pub is_router: bool,
        pub router_caller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `swapTokenForSpecificNFTs` function with signature `swapTokenForSpecificNFTs(uint256[],uint256,address,bool,address)` and selector `0x6d8b99f7`
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
        name = "swapTokenForSpecificNFTs",
        abi = "swapTokenForSpecificNFTs(uint256[],uint256,address,bool,address)"
    )]
    pub struct SwapTokenForSpecificNFTsCall {
        pub nft_ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub max_expected_token_input: ::ethers::core::types::U256,
        pub nft_recipient: ::ethers::core::types::Address,
        pub is_router: bool,
        pub router_caller: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `transferOwnership` function with signature `transferOwnership(address)` and selector `0xf2fde38b`
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
    #[ethcall(name = "transferOwnership", abi = "transferOwnership(address)")]
    pub struct TransferOwnershipCall {
        pub new_owner: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `withdrawERC1155` function with signature `withdrawERC1155(address,uint256[],uint256[])` and selector `0xa5ceac99`
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
        name = "withdrawERC1155",
        abi = "withdrawERC1155(address,uint256[],uint256[])"
    )]
    pub struct WithdrawERC1155Call {
        pub a: ::ethers::core::types::Address,
        pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
        pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all input parameters for the `withdrawERC20` function with signature `withdrawERC20(address,uint256)` and selector `0xa1db9782`
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
    #[ethcall(name = "withdrawERC20", abi = "withdrawERC20(address,uint256)")]
    pub struct WithdrawERC20Call {
        pub a: ::ethers::core::types::Address,
        pub amount: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `withdrawERC721` function with signature `withdrawERC721(address,uint256[])` and selector `0x13edab81`
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
    #[ethcall(name = "withdrawERC721", abi = "withdrawERC721(address,uint256[])")]
    pub struct WithdrawERC721Call {
        pub a: ::ethers::core::types::Address,
        pub nft_ids: ::std::vec::Vec<::ethers::core::types::U256>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType, Debug, PartialEq, Eq, Hash)]
    pub enum LSSVMPairCalls {
        AssetRecipient(AssetRecipientCall),
        BondingCurve(BondingCurveCall),
        Call(CallCall),
        ChangeAssetRecipient(ChangeAssetRecipientCall),
        ChangeDelta(ChangeDeltaCall),
        ChangeFee(ChangeFeeCall),
        ChangeSpotPrice(ChangeSpotPriceCall),
        Delta(DeltaCall),
        Factory(FactoryCall),
        Fee(FeeCall),
        GetAllHeldIds(GetAllHeldIdsCall),
        GetAssetRecipient(GetAssetRecipientCall),
        GetBuyNFTQuote(GetBuyNFTQuoteCall),
        GetSellNFTQuote(GetSellNFTQuoteCall),
        Initialize(InitializeCall),
        Multicall(MulticallCall),
        Nft(NftCall),
        OnERC1155BatchReceived(OnERC1155BatchReceivedCall),
        OnERC1155Received(OnERC1155ReceivedCall),
        Owner(OwnerCall),
        PairVariant(PairVariantCall),
        PoolType(PoolTypeCall),
        SpotPrice(SpotPriceCall),
        SupportsInterface(SupportsInterfaceCall),
        SwapNFTsForToken(SwapNFTsForTokenCall),
        SwapTokenForAnyNFTs(SwapTokenForAnyNFTsCall),
        SwapTokenForSpecificNFTs(SwapTokenForSpecificNFTsCall),
        TransferOwnership(TransferOwnershipCall),
        WithdrawERC1155(WithdrawERC1155Call),
        WithdrawERC20(WithdrawERC20Call),
        WithdrawERC721(WithdrawERC721Call),
    }
    impl ::ethers::core::abi::AbiDecode for LSSVMPairCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <AssetRecipientCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::AssetRecipient(decoded));
            }
            if let Ok(decoded)
                = <BondingCurveCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::BondingCurve(decoded));
            }
            if let Ok(decoded)
                = <CallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Call(decoded));
            }
            if let Ok(decoded)
                = <ChangeAssetRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::ChangeAssetRecipient(decoded));
            }
            if let Ok(decoded)
                = <ChangeDeltaCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChangeDelta(decoded));
            }
            if let Ok(decoded)
                = <ChangeFeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChangeFee(decoded));
            }
            if let Ok(decoded)
                = <ChangeSpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::ChangeSpotPrice(decoded));
            }
            if let Ok(decoded)
                = <DeltaCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Delta(decoded));
            }
            if let Ok(decoded)
                = <FactoryCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Factory(decoded));
            }
            if let Ok(decoded)
                = <FeeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Fee(decoded));
            }
            if let Ok(decoded)
                = <GetAllHeldIdsCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetAllHeldIds(decoded));
            }
            if let Ok(decoded)
                = <GetAssetRecipientCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::GetAssetRecipient(decoded));
            }
            if let Ok(decoded)
                = <GetBuyNFTQuoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetBuyNFTQuote(decoded));
            }
            if let Ok(decoded)
                = <GetSellNFTQuoteCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetSellNFTQuote(decoded));
            }
            if let Ok(decoded)
                = <InitializeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Initialize(decoded));
            }
            if let Ok(decoded)
                = <MulticallCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Multicall(decoded));
            }
            if let Ok(decoded)
                = <NftCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Nft(decoded));
            }
            if let Ok(decoded)
                = <OnERC1155BatchReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC1155BatchReceived(decoded));
            }
            if let Ok(decoded)
                = <OnERC1155ReceivedCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::OnERC1155Received(decoded));
            }
            if let Ok(decoded)
                = <OwnerCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Owner(decoded));
            }
            if let Ok(decoded)
                = <PairVariantCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PairVariant(decoded));
            }
            if let Ok(decoded)
                = <PoolTypeCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::PoolType(decoded));
            }
            if let Ok(decoded)
                = <SpotPriceCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::SpotPrice(decoded));
            }
            if let Ok(decoded)
                = <SupportsInterfaceCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SupportsInterface(decoded));
            }
            if let Ok(decoded)
                = <SwapNFTsForTokenCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapNFTsForToken(decoded));
            }
            if let Ok(decoded)
                = <SwapTokenForAnyNFTsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapTokenForAnyNFTs(decoded));
            }
            if let Ok(decoded)
                = <SwapTokenForSpecificNFTsCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::SwapTokenForSpecificNFTs(decoded));
            }
            if let Ok(decoded)
                = <TransferOwnershipCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::TransferOwnership(decoded));
            }
            if let Ok(decoded)
                = <WithdrawERC1155Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawERC1155(decoded));
            }
            if let Ok(decoded)
                = <WithdrawERC20Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawERC20(decoded));
            }
            if let Ok(decoded)
                = <WithdrawERC721Call as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::WithdrawERC721(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for LSSVMPairCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::AssetRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::BondingCurve(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Call(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::ChangeAssetRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeDelta(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeFee(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::ChangeSpotPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Delta(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Factory(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Fee(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::GetAllHeldIds(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetAssetRecipient(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetBuyNFTQuote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetSellNFTQuote(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Initialize(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Multicall(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Nft(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::OnERC1155BatchReceived(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::OnERC1155Received(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Owner(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::PairVariant(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::PoolType(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SpotPrice(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SupportsInterface(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapNFTsForToken(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapTokenForAnyNFTs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::SwapTokenForSpecificNFTs(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::TransferOwnership(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawERC1155(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawERC20(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::WithdrawERC721(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for LSSVMPairCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::AssetRecipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::BondingCurve(element) => ::core::fmt::Display::fmt(element, f),
                Self::Call(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeAssetRecipient(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::ChangeDelta(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeFee(element) => ::core::fmt::Display::fmt(element, f),
                Self::ChangeSpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::Delta(element) => ::core::fmt::Display::fmt(element, f),
                Self::Factory(element) => ::core::fmt::Display::fmt(element, f),
                Self::Fee(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAllHeldIds(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetAssetRecipient(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetBuyNFTQuote(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetSellNFTQuote(element) => ::core::fmt::Display::fmt(element, f),
                Self::Initialize(element) => ::core::fmt::Display::fmt(element, f),
                Self::Multicall(element) => ::core::fmt::Display::fmt(element, f),
                Self::Nft(element) => ::core::fmt::Display::fmt(element, f),
                Self::OnERC1155BatchReceived(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::OnERC1155Received(element) => ::core::fmt::Display::fmt(element, f),
                Self::Owner(element) => ::core::fmt::Display::fmt(element, f),
                Self::PairVariant(element) => ::core::fmt::Display::fmt(element, f),
                Self::PoolType(element) => ::core::fmt::Display::fmt(element, f),
                Self::SpotPrice(element) => ::core::fmt::Display::fmt(element, f),
                Self::SupportsInterface(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapNFTsForToken(element) => ::core::fmt::Display::fmt(element, f),
                Self::SwapTokenForAnyNFTs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::SwapTokenForSpecificNFTs(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::TransferOwnership(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawERC1155(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawERC20(element) => ::core::fmt::Display::fmt(element, f),
                Self::WithdrawERC721(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<AssetRecipientCall> for LSSVMPairCalls {
        fn from(value: AssetRecipientCall) -> Self {
            Self::AssetRecipient(value)
        }
    }
    impl ::core::convert::From<BondingCurveCall> for LSSVMPairCalls {
        fn from(value: BondingCurveCall) -> Self {
            Self::BondingCurve(value)
        }
    }
    impl ::core::convert::From<CallCall> for LSSVMPairCalls {
        fn from(value: CallCall) -> Self {
            Self::Call(value)
        }
    }
    impl ::core::convert::From<ChangeAssetRecipientCall> for LSSVMPairCalls {
        fn from(value: ChangeAssetRecipientCall) -> Self {
            Self::ChangeAssetRecipient(value)
        }
    }
    impl ::core::convert::From<ChangeDeltaCall> for LSSVMPairCalls {
        fn from(value: ChangeDeltaCall) -> Self {
            Self::ChangeDelta(value)
        }
    }
    impl ::core::convert::From<ChangeFeeCall> for LSSVMPairCalls {
        fn from(value: ChangeFeeCall) -> Self {
            Self::ChangeFee(value)
        }
    }
    impl ::core::convert::From<ChangeSpotPriceCall> for LSSVMPairCalls {
        fn from(value: ChangeSpotPriceCall) -> Self {
            Self::ChangeSpotPrice(value)
        }
    }
    impl ::core::convert::From<DeltaCall> for LSSVMPairCalls {
        fn from(value: DeltaCall) -> Self {
            Self::Delta(value)
        }
    }
    impl ::core::convert::From<FactoryCall> for LSSVMPairCalls {
        fn from(value: FactoryCall) -> Self {
            Self::Factory(value)
        }
    }
    impl ::core::convert::From<FeeCall> for LSSVMPairCalls {
        fn from(value: FeeCall) -> Self {
            Self::Fee(value)
        }
    }
    impl ::core::convert::From<GetAllHeldIdsCall> for LSSVMPairCalls {
        fn from(value: GetAllHeldIdsCall) -> Self {
            Self::GetAllHeldIds(value)
        }
    }
    impl ::core::convert::From<GetAssetRecipientCall> for LSSVMPairCalls {
        fn from(value: GetAssetRecipientCall) -> Self {
            Self::GetAssetRecipient(value)
        }
    }
    impl ::core::convert::From<GetBuyNFTQuoteCall> for LSSVMPairCalls {
        fn from(value: GetBuyNFTQuoteCall) -> Self {
            Self::GetBuyNFTQuote(value)
        }
    }
    impl ::core::convert::From<GetSellNFTQuoteCall> for LSSVMPairCalls {
        fn from(value: GetSellNFTQuoteCall) -> Self {
            Self::GetSellNFTQuote(value)
        }
    }
    impl ::core::convert::From<InitializeCall> for LSSVMPairCalls {
        fn from(value: InitializeCall) -> Self {
            Self::Initialize(value)
        }
    }
    impl ::core::convert::From<MulticallCall> for LSSVMPairCalls {
        fn from(value: MulticallCall) -> Self {
            Self::Multicall(value)
        }
    }
    impl ::core::convert::From<NftCall> for LSSVMPairCalls {
        fn from(value: NftCall) -> Self {
            Self::Nft(value)
        }
    }
    impl ::core::convert::From<OnERC1155BatchReceivedCall> for LSSVMPairCalls {
        fn from(value: OnERC1155BatchReceivedCall) -> Self {
            Self::OnERC1155BatchReceived(value)
        }
    }
    impl ::core::convert::From<OnERC1155ReceivedCall> for LSSVMPairCalls {
        fn from(value: OnERC1155ReceivedCall) -> Self {
            Self::OnERC1155Received(value)
        }
    }
    impl ::core::convert::From<OwnerCall> for LSSVMPairCalls {
        fn from(value: OwnerCall) -> Self {
            Self::Owner(value)
        }
    }
    impl ::core::convert::From<PairVariantCall> for LSSVMPairCalls {
        fn from(value: PairVariantCall) -> Self {
            Self::PairVariant(value)
        }
    }
    impl ::core::convert::From<PoolTypeCall> for LSSVMPairCalls {
        fn from(value: PoolTypeCall) -> Self {
            Self::PoolType(value)
        }
    }
    impl ::core::convert::From<SpotPriceCall> for LSSVMPairCalls {
        fn from(value: SpotPriceCall) -> Self {
            Self::SpotPrice(value)
        }
    }
    impl ::core::convert::From<SupportsInterfaceCall> for LSSVMPairCalls {
        fn from(value: SupportsInterfaceCall) -> Self {
            Self::SupportsInterface(value)
        }
    }
    impl ::core::convert::From<SwapNFTsForTokenCall> for LSSVMPairCalls {
        fn from(value: SwapNFTsForTokenCall) -> Self {
            Self::SwapNFTsForToken(value)
        }
    }
    impl ::core::convert::From<SwapTokenForAnyNFTsCall> for LSSVMPairCalls {
        fn from(value: SwapTokenForAnyNFTsCall) -> Self {
            Self::SwapTokenForAnyNFTs(value)
        }
    }
    impl ::core::convert::From<SwapTokenForSpecificNFTsCall> for LSSVMPairCalls {
        fn from(value: SwapTokenForSpecificNFTsCall) -> Self {
            Self::SwapTokenForSpecificNFTs(value)
        }
    }
    impl ::core::convert::From<TransferOwnershipCall> for LSSVMPairCalls {
        fn from(value: TransferOwnershipCall) -> Self {
            Self::TransferOwnership(value)
        }
    }
    impl ::core::convert::From<WithdrawERC1155Call> for LSSVMPairCalls {
        fn from(value: WithdrawERC1155Call) -> Self {
            Self::WithdrawERC1155(value)
        }
    }
    impl ::core::convert::From<WithdrawERC20Call> for LSSVMPairCalls {
        fn from(value: WithdrawERC20Call) -> Self {
            Self::WithdrawERC20(value)
        }
    }
    impl ::core::convert::From<WithdrawERC721Call> for LSSVMPairCalls {
        fn from(value: WithdrawERC721Call) -> Self {
            Self::WithdrawERC721(value)
        }
    }
    ///Container type for all return fields from the `assetRecipient` function with signature `assetRecipient()` and selector `0x3bfa67fe`
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
    pub struct AssetRecipientReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `bondingCurve` function with signature `bondingCurve()` and selector `0xeff1d50e`
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
    pub struct BondingCurveReturn {
        pub bonding_curve: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `delta` function with signature `delta()` and selector `0x12b495a8`
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
    pub struct DeltaReturn(pub u128);
    ///Container type for all return fields from the `factory` function with signature `factory()` and selector `0xc45a0155`
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
    pub struct FactoryReturn {
        pub factory: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `fee` function with signature `fee()` and selector `0xddca3f43`
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
    pub struct FeeReturn(pub u128);
    ///Container type for all return fields from the `getAllHeldIds` function with signature `getAllHeldIds()` and selector `0x2f4fefaf`
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
    pub struct GetAllHeldIdsReturn(pub ::std::vec::Vec<::ethers::core::types::U256>);
    ///Container type for all return fields from the `getAssetRecipient` function with signature `getAssetRecipient()` and selector `0x79eac6c2`
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
    pub struct GetAssetRecipientReturn {
        pub asset_recipient: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `getBuyNFTQuote` function with signature `getBuyNFTQuote(uint256)` and selector `0xa5cb2b91`
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
    pub struct GetBuyNFTQuoteReturn {
        pub error: u8,
        pub new_spot_price: ::ethers::core::types::U256,
        pub new_delta: ::ethers::core::types::U256,
        pub input_amount: ::ethers::core::types::U256,
        pub protocol_fee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getSellNFTQuote` function with signature `getSellNFTQuote(uint256)` and selector `0x0c295e56`
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
    pub struct GetSellNFTQuoteReturn {
        pub error: u8,
        pub new_spot_price: ::ethers::core::types::U256,
        pub new_delta: ::ethers::core::types::U256,
        pub output_amount: ::ethers::core::types::U256,
        pub protocol_fee: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `nft` function with signature `nft()` and selector `0x47ccca02`
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
    pub struct NftReturn {
        pub nft: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `onERC1155BatchReceived` function with signature `onERC1155BatchReceived(address,address,uint256[],uint256[],bytes)` and selector `0xbc197c81`
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
    pub struct OnERC1155BatchReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `onERC1155Received` function with signature `onERC1155Received(address,address,uint256,uint256,bytes)` and selector `0xf23a6e61`
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
    pub struct OnERC1155ReceivedReturn(pub [u8; 4]);
    ///Container type for all return fields from the `owner` function with signature `owner()` and selector `0x8da5cb5b`
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
    pub struct OwnerReturn(pub ::ethers::core::types::Address);
    ///Container type for all return fields from the `pairVariant` function with signature `pairVariant()` and selector `0x3053fc58`
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
    pub struct PairVariantReturn(pub u8);
    ///Container type for all return fields from the `poolType` function with signature `poolType()` and selector `0xb1dd61b6`
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
    pub struct PoolTypeReturn {
        pub pool_type: u8,
    }
    ///Container type for all return fields from the `spotPrice` function with signature `spotPrice()` and selector `0x398482d8`
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
    pub struct SpotPriceReturn(pub u128);
    ///Container type for all return fields from the `supportsInterface` function with signature `supportsInterface(bytes4)` and selector `0x01ffc9a7`
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
    pub struct SupportsInterfaceReturn(pub bool);
    ///Container type for all return fields from the `swapNFTsForToken` function with signature `swapNFTsForToken(uint256[],uint256,address,bool,address)` and selector `0xb1d3f1c1`
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
    pub struct SwapNFTsForTokenReturn {
        pub output_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `swapTokenForAnyNFTs` function with signature `swapTokenForAnyNFTs(uint256,uint256,address,bool,address)` and selector `0x28b8aee1`
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
    pub struct SwapTokenForAnyNFTsReturn {
        pub input_amount: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `swapTokenForSpecificNFTs` function with signature `swapTokenForSpecificNFTs(uint256[],uint256,address,bool,address)` and selector `0x6d8b99f7`
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
    pub struct SwapTokenForSpecificNFTsReturn {
        pub input_amount: ::ethers::core::types::U256,
    }
}
