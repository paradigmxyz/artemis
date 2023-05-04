// SPDX-License-Identifier: MIT
pragma solidity >=0.8.13;

// prettier-ignore
import {
    ConsiderationInterface
} from "../interfaces/ConsiderationInterface.sol";

// prettier-ignore
import {
    OrderComponents,
    BasicOrderParameters,
    OrderParameters,
    Order,
    AdvancedOrder,
    OrderStatus,
    CriteriaResolver,
    Fulfillment,
    FulfillmentComponent,
    Execution
} from "./ConsiderationStructs.sol";

import { OrderCombiner } from "./OrderCombiner.sol";

/**
 * @title Consideration
 * @author 0age
 * @custom:coauthor d1ll0n
 * @custom:coauthor transmissions11
 * @custom:version 1.1
 * @notice Consideration is a generalized ETH/ERC20/ERC721/ERC1155 marketplace.
 *         It minimizes external calls to the greatest extent possible and
 *         provides lightweight methods for common routes as well as more
 *         flexible methods for composing advanced orders or groups of orders.
 *         Each order contains an arbitrary number of items that may be spent
 *         (the "offer") along with an arbitrary number of items that must be
 *         received back by the indicated recipients (the "consideration").
 */
contract Consideration is ConsiderationInterface, OrderCombiner {
    /**
     * @notice Derive and set hashes, reference chainId, and associated domain
     *         separator during deployment.
     *
     * @param conduitController A contract that deploys conduits, or proxies
     *                          that may optionally be used to transfer approved
     *                          ERC20/721/1155 tokens.
     */
    constructor(address conduitController) OrderCombiner(conduitController) {}

    /**
     * @notice Fulfill an order offering an ERC20, ERC721, or ERC1155 item by
     *         supplying Ether (or other native tokens), ERC20 tokens, an ERC721
     *         item, or an ERC1155 item as consideration. Six permutations are
     *         supported: Native token to ERC721, Native token to ERC1155, ERC20
     *         to ERC721, ERC20 to ERC1155, ERC721 to ERC20, and ERC1155 to
     *         ERC20 (with native tokens supplied as msg.value). For an order to
     *         be eligible for fulfillment via this method, it must contain a
     *         single offer item (though that item may have a greater amount if
     *         the item is not an ERC721). An arbitrary number of "additional
     *         recipients" may also be supplied which will each receive native
     *         tokens or ERC20 items from the fulfiller as consideration. Refer
     *         to the documentation for a more comprehensive summary of how to
     *         utilize this method and what orders are compatible with it.
     *
     * @param parameters Additional information on the fulfilled order. Note
     *                   that the offerer and the fulfiller must first approve
     *                   this contract (or their chosen conduit if indicated)
     *                   before any tokens can be transferred. Also note that
     *                   contract recipients of ERC1155 consideration items must
     *                   implement `onERC1155Received` in order to receive those
     *                   items.
     *
     * @return fulfilled A boolean indicating whether the order has been
     *                   successfully fulfilled.
     */
    function fulfillBasicOrder(BasicOrderParameters calldata parameters)
        external
        payable
        override
        returns (bool fulfilled)
    {
        // Validate and fulfill the basic order.
        fulfilled = _validateAndFulfillBasicOrder(parameters);
    }

    /**
     * @notice Fulfill an order with an arbitrary number of items for offer and
     *         consideration. Note that this function does not support
     *         criteria-based orders or partial filling of orders (though
     *         filling the remainder of a partially-filled order is supported).
     *
     * @param order               The order to fulfill. Note that both the
     *                            offerer and the fulfiller must first approve
     *                            this contract (or the corresponding conduit if
     *                            indicated) to transfer any relevant tokens on
     *                            their behalf and that contracts must implement
     *                            `onERC1155Received` to receive ERC1155 tokens
     *                            as consideration.
     * @param fulfillerConduitKey A bytes32 value indicating what conduit, if
     *                            any, to source the fulfiller's token approvals
     *                            from. The zero hash signifies that no conduit
     *                            should be used (and direct approvals set on
     *                            Consideration).
     *
     * @return fulfilled A boolean indicating whether the order has been
     *                   successfully fulfilled.
     */
    function fulfillOrder(Order calldata order, bytes32 fulfillerConduitKey)
        external
        payable
        override
        returns (bool fulfilled)
    {
        // Convert order to "advanced" order, then validate and fulfill it.
        fulfilled = _validateAndFulfillAdvancedOrder(
            _convertOrderToAdvanced(order),
            new CriteriaResolver[](0), // No criteria resolvers supplied.
            fulfillerConduitKey,
            msg.sender
        );
    }

    /**
     * @notice Fill an order, fully or partially, with an arbitrary number of
     *         items for offer and consideration alongside criteria resolvers
     *         containing specific token identifiers and associated proofs.
     *
     * @param advancedOrder       The order to fulfill along with the fraction
     *                            of the order to attempt to fill. Note that
     *                            both the offerer and the fulfiller must first
     *                            approve this contract (or their conduit if
     *                            indicated by the order) to transfer any
     *                            relevant tokens on their behalf and that
     *                            contracts must implement `onERC1155Received`
     *                            to receive ERC1155 tokens as consideration.
     *                            Also note that all offer and consideration
     *                            components must have no remainder after
     *                            multiplication of the respective amount with
     *                            the supplied fraction for the partial fill to
     *                            be considered valid.
     * @param criteriaResolvers   An array where each element contains a
     *                            reference to a specific offer or
     *                            consideration, a token identifier, and a proof
     *                            that the supplied token identifier is
     *                            contained in the merkle root held by the item
     *                            in question's criteria element. Note that an
     *                            empty criteria indicates that any
     *                            (transferable) token identifier on the token
     *                            in question is valid and that no associated
     *                            proof needs to be supplied.
     * @param fulfillerConduitKey A bytes32 value indicating what conduit, if
     *                            any, to source the fulfiller's token approvals
     *                            from. The zero hash signifies that no conduit
     *                            should be used (and direct approvals set on
     *                            Consideration).
     * @param recipient           The intended recipient for all received items,
     *                            with `address(0)` indicating that the caller
     *                            should receive the items.
     *
     * @return fulfilled A boolean indicating whether the order has been
     *                   successfully fulfilled.
     */
    function fulfillAdvancedOrder(
        AdvancedOrder calldata advancedOrder,
        CriteriaResolver[] calldata criteriaResolvers,
        bytes32 fulfillerConduitKey,
        address recipient
    ) external payable override returns (bool fulfilled) {
        // Validate and fulfill the order.
        fulfilled = _validateAndFulfillAdvancedOrder(
            advancedOrder,
            criteriaResolvers,
            fulfillerConduitKey,
            recipient == address(0) ? msg.sender : recipient
        );
    }

    /**
     * @notice Attempt to fill a group of orders, each with an arbitrary number
     *         of items for offer and consideration. Any order that is not
     *         currently active, has already been fully filled, or has been
     *         cancelled will be omitted. Remaining offer and consideration
     *         items will then be aggregated where possible as indicated by the
     *         supplied offer and consideration component arrays and aggregated
     *         items will be transferred to the fulfiller or to each intended
     *         recipient, respectively. Note that a failing item transfer or an
     *         issue with order formatting will cause the entire batch to fail.
     *         Note that this function does not support criteria-based orders or
     *         partial filling of orders (though filling the remainder of a
     *         partially-filled order is supported).
     *
     * @param orders                    The orders to fulfill. Note that both
     *                                  the offerer and the fulfiller must first
     *                                  approve this contract (or the
     *                                  corresponding conduit if indicated) to
     *                                  transfer any relevant tokens on their
     *                                  behalf and that contracts must implement
     *                                  `onERC1155Received` to receive ERC1155
     *                                  tokens as consideration.
     * @param offerFulfillments         An array of FulfillmentComponent arrays
     *                                  indicating which offer items to attempt
     *                                  to aggregate when preparing executions.
     * @param considerationFulfillments An array of FulfillmentComponent arrays
     *                                  indicating which consideration items to
     *                                  attempt to aggregate when preparing
     *                                  executions.
     * @param fulfillerConduitKey       A bytes32 value indicating what conduit,
     *                                  if any, to source the fulfiller's token
     *                                  approvals from. The zero hash signifies
     *                                  that no conduit should be used (and
     *                                  direct approvals set on Consideration).
     * @param maximumFulfilled          The maximum number of orders to fulfill.
     *
     * @return availableOrders An array of booleans indicating if each order
     *                         with an index corresponding to the index of the
     *                         returned boolean was fulfillable or not.
     * @return executions      An array of elements indicating the sequence of
     *                         transfers performed as part of matching the given
     *                         orders.
     */
    function fulfillAvailableOrders(
        Order[] calldata orders,
        FulfillmentComponent[][] calldata offerFulfillments,
        FulfillmentComponent[][] calldata considerationFulfillments,
        bytes32 fulfillerConduitKey,
        uint256 maximumFulfilled
    )
        external
        payable
        override
        returns (bool[] memory availableOrders, Execution[] memory executions)
    {
        // Convert orders to "advanced" orders and fulfill all available orders.
        return
            _fulfillAvailableAdvancedOrders(
                _convertOrdersToAdvanced(orders), // Convert to advanced orders.
                new CriteriaResolver[](0), // No criteria resolvers supplied.
                offerFulfillments,
                considerationFulfillments,
                fulfillerConduitKey,
                msg.sender,
                maximumFulfilled
            );
    }

    /**
     * @notice Attempt to fill a group of orders, fully or partially, with an
     *         arbitrary number of items for offer and consideration per order
     *         alongside criteria resolvers containing specific token
     *         identifiers and associated proofs. Any order that is not
     *         currently active, has already been fully filled, or has been
     *         cancelled will be omitted. Remaining offer and consideration
     *         items will then be aggregated where possible as indicated by the
     *         supplied offer and consideration component arrays and aggregated
     *         items will be transferred to the fulfiller or to each intended
     *         recipient, respectively. Note that a failing item transfer or an
     *         issue with order formatting will cause the entire batch to fail.
     *
     * @param advancedOrders            The orders to fulfill along with the
     *                                  fraction of those orders to attempt to
     *                                  fill. Note that both the offerer and the
     *                                  fulfiller must first approve this
     *                                  contract (or their conduit if indicated
     *                                  by the order) to transfer any relevant
     *                                  tokens on their behalf and that
     *                                  contracts must implement
     *                                  `onERC1155Received` in order to receive
     *                                  ERC1155 tokens as consideration. Also
     *                                  note that all offer and consideration
     *                                  components must have no remainder after
     *                                  multiplication of the respective amount
     *                                  with the supplied fraction for an
     *                                  order's partial fill amount to be
     *                                  considered valid.
     * @param criteriaResolvers         An array where each element contains a
     *                                  reference to a specific offer or
     *                                  consideration, a token identifier, and a
     *                                  proof that the supplied token identifier
     *                                  is contained in the merkle root held by
     *                                  the item in question's criteria element.
     *                                  Note that an empty criteria indicates
     *                                  that any (transferable) token
     *                                  identifier on the token in question is
     *                                  valid and that no associated proof needs
     *                                  to be supplied.
     * @param offerFulfillments         An array of FulfillmentComponent arrays
     *                                  indicating which offer items to attempt
     *                                  to aggregate when preparing executions.
     * @param considerationFulfillments An array of FulfillmentComponent arrays
     *                                  indicating which consideration items to
     *                                  attempt to aggregate when preparing
     *                                  executions.
     * @param fulfillerConduitKey       A bytes32 value indicating what conduit,
     *                                  if any, to source the fulfiller's token
     *                                  approvals from. The zero hash signifies
     *                                  that no conduit should be used (and
     *                                  direct approvals set on Consideration).
     * @param recipient                 The intended recipient for all received
     *                                  items, with `address(0)` indicating that
     *                                  the caller should receive the items.
     * @param maximumFulfilled          The maximum number of orders to fulfill.
     *
     * @return availableOrders An array of booleans indicating if each order
     *                         with an index corresponding to the index of the
     *                         returned boolean was fulfillable or not.
     * @return executions      An array of elements indicating the sequence of
     *                         transfers performed as part of matching the given
     *                         orders.
     */
    function fulfillAvailableAdvancedOrders(
        AdvancedOrder[] memory advancedOrders,
        CriteriaResolver[] calldata criteriaResolvers,
        FulfillmentComponent[][] calldata offerFulfillments,
        FulfillmentComponent[][] calldata considerationFulfillments,
        bytes32 fulfillerConduitKey,
        address recipient,
        uint256 maximumFulfilled
    )
        external
        payable
        override
        returns (bool[] memory availableOrders, Execution[] memory executions)
    {
        // Fulfill all available orders.
        return
            _fulfillAvailableAdvancedOrders(
                advancedOrders,
                criteriaResolvers,
                offerFulfillments,
                considerationFulfillments,
                fulfillerConduitKey,
                recipient == address(0) ? msg.sender : recipient,
                maximumFulfilled
            );
    }

    /**
     * @notice Match an arbitrary number of orders, each with an arbitrary
     *         number of items for offer and consideration along with a set of
     *         fulfillments allocating offer components to consideration
     *         components. Note that this function does not support
     *         criteria-based or partial filling of orders (though filling the
     *         remainder of a partially-filled order is supported).
     *
     * @param orders            The orders to match. Note that both the offerer
     *                          and fulfiller on each order must first approve
     *                          this contract (or their conduit if indicated by
     *                          the order) to transfer any relevant tokens on
     *                          their behalf and each consideration recipient
     *                          must implement `onERC1155Received` in order to
     *                          receive ERC1155 tokens.
     * @param fulfillments      An array of elements allocating offer components
     *                          to consideration components. Note that each
     *                          consideration component must be fully met in
     *                          order for the match operation to be valid.
     *
     * @return executions An array of elements indicating the sequence of
     *                    transfers performed as part of matching the given
     *                    orders.
     */
    function matchOrders(
        Order[] calldata orders,
        Fulfillment[] calldata fulfillments
    ) external payable override returns (Execution[] memory executions) {
        // Convert to advanced, validate, and match orders using fulfillments.
        return
            _matchAdvancedOrders(
                _convertOrdersToAdvanced(orders),
                new CriteriaResolver[](0), // No criteria resolvers supplied.
                fulfillments
            );
    }

    /**
     * @notice Match an arbitrary number of full or partial orders, each with an
     *         arbitrary number of items for offer and consideration, supplying
     *         criteria resolvers containing specific token identifiers and
     *         associated proofs as well as fulfillments allocating offer
     *         components to consideration components.
     *
     * @param advancedOrders    The advanced orders to match. Note that both the
     *                          offerer and fulfiller on each order must first
     *                          approve this contract (or their conduit if
     *                          indicated by the order) to transfer any relevant
     *                          tokens on their behalf and each consideration
     *                          recipient must implement `onERC1155Received` in
     *                          order to receive ERC1155 tokens. Also note that
     *                          the offer and consideration components for each
     *                          order must have no remainder after multiplying
     *                          the respective amount with the supplied fraction
     *                          in order for the group of partial fills to be
     *                          considered valid.
     * @param criteriaResolvers An array where each element contains a reference
     *                          to a specific order as well as that order's
     *                          offer or consideration, a token identifier, and
     *                          a proof that the supplied token identifier is
     *                          contained in the order's merkle root. Note that
     *                          an empty root indicates that any (transferable)
     *                          token identifier is valid and that no associated
     *                          proof needs to be supplied.
     * @param fulfillments      An array of elements allocating offer components
     *                          to consideration components. Note that each
     *                          consideration component must be fully met in
     *                          order for the match operation to be valid.
     *
     * @return executions An array of elements indicating the sequence of
     *                    transfers performed as part of matching the given
     *                    orders.
     */
    function matchAdvancedOrders(
        AdvancedOrder[] memory advancedOrders,
        CriteriaResolver[] calldata criteriaResolvers,
        Fulfillment[] calldata fulfillments
    ) external payable override returns (Execution[] memory executions) {
        // Validate and match the advanced orders using supplied fulfillments.
        return
            _matchAdvancedOrders(
                advancedOrders,
                criteriaResolvers,
                fulfillments
            );
    }

    /**
     * @notice Cancel an arbitrary number of orders. Note that only the offerer
     *         or the zone of a given order may cancel it. Callers should ensure
     *         that the intended order was cancelled by calling `getOrderStatus`
     *         and confirming that `isCancelled` returns `true`.
     *
     * @param orders The orders to cancel.
     *
     * @return cancelled A boolean indicating whether the supplied orders have
     *                   been successfully cancelled.
     */
    function cancel(OrderComponents[] calldata orders)
        external
        override
        returns (bool cancelled)
    {
        // Cancel the orders.
        cancelled = _cancel(orders);
    }

    /**
     * @notice Validate an arbitrary number of orders, thereby registering their
     *         signatures as valid and allowing the fulfiller to skip signature
     *         verification on fulfillment. Note that validated orders may still
     *         be unfulfillable due to invalid item amounts or other factors;
     *         callers should determine whether validated orders are fulfillable
     *         by simulating the fulfillment call prior to execution. Also note
     *         that anyone can validate a signed order, but only the offerer can
     *         validate an order without supplying a signature.
     *
     * @param orders The orders to validate.
     *
     * @return validated A boolean indicating whether the supplied orders have
     *                   been successfully validated.
     */
    function validate(Order[] calldata orders)
        external
        override
        returns (bool validated)
    {
        // Validate the orders.
        validated = _validate(orders);
    }

    /**
     * @notice Cancel all orders from a given offerer with a given zone in bulk
     *         by incrementing a counter. Note that only the offerer may
     *         increment the counter.
     *
     * @return newCounter The new counter.
     */
    function incrementCounter() external override returns (uint256 newCounter) {
        // Increment current counter for the supplied offerer.
        newCounter = _incrementCounter();
    }

    /**
     * @notice Retrieve the order hash for a given order.
     *
     * @param order The components of the order.
     *
     * @return orderHash The order hash.
     */
    function getOrderHash(OrderComponents calldata order)
        external
        view
        override
        returns (bytes32 orderHash)
    {
        // Derive order hash by supplying order parameters along with counter.
        orderHash = _deriveOrderHash(
            OrderParameters(
                order.offerer,
                order.zone,
                order.offer,
                order.consideration,
                order.orderType,
                order.startTime,
                order.endTime,
                order.zoneHash,
                order.salt,
                order.conduitKey,
                order.consideration.length
            ),
            order.counter
        );
    }

    /**
     * @notice Retrieve the status of a given order by hash, including whether
     *         the order has been cancelled or validated and the fraction of the
     *         order that has been filled.
     *
     * @param orderHash The order hash in question.
     *
     * @return isValidated A boolean indicating whether the order in question
     *                     has been validated (i.e. previously approved or
     *                     partially filled).
     * @return isCancelled A boolean indicating whether the order in question
     *                     has been cancelled.
     * @return totalFilled The total portion of the order that has been filled
     *                     (i.e. the "numerator").
     * @return totalSize   The total size of the order that is either filled or
     *                     unfilled (i.e. the "denominator").
     */
    function getOrderStatus(bytes32 orderHash)
        external
        view
        override
        returns (
            bool isValidated,
            bool isCancelled,
            uint256 totalFilled,
            uint256 totalSize
        )
    {
        // Retrieve the order status using the order hash.
        return _getOrderStatus(orderHash);
    }

    /**
     * @notice Retrieve the current counter for a given offerer.
     *
     * @param offerer The offerer in question.
     *
     * @return counter The current counter.
     */
    function getCounter(address offerer)
        external
        view
        override
        returns (uint256 counter)
    {
        // Return the counter for the supplied offerer.
        counter = _getCounter(offerer);
    }

    /**
     * @notice Retrieve configuration information for this contract.
     *
     * @return version           The contract version.
     * @return domainSeparator   The domain separator for this contract.
     * @return conduitController The conduit Controller set for this contract.
     */
    function information()
        external
        view
        override
        returns (
            string memory version,
            bytes32 domainSeparator,
            address conduitController
        )
    {
        // Return the information for this contract.
        return _information();
    }

    /**
     * @notice Retrieve the name of this contract.
     *
     * @return contractName The name of this contract.
     */
    function name()
        external
        pure
        override
        returns (string memory contractName)
    {
        // Return the name of the contract.
        contractName = _name();
    }
}
