pub use consideration_interface::*;
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
pub mod consideration_interface {
    pub use super::super::shared_types::*;
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"struct OrderComponents[]\",\"name\":\"orders\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"counter\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"cancel\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"cancelled\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct AdvancedOrder\",\"name\":\"advancedOrder\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"uint120\",\"name\":\"numerator\",\"type\":\"uint120\",\"components\":[]},{\"internalType\":\"uint120\",\"name\":\"denominator\",\"type\":\"uint120\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"extraData\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct CriteriaResolver[]\",\"name\":\"criteriaResolvers\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum Side\",\"name\":\"side\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"criteriaProof\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"fulfillerConduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"fulfillAdvancedOrder\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"fulfilled\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct AdvancedOrder[]\",\"name\":\"advancedOrders\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"uint120\",\"name\":\"numerator\",\"type\":\"uint120\",\"components\":[]},{\"internalType\":\"uint120\",\"name\":\"denominator\",\"type\":\"uint120\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"extraData\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct CriteriaResolver[]\",\"name\":\"criteriaResolvers\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum Side\",\"name\":\"side\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"criteriaProof\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"internalType\":\"struct FulfillmentComponent[][]\",\"name\":\"offerFulfillments\",\"type\":\"tuple[][]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct FulfillmentComponent[][]\",\"name\":\"considerationFulfillments\",\"type\":\"tuple[][]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"fulfillerConduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maximumFulfilled\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"fulfillAvailableAdvancedOrders\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"availableOrders\",\"type\":\"bool[]\",\"components\":[]},{\"internalType\":\"struct Execution[]\",\"name\":\"executions\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct ReceivedItem\",\"name\":\"item\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct Order[]\",\"name\":\"orders\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct FulfillmentComponent[][]\",\"name\":\"offerFulfillments\",\"type\":\"tuple[][]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct FulfillmentComponent[][]\",\"name\":\"considerationFulfillments\",\"type\":\"tuple[][]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"fulfillerConduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"maximumFulfilled\",\"type\":\"uint256\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"fulfillAvailableOrders\",\"outputs\":[{\"internalType\":\"bool[]\",\"name\":\"availableOrders\",\"type\":\"bool[]\",\"components\":[]},{\"internalType\":\"struct Execution[]\",\"name\":\"executions\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct ReceivedItem\",\"name\":\"item\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct BasicOrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"considerationToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"considerationIdentifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"considerationAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"offerToken\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"offerIdentifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"offerAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum BasicOrderType\",\"name\":\"basicOrderType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"offererConduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"fulfillerConduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalOriginalAdditionalRecipients\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"struct AdditionalRecipient[]\",\"name\":\"additionalRecipients\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"fulfillBasicOrder\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"fulfilled\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Order\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"bytes32\",\"name\":\"fulfillerConduitKey\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"fulfillOrder\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"fulfilled\",\"type\":\"bool\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getCounter\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"counter\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct OrderComponents\",\"name\":\"order\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"counter\",\"type\":\"uint256\",\"components\":[]}]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOrderHash\",\"outputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"orderHash\",\"type\":\"bytes32\",\"components\":[]}],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"getOrderStatus\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"isValidated\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"bool\",\"name\":\"isCancelled\",\"type\":\"bool\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalFilled\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalSize\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"incrementCounter\",\"outputs\":[{\"internalType\":\"uint256\",\"name\":\"newCounter\",\"type\":\"uint256\",\"components\":[]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"information\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"version\",\"type\":\"string\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"domainSeparator\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"conduitController\",\"type\":\"address\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct AdvancedOrder[]\",\"name\":\"orders\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"uint120\",\"name\":\"numerator\",\"type\":\"uint120\",\"components\":[]},{\"internalType\":\"uint120\",\"name\":\"denominator\",\"type\":\"uint120\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]},{\"internalType\":\"bytes\",\"name\":\"extraData\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct CriteriaResolver[]\",\"name\":\"criteriaResolvers\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"enum Side\",\"name\":\"side\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"index\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32[]\",\"name\":\"criteriaProof\",\"type\":\"bytes32[]\",\"components\":[]}]},{\"internalType\":\"struct Fulfillment[]\",\"name\":\"fulfillments\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct FulfillmentComponent[]\",\"name\":\"offerComponents\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct FulfillmentComponent[]\",\"name\":\"considerationComponents\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\",\"components\":[]}]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"matchAdvancedOrders\",\"outputs\":[{\"internalType\":\"struct Execution[]\",\"name\":\"executions\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct ReceivedItem\",\"name\":\"item\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]}]}]},{\"inputs\":[{\"internalType\":\"struct Order[]\",\"name\":\"orders\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]},{\"internalType\":\"struct Fulfillment[]\",\"name\":\"fulfillments\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct FulfillmentComponent[]\",\"name\":\"offerComponents\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct FulfillmentComponent[]\",\"name\":\"considerationComponents\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"uint256\",\"name\":\"orderIndex\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"itemIndex\",\"type\":\"uint256\",\"components\":[]}]}]}],\"stateMutability\":\"payable\",\"type\":\"function\",\"name\":\"matchOrders\",\"outputs\":[{\"internalType\":\"struct Execution[]\",\"name\":\"executions\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct ReceivedItem\",\"name\":\"item\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifier\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"amount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]}]}]},{\"inputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"name\",\"outputs\":[{\"internalType\":\"string\",\"name\":\"contractName\",\"type\":\"string\",\"components\":[]}]},{\"inputs\":[{\"internalType\":\"struct Order[]\",\"name\":\"orders\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"struct OrderParameters\",\"name\":\"parameters\",\"type\":\"tuple\",\"components\":[{\"internalType\":\"address\",\"name\":\"offerer\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"zone\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"struct OfferItem[]\",\"name\":\"offer\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"struct ConsiderationItem[]\",\"name\":\"consideration\",\"type\":\"tuple[]\",\"components\":[{\"internalType\":\"enum ItemType\",\"name\":\"itemType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"address\",\"name\":\"token\",\"type\":\"address\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"identifierOrCriteria\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endAmount\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"address payable\",\"name\":\"recipient\",\"type\":\"address\",\"components\":[]}]},{\"internalType\":\"enum OrderType\",\"name\":\"orderType\",\"type\":\"uint8\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"startTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"endTime\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"zoneHash\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"salt\",\"type\":\"uint256\",\"components\":[]},{\"internalType\":\"bytes32\",\"name\":\"conduitKey\",\"type\":\"bytes32\",\"components\":[]},{\"internalType\":\"uint256\",\"name\":\"totalOriginalConsiderationItems\",\"type\":\"uint256\",\"components\":[]}]},{\"internalType\":\"bytes\",\"name\":\"signature\",\"type\":\"bytes\",\"components\":[]}]}],\"stateMutability\":\"nonpayable\",\"type\":\"function\",\"name\":\"validate\",\"outputs\":[{\"internalType\":\"bool\",\"name\":\"validated\",\"type\":\"bool\",\"components\":[]}]}]";
    ///The parsed JSON ABI of the contract.
    pub static CONSIDERATIONINTERFACE_ABI: ::ethers::contract::Lazy<
        ::ethers::core::abi::Abi,
    > = ::ethers::contract::Lazy::new(|| {
        ::ethers::core::utils::__serde_json::from_str(__ABI)
            .expect("ABI is always valid")
    });
    pub struct ConsiderationInterface<M>(::ethers::contract::Contract<M>);
    impl<M> ::core::clone::Clone for ConsiderationInterface<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for ConsiderationInterface<M> {
        type Target = ::ethers::contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for ConsiderationInterface<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for ConsiderationInterface<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(ConsiderationInterface))
                .field(&self.address())
                .finish()
        }
    }
    impl<M: ::ethers::providers::Middleware> ConsiderationInterface<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers::core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers::contract::Contract::new(
                    address.into(),
                    CONSIDERATIONINTERFACE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `cancel` (0xfd9f1e10) function
        pub fn cancel(
            &self,
            orders: ::std::vec::Vec<OrderComponents>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([253, 159, 30, 16], orders)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fulfillAdvancedOrder` (0xe7acab24) function
        pub fn fulfill_advanced_order(
            &self,
            advanced_order: AdvancedOrder,
            criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
            fulfiller_conduit_key: [u8; 32],
            recipient: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash(
                    [231, 172, 171, 36],
                    (
                        advanced_order,
                        criteria_resolvers,
                        fulfiller_conduit_key,
                        recipient,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fulfillAvailableAdvancedOrders` (0x87201b41) function
        pub fn fulfill_available_advanced_orders(
            &self,
            advanced_orders: ::std::vec::Vec<AdvancedOrder>,
            criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
            offer_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
            consideration_fulfillments: ::std::vec::Vec<
                ::std::vec::Vec<FulfillmentComponent>,
            >,
            fulfiller_conduit_key: [u8; 32],
            recipient: ::ethers::core::types::Address,
            maximum_fulfilled: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<bool>, ::std::vec::Vec<Execution>),
        > {
            self.0
                .method_hash(
                    [135, 32, 27, 65],
                    (
                        advanced_orders,
                        criteria_resolvers,
                        offer_fulfillments,
                        consideration_fulfillments,
                        fulfiller_conduit_key,
                        recipient,
                        maximum_fulfilled,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fulfillAvailableOrders` (0xed98a574) function
        pub fn fulfill_available_orders(
            &self,
            orders: ::std::vec::Vec<Order>,
            offer_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
            consideration_fulfillments: ::std::vec::Vec<
                ::std::vec::Vec<FulfillmentComponent>,
            >,
            fulfiller_conduit_key: [u8; 32],
            maximum_fulfilled: ::ethers::core::types::U256,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::vec::Vec<bool>, ::std::vec::Vec<Execution>),
        > {
            self.0
                .method_hash(
                    [237, 152, 165, 116],
                    (
                        orders,
                        offer_fulfillments,
                        consideration_fulfillments,
                        fulfiller_conduit_key,
                        maximum_fulfilled,
                    ),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fulfillBasicOrder` (0xfb0f3ee1) function
        pub fn fulfill_basic_order(
            &self,
            parameters: BasicOrderParameters,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([251, 15, 62, 225], (parameters,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `fulfillOrder` (0xb3a34c4c) function
        pub fn fulfill_order(
            &self,
            order: Order,
            fulfiller_conduit_key: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([179, 163, 76, 76], (order, fulfiller_conduit_key))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getCounter` (0xf07ec373) function
        pub fn get_counter(
            &self,
            offerer: ::ethers::core::types::Address,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([240, 126, 195, 115], offerer)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOrderHash` (0x79df72bd) function
        pub fn get_order_hash(
            &self,
            order: OrderComponents,
        ) -> ::ethers::contract::builders::ContractCall<M, [u8; 32]> {
            self.0
                .method_hash([121, 223, 114, 189], (order,))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `getOrderStatus` (0x46423aa7) function
        pub fn get_order_status(
            &self,
            order_hash: [u8; 32],
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (bool, bool, ::ethers::core::types::U256, ::ethers::core::types::U256),
        > {
            self.0
                .method_hash([70, 66, 58, 167], order_hash)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `incrementCounter` (0x5b34b966) function
        pub fn increment_counter(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::ethers::core::types::U256> {
            self.0
                .method_hash([91, 52, 185, 102], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `information` (0xf47b7740) function
        pub fn information(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<
            M,
            (::std::string::String, [u8; 32], ::ethers::core::types::Address),
        > {
            self.0
                .method_hash([244, 123, 119, 64], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchAdvancedOrders` (0x55944a42) function
        pub fn match_advanced_orders(
            &self,
            orders: ::std::vec::Vec<AdvancedOrder>,
            criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
            fulfillments: ::std::vec::Vec<Fulfillment>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Execution>> {
            self.0
                .method_hash(
                    [85, 148, 74, 66],
                    (orders, criteria_resolvers, fulfillments),
                )
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `matchOrders` (0xa8174404) function
        pub fn match_orders(
            &self,
            orders: ::std::vec::Vec<Order>,
            fulfillments: ::std::vec::Vec<Fulfillment>,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::vec::Vec<Execution>> {
            self.0
                .method_hash([168, 23, 68, 4], (orders, fulfillments))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `name` (0x06fdde03) function
        pub fn name(
            &self,
        ) -> ::ethers::contract::builders::ContractCall<M, ::std::string::String> {
            self.0
                .method_hash([6, 253, 222, 3], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `validate` (0x88147732) function
        pub fn validate(
            &self,
            orders: ::std::vec::Vec<Order>,
        ) -> ::ethers::contract::builders::ContractCall<M, bool> {
            self.0
                .method_hash([136, 20, 119, 50], orders)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers::providers::Middleware> From<::ethers::contract::Contract<M>>
    for ConsiderationInterface<M> {
        fn from(contract: ::ethers::contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `cancel` function with signature `cancel((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)[])` and selector `0xfd9f1e10`
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
        name = "cancel",
        abi = "cancel((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)[])"
    )]
    pub struct CancelCall {
        pub orders: ::std::vec::Vec<OrderComponents>,
    }
    ///Container type for all input parameters for the `fulfillAdvancedOrder` function with signature `fulfillAdvancedOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes),(uint256,uint8,uint256,uint256,bytes32[])[],bytes32,address)` and selector `0xe7acab24`
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
        name = "fulfillAdvancedOrder",
        abi = "fulfillAdvancedOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes),(uint256,uint8,uint256,uint256,bytes32[])[],bytes32,address)"
    )]
    pub struct FulfillAdvancedOrderCall {
        pub advanced_order: AdvancedOrder,
        pub criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
        pub fulfiller_conduit_key: [u8; 32],
        pub recipient: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `fulfillAvailableAdvancedOrders` function with signature `fulfillAvailableAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,address,uint256)` and selector `0x87201b41`
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
        name = "fulfillAvailableAdvancedOrders",
        abi = "fulfillAvailableAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,address,uint256)"
    )]
    pub struct FulfillAvailableAdvancedOrdersCall {
        pub advanced_orders: ::std::vec::Vec<AdvancedOrder>,
        pub criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
        pub offer_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
        pub consideration_fulfillments: ::std::vec::Vec<
            ::std::vec::Vec<FulfillmentComponent>,
        >,
        pub fulfiller_conduit_key: [u8; 32],
        pub recipient: ::ethers::core::types::Address,
        pub maximum_fulfilled: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `fulfillAvailableOrders` function with signature `fulfillAvailableOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,uint256)` and selector `0xed98a574`
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
        name = "fulfillAvailableOrders",
        abi = "fulfillAvailableOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,uint256)"
    )]
    pub struct FulfillAvailableOrdersCall {
        pub orders: ::std::vec::Vec<Order>,
        pub offer_fulfillments: ::std::vec::Vec<::std::vec::Vec<FulfillmentComponent>>,
        pub consideration_fulfillments: ::std::vec::Vec<
            ::std::vec::Vec<FulfillmentComponent>,
        >,
        pub fulfiller_conduit_key: [u8; 32],
        pub maximum_fulfilled: ::ethers::core::types::U256,
    }
    ///Container type for all input parameters for the `fulfillBasicOrder` function with signature `fulfillBasicOrder((address,uint256,uint256,address,address,address,uint256,uint256,uint8,uint256,uint256,bytes32,uint256,bytes32,bytes32,uint256,(uint256,address)[],bytes))` and selector `0xfb0f3ee1`
    #[derive(Clone, ::ethers::contract::EthCall, ::ethers::contract::EthDisplay)]
    #[ethcall(
        name = "fulfillBasicOrder",
        abi = "fulfillBasicOrder((address,uint256,uint256,address,address,address,uint256,uint256,uint8,uint256,uint256,bytes32,uint256,bytes32,bytes32,uint256,(uint256,address)[],bytes))"
    )]
    pub struct FulfillBasicOrderCall {
        pub parameters: BasicOrderParameters,
    }
    ///Container type for all input parameters for the `fulfillOrder` function with signature `fulfillOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes),bytes32)` and selector `0xb3a34c4c`
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
        name = "fulfillOrder",
        abi = "fulfillOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes),bytes32)"
    )]
    pub struct FulfillOrderCall {
        pub order: Order,
        pub fulfiller_conduit_key: [u8; 32],
    }
    ///Container type for all input parameters for the `getCounter` function with signature `getCounter(address)` and selector `0xf07ec373`
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
    #[ethcall(name = "getCounter", abi = "getCounter(address)")]
    pub struct GetCounterCall {
        pub offerer: ::ethers::core::types::Address,
    }
    ///Container type for all input parameters for the `getOrderHash` function with signature `getOrderHash((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256))` and selector `0x79df72bd`
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
        name = "getOrderHash",
        abi = "getOrderHash((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256))"
    )]
    pub struct GetOrderHashCall {
        pub order: OrderComponents,
    }
    ///Container type for all input parameters for the `getOrderStatus` function with signature `getOrderStatus(bytes32)` and selector `0x46423aa7`
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
    #[ethcall(name = "getOrderStatus", abi = "getOrderStatus(bytes32)")]
    pub struct GetOrderStatusCall {
        pub order_hash: [u8; 32],
    }
    ///Container type for all input parameters for the `incrementCounter` function with signature `incrementCounter()` and selector `0x5b34b966`
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
    #[ethcall(name = "incrementCounter", abi = "incrementCounter()")]
    pub struct IncrementCounterCall;
    ///Container type for all input parameters for the `information` function with signature `information()` and selector `0xf47b7740`
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
    #[ethcall(name = "information", abi = "information()")]
    pub struct InformationCall;
    ///Container type for all input parameters for the `matchAdvancedOrders` function with signature `matchAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],((uint256,uint256)[],(uint256,uint256)[])[])` and selector `0x55944a42`
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
        name = "matchAdvancedOrders",
        abi = "matchAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],((uint256,uint256)[],(uint256,uint256)[])[])"
    )]
    pub struct MatchAdvancedOrdersCall {
        pub orders: ::std::vec::Vec<AdvancedOrder>,
        pub criteria_resolvers: ::std::vec::Vec<CriteriaResolver>,
        pub fulfillments: ::std::vec::Vec<Fulfillment>,
    }
    ///Container type for all input parameters for the `matchOrders` function with signature `matchOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],((uint256,uint256)[],(uint256,uint256)[])[])` and selector `0xa8174404`
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
        name = "matchOrders",
        abi = "matchOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],((uint256,uint256)[],(uint256,uint256)[])[])"
    )]
    pub struct MatchOrdersCall {
        pub orders: ::std::vec::Vec<Order>,
        pub fulfillments: ::std::vec::Vec<Fulfillment>,
    }
    ///Container type for all input parameters for the `name` function with signature `name()` and selector `0x06fdde03`
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
    #[ethcall(name = "name", abi = "name()")]
    pub struct NameCall;
    ///Container type for all input parameters for the `validate` function with signature `validate(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[])` and selector `0x88147732`
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
        name = "validate",
        abi = "validate(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[])"
    )]
    pub struct ValidateCall {
        pub orders: ::std::vec::Vec<Order>,
    }
    ///Container type for all of the contract's call
    #[derive(Clone, ::ethers::contract::EthAbiType)]
    pub enum ConsiderationInterfaceCalls {
        Cancel(CancelCall),
        FulfillAdvancedOrder(FulfillAdvancedOrderCall),
        FulfillAvailableAdvancedOrders(FulfillAvailableAdvancedOrdersCall),
        FulfillAvailableOrders(FulfillAvailableOrdersCall),
        FulfillBasicOrder(FulfillBasicOrderCall),
        FulfillOrder(FulfillOrderCall),
        GetCounter(GetCounterCall),
        GetOrderHash(GetOrderHashCall),
        GetOrderStatus(GetOrderStatusCall),
        IncrementCounter(IncrementCounterCall),
        Information(InformationCall),
        MatchAdvancedOrders(MatchAdvancedOrdersCall),
        MatchOrders(MatchOrdersCall),
        Name(NameCall),
        Validate(ValidateCall),
    }
    impl ::ethers::core::abi::AbiDecode for ConsiderationInterfaceCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers::core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <CancelCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Cancel(decoded));
            }
            if let Ok(decoded)
                = <FulfillAdvancedOrderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FulfillAdvancedOrder(decoded));
            }
            if let Ok(decoded)
                = <FulfillAvailableAdvancedOrdersCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FulfillAvailableAdvancedOrders(decoded));
            }
            if let Ok(decoded)
                = <FulfillAvailableOrdersCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FulfillAvailableOrders(decoded));
            }
            if let Ok(decoded)
                = <FulfillBasicOrderCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::FulfillBasicOrder(decoded));
            }
            if let Ok(decoded)
                = <FulfillOrderCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::FulfillOrder(decoded));
            }
            if let Ok(decoded)
                = <GetCounterCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetCounter(decoded));
            }
            if let Ok(decoded)
                = <GetOrderHashCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetOrderHash(decoded));
            }
            if let Ok(decoded)
                = <GetOrderStatusCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::GetOrderStatus(decoded));
            }
            if let Ok(decoded)
                = <IncrementCounterCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::IncrementCounter(decoded));
            }
            if let Ok(decoded)
                = <InformationCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Information(decoded));
            }
            if let Ok(decoded)
                = <MatchAdvancedOrdersCall as ::ethers::core::abi::AbiDecode>::decode(
                    data,
                ) {
                return Ok(Self::MatchAdvancedOrders(decoded));
            }
            if let Ok(decoded)
                = <MatchOrdersCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::MatchOrders(decoded));
            }
            if let Ok(decoded)
                = <NameCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Name(decoded));
            }
            if let Ok(decoded)
                = <ValidateCall as ::ethers::core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Validate(decoded));
            }
            Err(::ethers::core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers::core::abi::AbiEncode for ConsiderationInterfaceCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Cancel(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::FulfillAdvancedOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FulfillAvailableAdvancedOrders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FulfillAvailableOrders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FulfillBasicOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::FulfillOrder(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetCounter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOrderHash(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::GetOrderStatus(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::IncrementCounter(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Information(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MatchAdvancedOrders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::MatchOrders(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
                Self::Name(element) => ::ethers::core::abi::AbiEncode::encode(element),
                Self::Validate(element) => {
                    ::ethers::core::abi::AbiEncode::encode(element)
                }
            }
        }
    }
    impl ::core::fmt::Display for ConsiderationInterfaceCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Cancel(element) => ::core::fmt::Display::fmt(element, f),
                Self::FulfillAdvancedOrder(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FulfillAvailableAdvancedOrders(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FulfillAvailableOrders(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::FulfillBasicOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::FulfillOrder(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetCounter(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOrderHash(element) => ::core::fmt::Display::fmt(element, f),
                Self::GetOrderStatus(element) => ::core::fmt::Display::fmt(element, f),
                Self::IncrementCounter(element) => ::core::fmt::Display::fmt(element, f),
                Self::Information(element) => ::core::fmt::Display::fmt(element, f),
                Self::MatchAdvancedOrders(element) => {
                    ::core::fmt::Display::fmt(element, f)
                }
                Self::MatchOrders(element) => ::core::fmt::Display::fmt(element, f),
                Self::Name(element) => ::core::fmt::Display::fmt(element, f),
                Self::Validate(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<CancelCall> for ConsiderationInterfaceCalls {
        fn from(value: CancelCall) -> Self {
            Self::Cancel(value)
        }
    }
    impl ::core::convert::From<FulfillAdvancedOrderCall>
    for ConsiderationInterfaceCalls {
        fn from(value: FulfillAdvancedOrderCall) -> Self {
            Self::FulfillAdvancedOrder(value)
        }
    }
    impl ::core::convert::From<FulfillAvailableAdvancedOrdersCall>
    for ConsiderationInterfaceCalls {
        fn from(value: FulfillAvailableAdvancedOrdersCall) -> Self {
            Self::FulfillAvailableAdvancedOrders(value)
        }
    }
    impl ::core::convert::From<FulfillAvailableOrdersCall>
    for ConsiderationInterfaceCalls {
        fn from(value: FulfillAvailableOrdersCall) -> Self {
            Self::FulfillAvailableOrders(value)
        }
    }
    impl ::core::convert::From<FulfillBasicOrderCall> for ConsiderationInterfaceCalls {
        fn from(value: FulfillBasicOrderCall) -> Self {
            Self::FulfillBasicOrder(value)
        }
    }
    impl ::core::convert::From<FulfillOrderCall> for ConsiderationInterfaceCalls {
        fn from(value: FulfillOrderCall) -> Self {
            Self::FulfillOrder(value)
        }
    }
    impl ::core::convert::From<GetCounterCall> for ConsiderationInterfaceCalls {
        fn from(value: GetCounterCall) -> Self {
            Self::GetCounter(value)
        }
    }
    impl ::core::convert::From<GetOrderHashCall> for ConsiderationInterfaceCalls {
        fn from(value: GetOrderHashCall) -> Self {
            Self::GetOrderHash(value)
        }
    }
    impl ::core::convert::From<GetOrderStatusCall> for ConsiderationInterfaceCalls {
        fn from(value: GetOrderStatusCall) -> Self {
            Self::GetOrderStatus(value)
        }
    }
    impl ::core::convert::From<IncrementCounterCall> for ConsiderationInterfaceCalls {
        fn from(value: IncrementCounterCall) -> Self {
            Self::IncrementCounter(value)
        }
    }
    impl ::core::convert::From<InformationCall> for ConsiderationInterfaceCalls {
        fn from(value: InformationCall) -> Self {
            Self::Information(value)
        }
    }
    impl ::core::convert::From<MatchAdvancedOrdersCall> for ConsiderationInterfaceCalls {
        fn from(value: MatchAdvancedOrdersCall) -> Self {
            Self::MatchAdvancedOrders(value)
        }
    }
    impl ::core::convert::From<MatchOrdersCall> for ConsiderationInterfaceCalls {
        fn from(value: MatchOrdersCall) -> Self {
            Self::MatchOrders(value)
        }
    }
    impl ::core::convert::From<NameCall> for ConsiderationInterfaceCalls {
        fn from(value: NameCall) -> Self {
            Self::Name(value)
        }
    }
    impl ::core::convert::From<ValidateCall> for ConsiderationInterfaceCalls {
        fn from(value: ValidateCall) -> Self {
            Self::Validate(value)
        }
    }
    ///Container type for all return fields from the `cancel` function with signature `cancel((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256)[])` and selector `0xfd9f1e10`
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
    pub struct CancelReturn {
        pub cancelled: bool,
    }
    ///Container type for all return fields from the `fulfillAdvancedOrder` function with signature `fulfillAdvancedOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes),(uint256,uint8,uint256,uint256,bytes32[])[],bytes32,address)` and selector `0xe7acab24`
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
    pub struct FulfillAdvancedOrderReturn {
        pub fulfilled: bool,
    }
    ///Container type for all return fields from the `fulfillAvailableAdvancedOrders` function with signature `fulfillAvailableAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,address,uint256)` and selector `0x87201b41`
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
    pub struct FulfillAvailableAdvancedOrdersReturn {
        pub available_orders: ::std::vec::Vec<bool>,
        pub executions: ::std::vec::Vec<Execution>,
    }
    ///Container type for all return fields from the `fulfillAvailableOrders` function with signature `fulfillAvailableOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],(uint256,uint256)[][],(uint256,uint256)[][],bytes32,uint256)` and selector `0xed98a574`
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
    pub struct FulfillAvailableOrdersReturn {
        pub available_orders: ::std::vec::Vec<bool>,
        pub executions: ::std::vec::Vec<Execution>,
    }
    ///Container type for all return fields from the `fulfillBasicOrder` function with signature `fulfillBasicOrder((address,uint256,uint256,address,address,address,uint256,uint256,uint8,uint256,uint256,bytes32,uint256,bytes32,bytes32,uint256,(uint256,address)[],bytes))` and selector `0xfb0f3ee1`
    #[derive(Clone, ::ethers::contract::EthAbiType, ::ethers::contract::EthAbiCodec)]
    pub struct FulfillBasicOrderReturn {
        pub fulfilled: bool,
    }
    ///Container type for all return fields from the `fulfillOrder` function with signature `fulfillOrder(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes),bytes32)` and selector `0xb3a34c4c`
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
    pub struct FulfillOrderReturn {
        pub fulfilled: bool,
    }
    ///Container type for all return fields from the `getCounter` function with signature `getCounter(address)` and selector `0xf07ec373`
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
    pub struct GetCounterReturn {
        pub counter: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `getOrderHash` function with signature `getOrderHash((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256))` and selector `0x79df72bd`
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
    pub struct GetOrderHashReturn {
        pub order_hash: [u8; 32],
    }
    ///Container type for all return fields from the `getOrderStatus` function with signature `getOrderStatus(bytes32)` and selector `0x46423aa7`
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
    pub struct GetOrderStatusReturn {
        pub is_validated: bool,
        pub is_cancelled: bool,
        pub total_filled: ::ethers::core::types::U256,
        pub total_size: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `incrementCounter` function with signature `incrementCounter()` and selector `0x5b34b966`
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
    pub struct IncrementCounterReturn {
        pub new_counter: ::ethers::core::types::U256,
    }
    ///Container type for all return fields from the `information` function with signature `information()` and selector `0xf47b7740`
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
    pub struct InformationReturn {
        pub version: ::std::string::String,
        pub domain_separator: [u8; 32],
        pub conduit_controller: ::ethers::core::types::Address,
    }
    ///Container type for all return fields from the `matchAdvancedOrders` function with signature `matchAdvancedOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),uint120,uint120,bytes,bytes)[],(uint256,uint8,uint256,uint256,bytes32[])[],((uint256,uint256)[],(uint256,uint256)[])[])` and selector `0x55944a42`
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
    pub struct MatchAdvancedOrdersReturn {
        pub executions: ::std::vec::Vec<Execution>,
    }
    ///Container type for all return fields from the `matchOrders` function with signature `matchOrders(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[],((uint256,uint256)[],(uint256,uint256)[])[])` and selector `0xa8174404`
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
    pub struct MatchOrdersReturn {
        pub executions: ::std::vec::Vec<Execution>,
    }
    ///Container type for all return fields from the `name` function with signature `name()` and selector `0x06fdde03`
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
    pub struct NameReturn {
        pub contract_name: ::std::string::String,
    }
    ///Container type for all return fields from the `validate` function with signature `validate(((address,address,(uint8,address,uint256,uint256,uint256)[],(uint8,address,uint256,uint256,uint256,address)[],uint8,uint256,uint256,bytes32,uint256,bytes32,uint256),bytes)[])` and selector `0x88147732`
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
    pub struct ValidateReturn {
        pub validated: bool,
    }
}
