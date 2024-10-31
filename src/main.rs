mod matching_engine;
use matching_engine::orderbook::{BidorAsk, Order, Orderbook};
use matching_engine::engine::{TradingPair, MatchingEngine};

fn main() {
    let buy_order_from_alison = Order::new(BidorAsk::Bid, 5.5);
    let buy_order_from_duncan = Order::new(BidorAsk::Bid, 5.5);
    let ask_order_from_duncan = Order::new(BidorAsk::Ask, 5.5);
    let mut orderbook = Orderbook::new();
    orderbook.add_order(5.5, buy_order_from_alison);
    orderbook.add_order(5.5, buy_order_from_duncan);
    orderbook.add_order(5.5, ask_order_from_duncan);
    println!("{:?}", orderbook);

    let mut engine = MatchingEngine::new();
    let pair = TradingPair::new("BTC".to_string(), "USD".to_string());
    engine.add_new_market(pair.clone());
    let pair2 = TradingPair::new("BTD".to_string(), "USD".to_string());

    let buy_order = Order::new(BidorAsk::Bid, 6.5);
    match engine.place_limit_order(pair2, 10.000, buy_order) {
        Ok(()) => {

        }
        Err(e) => {

        }
    }
}
