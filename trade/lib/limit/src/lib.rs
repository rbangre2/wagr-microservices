use order::Order;
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Limit {
    pub price: Decimal,
    pub orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: Decimal) -> Limit {
        Limit {
            price,
            orders: Vec::new(),
        }
    }

    pub fn total_volume(&self) -> f64 {
        self.orders
            .iter()
            .map(|order| order.size)
            .reduce(|a, b| a + b)
            .unwrap()
    }

    pub fn remove_filled_orders(&mut self) {
        self.orders.retain(|order| order.size > 0.0);
    }

    pub fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                }
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0
                }
            }

            if market_order.is_filled() {
                break;
            }
        }

        self.remove_filled_orders();
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order);
    }
}

// TESTS
#[cfg(test)]
pub mod tests {
    use super::*;
    use order::BidOrAsk;
    use rust_decimal_macros::dec;

    #[test]
    fn limit_order_single_fill() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);

        let buy_limit_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_sell_order = Order::new(BidOrAsk::Ask, 99.0);
        limit.fill_order(&mut market_sell_order);

        assert_eq!(market_sell_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 1.0);

        println!("{:?}", limit);
    }

    #[test]
    fn limit_order_multi_fill() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);

        // Add two orders of 100 units each
        let buy_limit_order_1 = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_2 = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_1);
        limit.add_order(buy_limit_order_2);

        // Market order to sell 199 units, which should fully fill the first order and partially fill the second
        let mut market_sell_order = Order::new(BidOrAsk::Ask, 199.0);
        limit.fill_order(&mut market_sell_order);

        // Verify the market sell order is fully filled
        assert!(
            market_sell_order.is_filled(),
            "Market sell order should be fully filled"
        );

        // Verify that only one order remains in the limit and it is partially filled
        assert_eq!(
            limit.orders.len(),
            1,
            "There should be only one order remaining in the limit"
        );
        assert_eq!(
            limit.orders[0].size, 1.0,
            "The remaining order should have 1.0 units left unfilled"
        );
    }

    #[test]
    fn limit_total_volume() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);

        let buy_limit_order_1 = Order::new(BidOrAsk::Bid, 100.0);
        let buy_limit_order_2 = Order::new(BidOrAsk::Bid, 100.0);

        limit.add_order(buy_limit_order_1);
        limit.add_order(buy_limit_order_2);

        assert_eq!(limit.total_volume(), 200.0);
    }

    #[test]
    fn limit_removes_filled_orders() {
        let price = dec!(10000);
        let mut limit = Limit::new(price);

        // Add two orders: one that will be completely filled, and one that will not
        let fully_filled_order = Order::new(BidOrAsk::Bid, 50.0);
        let partially_filled_order = Order::new(BidOrAsk::Bid, 100.0);
        limit.add_order(fully_filled_order);
        limit.add_order(partially_filled_order);

        // Create a market sell order that will fully fill the first limit order
        let mut market_sell_order = Order::new(BidOrAsk::Ask, 50.0);
        limit.fill_order(&mut market_sell_order);

        // Check that the fully filled order is removed
        assert_eq!(limit.orders.len(), 1);

        // Ensure the remaining order is the one that was partially filled (and not removed)
        assert_eq!(limit.orders[0].size, 100.0);
    }
}
