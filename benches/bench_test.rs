/*
#![feature(test)]
extern crate test;
use test::Bencher;
use int::engine::trading_strategy::{
    TradingStrategy,
};
use int::engine::common::{
    BookEvent,
    Side,
    Action

};
#[bench]
fn bench_test_insert(b:&mut Bencher){
    b.iter(||{
        let n = test::black_box(100);
        (0..n).fold(0,|_,_|{test_insert();0})
    })
}
fn test_insert(){
    let mut trading_strategy = TradingStrategy::new(None,None,None);
        let book_event = BookEvent{
            id:None,
            bid_price:12,
            bid_quantity:100,
            offer_price:11,
            offer_quantity:150,
            status:None,
        };
    trading_strategy.handle_book_event(Some(book_event));
    assert_eq!(trading_strategy.orders.len(),2);
    assert_eq!(trading_strategy.orders.get(0).unwrap().side,Side::Ask);
    assert_eq!(trading_strategy.orders.get(1).unwrap().side,Side::Bid);
    assert_eq!(trading_strategy.orders.get(0).unwrap().price,12);
    assert_eq!(trading_strategy.orders.get(1).unwrap().price,11);
    assert_eq!(trading_strategy.orders.get(0).unwrap().quantity,100);
    assert_eq!(trading_strategy.orders.get(1).unwrap().quantity,100);
    assert_eq!(trading_strategy.orders.get(0).unwrap().action,Action::ToBeSent);
    assert_eq!(trading_strategy.orders.get(1).unwrap().action,Action::ToBeSent);
}*/
