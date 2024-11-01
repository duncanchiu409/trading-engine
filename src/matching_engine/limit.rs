use std::collections::VecDeque;
use super::price::Price;
use super::order::{Order, BidorAsk};

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: VecDeque<Order>,
}

impl Limit {
    pub fn new(price: Price) -> Limit {
        Limit {
            price: price,
            orders: VecDeque::new(),
        }
    }

    fn total_volume(&self) -> f64 {
        let volume = self.orders.iter().map(|order| order.get_size()).reduce(|a, b| a + b).unwrap();
        volume
    }

    pub fn fill_order(&mut self, market_order: &mut Order) {
        for limit_order in self.orders.iter_mut() {
            match market_order.get_size() >= limit_order.get_size() {
                true => {
                    market_order.update_size(market_order.get_size() - limit_order.get_size());
                    limit_order.update_size(0.0);
                },
                false => {
                    limit_order.update_size(limit_order.get_size() - market_order.get_size());
                    market_order.update_size(0.0);
                },
            }

            if market_order.is_filled() {
                break;
            }
        }

        // Clean up filled orders
        self.orders.retain(|order| !order.is_filled());
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.push_back(order);
    }

    fn remove_order(&mut self, index: usize) -> Option<Order> {
        self.orders.remove(index)
    }

    fn get_order(&self, index: usize) -> Option<&Order> {
        self.orders.get(index)
    }

    fn get_order_mut(&mut self, index: usize) -> Option<&mut Order> {
        self.orders.get_mut(index)
    }

    fn get_price(&self) -> Price {
        self.price.clone()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn limit_total_volume() {
        let price = Price::new(10000.0);
        let mut limit = Limit::new(price);
        let buy_limit_order_a = Order::new("Trader a".to_string(), 5.5, BidorAsk::Bid, 100.0);
        let buy_limit_order_b = Order::new("Trader b".to_string(), 5.5, BidorAsk::Bid, 99.0);
        limit.add_order(buy_limit_order_a);
        limit.add_order(buy_limit_order_b);

        assert_eq!(limit.total_volume(), 199.0);
    }

    #[test]
    fn limit_order_single_fill() {
        let price = Price::new(100000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order = Order::new("Trader a".to_string(), 5.5,BidorAsk::Bid, 100.0);
        limit.add_order(buy_limit_order);

        let mut market_order = Order::new("Trader a".to_string(), 5.5,BidorAsk::Ask, 80.0);
        limit.fill_order(&mut market_order);

        assert_eq!(market_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().get_size(), 20.0);
    }

    #[test]
    fn limit_order_multi_fill() {
        let price = Price::new(100000.0);
        let mut limit = Limit::new(price);

        let buy_limit_order_a = Order::new("Trader a".to_string(), 5.5,BidorAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_a);
        let buy_limit_order_b = Order::new("Trader a".to_string(), 5.5,BidorAsk::Bid, 100.0);
        limit.add_order(buy_limit_order_b);

        let mut market_order = Order::new("Trader a".to_string(), 5.5,BidorAsk::Ask, 180.0);
        limit.fill_order(&mut market_order);

        assert_eq!(market_order.is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().is_filled(), true);
        assert_eq!(limit.orders.get(0).unwrap().get_size(), 0.0);
        assert_eq!(limit.orders.get(1).unwrap().is_filled(), false);
        assert_eq!(limit.orders.get(1).unwrap().get_size(), 20.0);
    }
}