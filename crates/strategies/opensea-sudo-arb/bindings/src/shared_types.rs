///`AdditionalRecipient(uint256,address)`
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
pub struct AdditionalRecipient {
    pub amount: ::ethers::core::types::U256,
    pub recipient: ::ethers::core::types::Address,
}
///`AdvancedOrder((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)`
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
pub struct AdvancedOrder {
    pub parameters: OrderParameters,
    pub numerator: u128,
    pub denominator: u128,
    pub signature: ::ethers::core::types::Bytes,
    pub extra_data: ::ethers::core::types::Bytes,
}
///`BasicOrderParameters(address,uint256,uint256,address,address,address,uint256,uint256,uint8,uint256,uint256,bytes32,uint256,bytes32,bytes32,uint256,(uint256,address)[],bytes)`
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
pub struct BasicOrderParameters {
    pub consideration_token: ::ethers::core::types::Address,
    pub consideration_identifier: ::ethers::core::types::U256,
    pub consideration_amount: ::ethers::core::types::U256,
    pub offerer: ::ethers::core::types::Address,
    pub zone: ::ethers::core::types::Address,
    pub offer_token: ::ethers::core::types::Address,
    pub offer_identifier: ::ethers::core::types::U256,
    pub offer_amount: ::ethers::core::types::U256,
    pub basic_order_type: u8,
    pub start_time: ::ethers::core::types::U256,
    pub end_time: ::ethers::core::types::U256,
    pub zone_hash: [u8; 32],
    pub salt: ::ethers::core::types::U256,
    pub offerer_conduit_key: [u8; 32],
    pub fulfiller_conduit_key: [u8; 32],
    pub total_original_additional_recipients: ::ethers::core::types::U256,
    pub additional_recipients: ::std::vec::Vec<AdditionalRecipient>,
    pub signature: ::ethers::core::types::Bytes,
}
///`ConduitBatch1155Transfer(address,address,address,uint256[],uint256[])`
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
pub struct ConduitBatch1155Transfer {
    pub token: ::ethers::core::types::Address,
    pub from: ::ethers::core::types::Address,
    pub to: ::ethers::core::types::Address,
    pub ids: ::std::vec::Vec<::ethers::core::types::U256>,
    pub amounts: ::std::vec::Vec<::ethers::core::types::U256>,
}
///`ConduitTransfer(uint8,address,address,address,uint256,uint256)`
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
pub struct ConduitTransfer {
    pub item_type: u8,
    pub token: ::ethers::core::types::Address,
    pub from: ::ethers::core::types::Address,
    pub to: ::ethers::core::types::Address,
    pub identifier: ::ethers::core::types::U256,
    pub amount: ::ethers::core::types::U256,
}
///`ConsiderationItem(uint8,address,uint256,uint256,uint256,address)`
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
pub struct ConsiderationItem {
    pub item_type: u8,
    pub token: ::ethers::core::types::Address,
    pub identifier_or_criteria: ::ethers::core::types::U256,
    pub start_amount: ::ethers::core::types::U256,
    pub end_amount: ::ethers::core::types::U256,
    pub recipient: ::ethers::core::types::Address,
}
///`CriteriaResolver(uint256,uint8,uint256,uint256,bytes32[])`
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
pub struct CriteriaResolver {
    pub order_index: ::ethers::core::types::U256,
    pub side: u8,
    pub index: ::ethers::core::types::U256,
    pub identifier: ::ethers::core::types::U256,
    pub criteria_proof: ::std::vec::Vec<[u8; 32]>,
}
///`Execution((uint8,address,uint256,uint256,address),address,bytes32)`
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
pub struct Execution {
    pub item: ReceivedItem,
    pub offerer: ::ethers::core::types::Address,
    pub conduit_key: [u8; 32],
}
///`Fulfillment((uint256,uint256)[],(uint256,uint256)[])`
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
pub struct Fulfillment {
    pub offer_components: ::std::vec::Vec<FulfillmentComponent>,
    pub consideration_components: ::std::vec::Vec<FulfillmentComponent>,
}
///`FulfillmentComponent(uint256,uint256)`
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
pub struct FulfillmentComponent {
    pub order_index: ::ethers::core::types::U256,
    pub item_index: ::ethers::core::types::U256,
}
///`NftsForAnyNFTsTrade((address,uint256[])[],(address,uint256)[])`
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
pub struct NftsForAnyNFTsTrade {
    pub nft_to_token_trades: ::std::vec::Vec<PairSwapSpecific>,
    pub token_to_nft_trades: ::std::vec::Vec<PairSwapAny>,
}
///`NftsForSpecificNFTsTrade((address,uint256[])[],(address,uint256[])[])`
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
pub struct NftsForSpecificNFTsTrade {
    pub nft_to_token_trades: ::std::vec::Vec<PairSwapSpecific>,
    pub token_to_nft_trades: ::std::vec::Vec<PairSwapSpecific>,
}
///`PairSwapAny(address,uint256)`
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
pub struct PairSwapAny {
    pub pair: ::ethers::core::types::Address,
    pub num_items: ::ethers::core::types::U256,
}
///`PairSwapSpecific(address,uint256[])`
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
pub struct PairSwapSpecific {
    pub pair: ::ethers::core::types::Address,
    pub nft_ids: ::std::vec::Vec<::ethers::core::types::U256>,
}
///`RobustPairNFTsFoTokenAndTokenforNFTsTrade(((address,uint256[]),uint256)[],((address,uint256[]),uint256)[],uint256,address,address)`
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
pub struct RobustPairNFTsFoTokenAndTokenforNFTsTrade {
    pub token_to_nft_trades: ::std::vec::Vec<RobustPairSwapSpecific>,
    pub nft_to_token_trades: ::std::vec::Vec<RobustPairSwapSpecificForToken>,
    pub input_amount: ::ethers::core::types::U256,
    pub token_recipient: ::ethers::core::types::Address,
    pub nft_recipient: ::ethers::core::types::Address,
}
///`RobustPairSwapAny((address,uint256),uint256)`
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
pub struct RobustPairSwapAny {
    pub swap_info: PairSwapAny,
    pub max_cost: ::ethers::core::types::U256,
}
///`RobustPairSwapSpecific((address,uint256[]),uint256)`
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
pub struct RobustPairSwapSpecific {
    pub swap_info: PairSwapSpecific,
    pub max_cost: ::ethers::core::types::U256,
}
///`RobustPairSwapSpecificForToken((address,uint256[]),uint256)`
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
pub struct RobustPairSwapSpecificForToken {
    pub swap_info: PairSwapSpecific,
    pub min_output: ::ethers::core::types::U256,
}
///`OfferItem(uint8,address,uint256,uint256,uint256)`
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
pub struct OfferItem {
    pub item_type: u8,
    pub token: ::ethers::core::types::Address,
    pub identifier_or_criteria: ::ethers::core::types::U256,
    pub start_amount: ::ethers::core::types::U256,
    pub end_amount: ::ethers::core::types::U256,
}
///`Order((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)`
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
pub struct Order {
    pub parameters: OrderParameters,
    pub signature: ::ethers::core::types::Bytes,
}
///`OrderComponents(address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)`
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
pub struct OrderComponents {
    pub offerer: ::ethers::core::types::Address,
    pub zone: ::ethers::core::types::Address,
    pub offer: ::std::vec::Vec<OfferItem>,
    pub consideration: ::std::vec::Vec<ConsiderationItem>,
    pub order_type: u8,
    pub start_time: ::ethers::core::types::U256,
    pub end_time: ::ethers::core::types::U256,
    pub zone_hash: [u8; 32],
    pub salt: ::ethers::core::types::U256,
    pub conduit_key: [u8; 32],
    pub counter: ::ethers::core::types::U256,
}
///`OrderParameters(address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)`
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
pub struct OrderParameters {
    pub offerer: ::ethers::core::types::Address,
    pub zone: ::ethers::core::types::Address,
    pub offer: ::std::vec::Vec<OfferItem>,
    pub consideration: ::std::vec::Vec<ConsiderationItem>,
    pub order_type: u8,
    pub start_time: ::ethers::core::types::U256,
    pub end_time: ::ethers::core::types::U256,
    pub zone_hash: [u8; 32],
    pub salt: ::ethers::core::types::U256,
    pub conduit_key: [u8; 32],
    pub total_original_consideration_items: ::ethers::core::types::U256,
}
///`ReceivedItem(uint8,address,uint256,uint256,address)`
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
pub struct ReceivedItem {
    pub item_type: u8,
    pub token: ::ethers::core::types::Address,
    pub identifier: ::ethers::core::types::U256,
    pub amount: ::ethers::core::types::U256,
    pub recipient: ::ethers::core::types::Address,
}
///`SpentItem(uint8,address,uint256,uint256)`
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
pub struct SpentItem {
    pub item_type: u8,
    pub token: ::ethers::core::types::Address,
    pub identifier: ::ethers::core::types::U256,
    pub amount: ::ethers::core::types::U256,
}
///`TransferHelperItem(uint8,address,uint256,uint256)`
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
pub struct TransferHelperItem {
    pub item_type: u8,
    pub token: ::ethers::core::types::Address,
    pub identifier: ::ethers::core::types::U256,
    pub amount: ::ethers::core::types::U256,
}
