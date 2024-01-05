mod matching_engine;
use matching_engine::engine::{MatchingEngine, TradingPair};
use matching_engine::orderbook::{BidOrAsk, Order, Orderbook};
use rust_decimal_macros::dec;

fn main() {
    // create new orderbook and add limit orders to it
    let mut Orderbook = Orderbook::new();
    Orderbook.add_limit_order(dec!(640), Order::new(BidOrAsk::Ask, 10.0));
    //Orderbook.add_limit_order(dec!(90), Order::new(BidOrAsk::Ask, 10.0));
    //Orderbook.add_limit_order(dec!(200), Order::new(BidOrAsk::Ask, 10.0));
    //Orderbook.add_limit_order(dec!(340), Order::new(BidOrAsk::Ask, 10.0));

    // add market order to orderbook
    let mut market_order = Order::new(BidOrAsk::Bid, 10.0);

    //fill the market order with the existing limit orders
    Orderbook.fill_market_order(&mut market_order);

    let ask_limit = Orderbook.ask_limits();
    let matched_limit = ask_limit.get(0).unwrap();

    println!("{:?}", Orderbook.ask_limits());

}