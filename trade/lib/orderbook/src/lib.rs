#![allow(dead_code)]
use limit::Limit;
use order::BidOrAsk;
use order::Order;
use rust_decimal::Decimal;
use std::collections::HashMap;

#[derive(Debug)]
pub struct OrderBook {
    pub asks: HashMap<Decimal, Limit>,
    pub bids: HashMap<Decimal, Limit>,
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn ask_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.asks.values_mut().collect::<Vec<&mut Limit>>();

        limits.sort_by(|a, b| a.price.cmp(&b.price));

        limits
    }

    pub fn bid_limits(&mut self) -> Vec<&mut Limit> {
        let mut limits = self.bids.values_mut().collect::<Vec<&mut Limit>>();

        limits.sort_by(|a, b| b.price.cmp(&a.price));

        limits
    }

    pub fn remove_empty_limits(&mut self) {
        self.asks.retain(|_, limit| !limit.orders.is_empty());
        self.bids.retain(|_, limit| !limit.orders.is_empty());
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let limits = match market_order.bid_or_ask {
            BidOrAsk::Bid => self.ask_limits(),
            BidOrAsk::Ask => self.bid_limits(),
        };

        for limit_order in limits {
            limit_order.fill_order(market_order);

            if market_order.is_filled() {
                break;
            }
        }
        match market_order.bid_or_ask {
            BidOrAsk::Bid => {}
            BidOrAsk::Ask => {}
        }

        self.remove_empty_limits();
    }

    pub fn add_limit_order(&mut self, price: Decimal, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Bid => match self.bids.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
            BidOrAsk::Ask => match self.asks.get_mut(&price) {
                Some(limit) => limit.add_order(order),
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
        }
    }
}

// TESTS
#[cfg(test)]
pub mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn orderbook_fill_market_order_asks() {
        let mut orderbook = OrderBook::new();
        orderbook.add_limit_order(dec!(500), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Ask, 10.0)); // This order should be matched and removed
        orderbook.add_limit_order(dec!(200), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(300), Order::new(BidOrAsk::Ask, 10.0));

        // Market order to match the ask at price 100
        let mut market_order = Order::new(BidOrAsk::Bid, 10.0);
        orderbook.fill_market_order(&mut market_order);

        // Assert market order is fully filled
        assert!(
            market_order.is_filled(),
            "Market order should be fully filled"
        );

        // After filling, the ask at price 100 should either be absent or empty
        if let Some(limit) = orderbook.asks.get(&dec!(100)) {
            assert!(
                limit.orders.is_empty(),
                "Limit at price 100 should be empty after being matched"
            );
        } else {
            // If the limit itself is removed, it's also a valid state
            assert!(true, "Limit at price 100 correctly removed from OrderBook");
        }

        // Verify the OrderBook still contains other limits
        assert!(
            !orderbook.asks.is_empty(),
            "OrderBook should still contain other asks"
        );
        assert!(
            orderbook.asks.contains_key(&dec!(200)),
            "OrderBook should have an ask at price 200"
        );
        assert!(
            orderbook.asks.contains_key(&dec!(300)),
            "OrderBook should have an ask at price 300"
        );
        assert!(
            orderbook.asks.contains_key(&dec!(500)),
            "OrderBook should have an ask at price 500"
        );
    }

    #[test]
    fn orderbook_no_orders_fill_market_order() {
        let mut orderbook = OrderBook::new();
        let mut market_order = Order::new(BidOrAsk::Bid, 10.0);
        orderbook.fill_market_order(&mut market_order);

        assert!(!market_order.is_filled());
    }

    #[test]
    fn orderbook_limit_order_sorting() {
        let mut orderbook = OrderBook::new();
        orderbook.add_limit_order(dec!(200), Order::new(BidOrAsk::Ask, 5.0));
        orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Ask, 5.0));

        let ask_limits = orderbook.ask_limits();
        assert_eq!(ask_limits[0].price, dec!(100));
        assert_eq!(ask_limits[1].price, dec!(200));
    }

    #[test]
    fn orderbook_partial_fill_market_order() {
        let mut orderbook = OrderBook::new();
        orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Ask, 5.0));

        let mut market_order = Order::new(BidOrAsk::Bid, 10.0);
        orderbook.fill_market_order(&mut market_order);

        assert!(!market_order.is_filled());
        assert_eq!(market_order.size, 5.0);
    }

    #[test]
    fn orderbook_multiple_fills_across_levels() {
        let mut orderbook = OrderBook::new();
        orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Ask, 5.0));
        orderbook.add_limit_order(dec!(110), Order::new(BidOrAsk::Ask, 10.0));

        let mut market_order = Order::new(BidOrAsk::Bid, 12.0);
        orderbook.fill_market_order(&mut market_order);

        // This should be true if the market order is expected to be fully filled
        assert!(market_order.is_filled());

        // Since the market order was for 12.0 and there was 15.0 available, it should be filled
        // and the remaining size in the order book at the price of 110 should be 3.0
        let remaining_limit_order = orderbook
            .asks
            .get(&dec!(110))
            .unwrap()
            .orders
            .get(0)
            .unwrap();
        assert_eq!(remaining_limit_order.size, 3.0);
    }

    #[test]
    fn orderbook_insertion_at_same_price() {
        let mut orderbook = OrderBook::new();
        let price = dec!(100);
        orderbook.add_limit_order(price, Order::new(BidOrAsk::Ask, 5.0));
        orderbook.add_limit_order(price, Order::new(BidOrAsk::Ask, 10.0));

        assert_eq!(orderbook.asks.get(&price).unwrap().orders.len(), 2);
    }

    #[test]
    fn orderbook_removes_empty_orders_after_filling() {
        let mut orderbook = OrderBook::new();

        // Add ask orders
        orderbook.add_limit_order(dec!(100), Order::new(BidOrAsk::Ask, 10.0));
        orderbook.add_limit_order(dec!(150), Order::new(BidOrAsk::Ask, 5.0));

        // Add bid orders to be matched
        let mut market_order_bid = Order::new(BidOrAsk::Bid, 10.0); // This order will match and remove the first ask order
        orderbook.fill_market_order(&mut market_order_bid);

        // Assert that the first ask order is filled and removed
        assert!(!orderbook.asks.contains_key(&dec!(100)));

        // Add more test steps as needed to verify behavior

        // Assert that other orders are still present
        assert!(orderbook.asks.contains_key(&dec!(150)));
        assert_eq!(orderbook.asks.get(&dec!(150)).unwrap().orders[0].size, 5.0);

        // Optionally, test removal of bid orders in a similar way
    }
}
