use rand::distributions::{Distribution, Standard};
use rand::Rng;
use rust_decimal::Decimal;
use crate::utils::time::get_nanos;
use chrono::{DateTime, Utc};


#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Side {
    Bid,
    Ask,
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Exchange{
    Binance,
    Bitfinex,
    Bitmex,
    Coinbase,
    Huobi,
    Okex,
    Uniswap,
    Bybit,
}
impl Distribution<Side> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Side {
        [Side::Bid, Side::Ask][rng.gen_range(0, 1)]
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Action {
    New,
    Modify,// for Price Gateway
    Delete,// for Price Gateway
    ToBeSent, // for Market Trade Gateway
    NoAction, // for Market Trade Gateway
    Cancel,   // for Market Trade Gateway
    Amend,// for Market Trade Gateway
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    New,
    Acked,
    Rejected,
    ToModify,
    Modified,
    ToCancell,
    Cancelled,
    Filled,
}
#[derive(Debug, Clone, PartialEq)]
pub struct Order {
    pub order_id: Option<usize>,
    pub trade_id: Option<String>,
    pub client_oid:Option<String>,
    pub price: Decimal,
    pub size: Decimal,
    pub side: Side,
    pub action: Option<Action>,
    pub status: Option<Status>,
    pub timstamp: Option<DateTime<Utc>>,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Depth {
    pub price: Decimal,
    pub size: Decimal,
    pub side: Side,
    pub count:Option<usize>,
    pub timestamp:Option<i64>,
}

#[derive(Debug, Copy, Clone)]
pub struct BookEvent {
    pub bid_price: Decimal,
    pub bid_size: Decimal,
    pub ask_price: Decimal,
    pub ask_size: Decimal,
}
pub fn get_oid(prefix:&str)->String{
    let nanos =get_nanos();
    let mut oid = prefix.to_string();
    oid.push_str(nanos.to_string().as_str());
    oid


}
