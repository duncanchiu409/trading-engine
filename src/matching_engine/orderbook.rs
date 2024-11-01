use std::collections::HashMap;
use super::order::{BidorAsk, Order};
use super::price::Price;
use super::limit::Limit;

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Price, Limit>,
    asks_sorted_keys: Vec<Price>,
    bids: HashMap<Price, Limit>,
    bids_sorted_keys: Vec<Price>,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            asks_sorted_keys: Vec::new(),
            bids: HashMap::new(),
            bids_sorted_keys: Vec::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        let price = order.get_price();
        let limit = match order.get_order_type() {
            BidorAsk::Bid => self.bids.get_mut(&price),
            BidorAsk::Ask => self.asks.get_mut(&price),
        };
        match limit {
            Some(limit) => limit.add_order(order),
            None => {
                let mut limit = Limit::new(price.clone());
                limit.add_order(order.clone());
                match order.get_order_type() {
                    BidorAsk::Bid => {
                        self.bids_sorted_keys.push(price);
                        self.bids.insert(price, limit).unwrap()
                    },
                    BidorAsk::Ask => self.asks.insert(price, limit),
                };
            }
        };
    }

    pub fn fill_market_order(&mut self, market_order: &mut Order) {
        let prices = match market_order.get_order_type() {
            BidorAsk::Bid => self.asks_sorted_keys.clone(),
            BidorAsk::Ask => self.bids_sorted_keys.clone(),
        };
        for price in prices {
            let limit = match market_order.get_order_type() {
                BidorAsk::Bid => self.asks.get_mut(&price),
                BidorAsk::Ask => self.bids.get_mut(&price),
            };
            match limit {
                Some(limit) => limit.fill_order(market_order),
                None => panic!("Key {:?} in Vec but not in Hashmap", price)
            };
            if market_order.is_filled() {
                break;
            }
        }
    }

    pub fn fill_limit_order(&mut self, market_order: &mut Order) {
        let price = market_order.get_price();
        let limit = match market_order.get_order_type() {
            BidorAsk::Bid => self.asks.get_mut(&price),
            BidorAsk::Ask => self.bids.get_mut(&price),
        };
        match limit {
            Some(limit) => {
                limit.fill_order(market_order);
                if market_order.is_filled() {
                    return
                }
            },
            None => {},
        }
        self.add_order(market_order.clone());
    }

    pub fn remove_price_level(&mut self, price: f64, bid_or_ask: BidorAsk) -> Option<Limit> {
        let price = Price::new(price);
        match bid_or_ask {
            BidorAsk::Bid => self.bids.remove(&price),
            BidorAsk::Ask => self.asks.remove(&price),
        }
    }

    pub fn get_price_level(&self, price: f64, bid_or_ask: BidorAsk) -> Option<&Limit> {
        let price = Price::new(price);
        match bid_or_ask {
            BidorAsk::Bid => self.bids.get(&price),
            BidorAsk::Ask => self.asks.get(&price),
        }
    }
}