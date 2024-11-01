use std::time::SystemTime;
use super::price::Price;

#[derive(Debug, Clone)]
pub enum BidorAsk {
    Bid,
    Ask,
}

#[derive(Debug, Clone)]
pub struct Order {
    trader_id: String,
    price: Price,
    size: f64,
    bid_or_ask: BidorAsk,
    timestamp: SystemTime,
}

impl Order {
    pub fn new(trader_id: String, price: f64, bid_or_ask: BidorAsk, size: f64) -> Order {
        Order {
            trader_id: trader_id,
            price: Price::new(price),
            size: size,
            bid_or_ask: bid_or_ask,
            timestamp: SystemTime::now()
        }
    }

    pub fn is_filled(&self) -> bool {
        self.size == 0.0
    }

    pub fn get_size(&self) -> f64 {
        self.size
    }

    pub fn update_size(&mut self, new_size: f64) {
        self.size = new_size;
    }

    pub fn get_order_type(&self) -> BidorAsk {
        self.bid_or_ask.clone()
    }

    pub fn get_price(&self) -> Price {
        self.price.clone()
    }
}