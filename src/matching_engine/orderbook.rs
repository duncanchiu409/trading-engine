use std::collections::HashMap;

#[derive(Debug)]
pub enum BidorAsk {
    Bid,
    Ask,
}

#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidorAsk,
}

impl Order {
    pub fn new(bid_or_ask: BidorAsk, size: f64) -> Order {
        Order {
            bid_or_ask: bid_or_ask,
            size: size,
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
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64,
}

impl Price {
    pub fn new(price: f64) -> Price {
        let scalar = 1000000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            scalar,
            fractional,
            integral,
        }
    }

    pub fn to_f64(&self) -> f64 {
        self.integral as f64 + (self.fractional as f64 / self.scalar as f64)
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>,
}

impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price: price,
            orders: Vec::new(),
        }
    }

    fn total_volume(&self) -> f64 {
        let volume = self.orders.iter().map(|order| order.size).reduce(|a, b| a + b).unwrap();
        volume
    }

    fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.size >= limit_order.size {
                true => {
                    market_order.size -= limit_order.size;
                    limit_order.size = 0.0;
                },
                false => {
                    limit_order.size -= market_order.size;
                    market_order.size = 0.0;
                },
            }

            if market_order.is_filled() {
                break;
            }
        }

        // Clean up filled orders
        self.orders.retain(|order| !order.is_filled());
    }

    fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }

    fn remove_order(&mut self, index: usize) -> Option<Order> {
        if index < self.orders.len() {
            Some(self.orders.remove(index))
        } else {
            None
        }
    }

    fn get_order(&self, index: usize) -> Option<&Order> {
        self.orders.get(index)
    }

    fn get_order_mut(&mut self, index: usize) -> Option<&mut Order> {
        self.orders.get_mut(index)
    }

    fn get_price(&self) -> Price {
        self.price
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn limit_total_volume() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order_a = Order::new(BidorAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new(BidorAsk::Bid, 99.0);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        assert_eq!(limit.total_volume(), 199.0);
    }

    #[test]
    fn limit_order_single_fill() {
        let price = Price::new(100000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order = Order::new(BidorAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_order = Order::new(BidorAsk::Ask, 80.0);
        limit.fill_order(&mut market_order);

        assert_eq!(market_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 20.0);
    }

    #[test]
    fn limit_order_multi_fill() {
        let price = Price::new(100000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order_a = Order::new(BidorAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_a);
        let buy_limit_order_b = Order::new(BidorAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_b);

        let mut market_order = Order::new(BidorAsk::Ask, 180.0);
        limit.fill_order(&mut market_order);

        assert_eq!(market_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().size, 0.0);
        assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
        assert_eq!(limit.orders.get(1).unwrap().size, 20.0);
    }
}

#[derive(Debug)]
pub struct Orderbook {
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>,
}

impl Orderbook {
    pub fn new() -> Orderbook {
        Orderbook {
            asks: HashMap::new(),
            bids: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, price: f64, order: Order) {
        let price = Price::new(price);
        match order.bid_or_ask {
            BidorAsk::Bid => match self.bids.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.bids.insert(price, limit);
                }
            },
            BidorAsk::Ask => match self.asks.get_mut(&price) {
                Some(limit) => {
                    limit.add_order(order);
                }
                None => {
                    let mut limit = Limit::new(price);
                    limit.add_order(order);
                    self.asks.insert(price, limit);
                }
            },
        }
    }

    pub fn get_asks(&self) -> &HashMap<Price, Limit> {
        &self.asks
    }

    pub fn get_bids(&self) -> &HashMap<Price, Limit> {
        &self.bids
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