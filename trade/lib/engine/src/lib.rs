use order::Order;
use orderbook::OrderBook;
use rust_decimal::Decimal;
use std::collections::HashMap;
use trade_pair::TradingPair;

pub struct MatchingEngine {
    pub orderbooks: HashMap<TradingPair, OrderBook>,
}

impl MatchingEngine {
    pub fn new() -> MatchingEngine {
        MatchingEngine {
            orderbooks: HashMap::new(),
        }
    }

    pub fn add_new_market(&mut self, pair: TradingPair) {
        self.orderbooks.insert(pair.clone(), OrderBook::new());
        println!("opening new orderbook for market {:?}", pair.to_string());
    }

    pub fn place_limit_order(
        &mut self,
        pair: TradingPair,
        price: Decimal,
        order: Order,
    ) -> Result<(), String> {
        match self.orderbooks.get_mut(&pair) {
            Some(orderbook) => {
                orderbook.add_limit_order(price, order);
                println!("placed limit order at price level {}", price);
                Ok(())
            }
            None => Err(format!(
                "the orderbook for the given trading pair ({}) does not exist",
                pair.to_string()
            )),
        }
    }

    pub fn place_market_order(
        &mut self,
        pair: TradingPair,
        mut order: Order,
    ) -> Result<(), String> {
        if let Some(orderbook) = self.orderbooks.get_mut(&pair) {
            // Directly attempt to fill the market order without adding it to the order book.
            // This assumes `fill_market_order` tries to match the order with existing opposite orders.
            orderbook.fill_market_order(&mut order);

            // After attempting to fill, check if the order is fully filled.
            if order.is_filled() {
                println!("Market order fully filled");
                Ok(())
            } else {
                // partially filled order does not complete execution
                println!("Market order partially filled or not filled at all");
                Ok(())
            }
        } else {
            Err(format!(
                "The orderbook for the given trading pair ({}) does not exist",
                pair.to_string()
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use order::{BidOrAsk, Order};
    use rust_decimal_macros::dec;
    use trade_pair::TradingPair;

    #[test]
    fn create_market() {
        let mut engine = MatchingEngine::new();
        let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
        engine.add_new_market(pair.clone());

        assert!(engine.orderbooks.contains_key(&pair));
    }

    #[test]
    fn place_limit_order_in_existing_market() {
        let mut engine = MatchingEngine::new();
        let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
        engine.add_new_market(pair.clone());

        let order = Order::new(BidOrAsk::Bid, 1.0);
        let result = engine.place_limit_order(pair.clone(), dec!(10000), order);

        assert!(result.is_ok());
        assert!(engine
            .orderbooks
            .get(&pair)
            .unwrap()
            .bids
            .contains_key(&dec!(10000)));
    }

    #[test]
    fn place_limit_order_in_non_existing_market() {
        let mut engine = MatchingEngine::new();
        let pair = TradingPair::new("BTC".to_string(), "USD".to_string());

        let order = Order::new(BidOrAsk::Bid, 1.0);
        let result = engine.place_limit_order(pair.clone(), dec!(10000), order);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "the orderbook for the given trading pair (BTC_USD) does not exist"
        );
    }

    #[test]
    fn place_market_order_fully_filled() {
        // Create a new MatchingEngine
        let mut engine = MatchingEngine::new();

        // Create a trading pair
        let pair = TradingPair::new("BTC".to_string(), "USD".to_string());

        // Add the trading pair to the engine
        engine.add_new_market(pair.clone());

        // Mock an order book with some existing limit orders
        let mut orderbook = OrderBook::new();
        orderbook.add_limit_order(dec!(10000), Order::new(BidOrAsk::Bid, 1.0));
        orderbook.add_limit_order(dec!(10001), Order::new(BidOrAsk::Bid, 1.0));

        // Insert the mock order book into the engine for the trading pair
        engine.orderbooks.insert(pair.clone(), orderbook);

        // Create a market order that should fully fill against existing limit orders
        let market_order = Order::new(BidOrAsk::Ask, 2.0);

        // Place the market order
        let result = engine.place_market_order(pair.clone(), market_order);

        // Assert that the market order was fully filled
        assert!(result.is_ok());

        // Check that the order book is now empty
        assert!(engine.orderbooks.get(&pair).unwrap().asks.is_empty());
    }

    #[test]
    fn place_market_order_partially_filled() {
        // Create a new MatchingEngine
        let mut engine = MatchingEngine::new();

        // Create a trading pair
        let pair = TradingPair::new("BTC".to_string(), "USD".to_string());

        // Add the trading pair to the engine
        engine.add_new_market(pair.clone());

        // Mock an order book with some existing limit orders
        let mut orderbook = OrderBook::new();
        orderbook.add_limit_order(dec!(10000), Order::new(BidOrAsk::Bid, 1.0));
        orderbook.add_limit_order(dec!(10001), Order::new(BidOrAsk::Bid, 1.0));

        // Insert the mock order book into the engine for the trading pair
        engine.orderbooks.insert(pair.clone(), orderbook);

        // Create a market order that should partially fill against existing limit orders
        let market_order = Order::new(BidOrAsk::Ask, 1.5);

        // Place the market order
        let result = engine.place_market_order(pair.clone(), market_order.clone());

        // Assert that the market order was partially filled
        assert!(result.is_ok());

        println!("{:?}", engine.orderbooks.get(&pair).unwrap());
        // Check that the order book still has some remaining limit orders
        assert!(engine.orderbooks.get(&pair).unwrap().asks.is_empty());
    }
}
