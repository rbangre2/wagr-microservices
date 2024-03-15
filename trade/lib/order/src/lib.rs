#[derive(Debug, Clone)]
pub enum BidOrAsk {
    Bid,
    Ask,
}

#[derive(Debug, Clone)]
pub struct Order {
    pub size: f64,
    pub bid_or_ask: BidOrAsk,
}

impl Order {
    pub fn new(bid_or_ask: BidOrAsk, size: f64) -> Order {
        Order { bid_or_ask, size }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }
}
