
use std::collections::{BTreeMap};

use crossbeam_channel::{Receiver, Sender};
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromStr, FromPrimitive};
use serde_json::{Map, Value};

use crate::engine::common::{BookEvent, Depth, Side};

pub struct BybitOrderBook

{
    pub map_asks: BTreeMap<Decimal,Depth>,
    pub map_bids: BTreeMap<Decimal,Depth>,
    pub gw_2_ob: Receiver<String>,
    pub ob_2_ts: Sender<BookEvent>,
    pub current_bid: Option<Depth>,
    pub current_ask: Option<Depth>,
    pub symbol:Option<String>,
}

impl BybitOrderBook {
    pub fn handle_input_from_gateway(&mut self) {
        //let mut i = 2;
        while let Ok(r) = self.gw_2_ob.recv() {
            let be=self.handle_depth_from_gateway(r.clone());

            if be.is_some(){
                let _=self.ob_2_ts.send(be.unwrap()).unwrap();
            }
        }
    }
    pub fn handle_depth_from_gateway(&mut self, line:String)->Option<BookEvent>{
        let mut value: Value = serde_json::from_slice(line.as_bytes()).unwrap();
        let data_object = value.as_object_mut().unwrap();
        if data_object.contains_key("topic") && data_object["topic"].as_str() == Some("orderBook_200.100ms.BTCUSDT") {
            if let Some(sub_type) = data_object["type"].as_str() {
                if sub_type=="snapshot"{
                    let order_data = data_object["data"].as_object_mut().unwrap()["order_book"].as_array_mut().unwrap();
                    self.init_book(order_data);

                }else if sub_type=="delta"{
                    let delta_data = data_object["data"].as_object_mut().unwrap();
                    let be=self.update_book(delta_data);
                    return  be;
                }
            }
        }
        None
    }
    fn insert_depth(&mut self, depth_data:&mut Value){
        let depth_value = depth_data.as_object_mut().unwrap();
        let depth = Depth {
                price: Decimal::from_str(depth_value["price"].as_str().unwrap()).unwrap(),
                size: Decimal::from_f64(depth_value["size"].as_f64().unwrap()).unwrap(),
                side: if depth_value["side"].as_str()==Some("Sell"){Side::Ask}else {Side::Bid},
                count: None,
                timestamp:None
            };
        if depth.side==Side::Ask{
            if self.map_asks.contains_key(&depth.price){
                println!("blank sell insert:{:?}",depth);
                panic!();
            }
            self.map_asks.insert(depth.price,depth);
        }else if depth.side==Side::Bid{
            if self.map_bids.contains_key(&depth.price){
                println!("blank buy insert:{:?}",depth);
                panic!();
            }
            self.map_bids.insert(depth.price,depth);
        }

    }
    fn update_depth(&mut self, depth_data:&mut Value){
        let depth_value = depth_data.as_object_mut().unwrap();
        let depth = Depth {
                price: Decimal::from_str(depth_value["price"].as_str().unwrap()).unwrap(),
                size: Decimal::from_f64(depth_value["size"].as_f64().unwrap()).unwrap(),
                side: if depth_value["side"].as_str()==Some("Sell"){Side::Ask}else {Side::Bid},
                count: None,
                timestamp:None
            };
        if depth.side==Side::Ask{
            if !self.map_asks.contains_key(&depth.price){

                println!("blank sell update:{:?}",depth);
                panic!()
            }
            self.map_asks.insert(depth.price,depth);
        }else if depth.side==Side::Bid{
            if !self.map_bids.contains_key(&depth.price){
                println!("blank buy update:{:?}",depth);
                panic!();
            }
            self.map_bids.insert(depth.price,depth);
        }

    }
    fn remove_depth(&mut self, depth_value:&mut Value){
        let price = Decimal::from_str(depth_value["price"].as_str().unwrap()).unwrap();
        if depth_value["side"].as_str()==Some("Sell"){
            if self.map_asks.contains_key(&price){
                self.map_asks.remove(&price);
            }else{
                println!("blank buy delete:{}",price);
            }

        }else if depth_value["side"].as_str()==Some("Buy"){
            if self.map_bids.contains_key(&price){
                self.map_bids.remove(&price);
            }else{
                println!("blank sell delete:{}",price);
            }

        }

    }
    pub(crate) fn init_book(&mut self, order_data: &mut Vec<Value>) {
        self.map_asks.clear();
        self.map_bids.clear();

        for depth_data in order_data {
            self.insert_depth(depth_data);
        }
    }
    pub(crate) fn update_book(&mut self, delta_data: &mut Map<String, Value>)->Option<BookEvent> {
        let delete_data = delta_data["delete"].as_array_mut().unwrap();
        for delete_depth in delete_data{
            self.remove_depth(delete_depth);

        }
        let update_data = delta_data["update"].as_array_mut().unwrap();
        for update_depth in update_data{
            self.update_depth(update_depth);

        }
        let insert_data = delta_data["insert"].as_array_mut().unwrap();
        for insert_depth in insert_data{
            self.insert_depth(insert_depth);

        }
        self.check_generate_top_of_book_event()
    }

    fn check_generate_top_of_book_event(&mut self)->Option<BookEvent> {
        let mut top_changed = false;

        if self.map_bids.len()==0{
            if self.current_bid.is_some(){
                top_changed=true;
                self.current_bid=None;
            }
        }else{
            if self.current_bid.is_none()||self.current_bid.unwrap().price!=self.map_bids.last_key_value().unwrap().1.price ||
                self.current_bid.unwrap().size !=self.map_bids.last_key_value().unwrap().1.size{
                top_changed=true;
                self.current_bid=Some(self.map_bids.last_key_value().unwrap().1.clone())
            }
        }
        if self.map_asks.len()==0{
            if self.current_ask.is_some(){
                top_changed=true;
                self.current_ask=None;
            }
        }else{
            if self.current_ask.is_none()||self.current_ask.unwrap().price!=self.map_asks.first_key_value().unwrap().1.price||
                self.current_ask.unwrap().size !=self.map_asks.first_key_value().unwrap().1.size{
                top_changed=true;
                self.current_ask=Some(self.map_asks.first_key_value().unwrap().1.clone())
            }
        }
        if top_changed{
            Some(self.create_book_event())
        }else { None }



    }
    fn create_book_event(&mut self) -> BookEvent {
        BookEvent {
            bid_price: {
                if self.current_bid.is_some() {
                    self.current_bid.unwrap().price
                } else {
                    0.into()
                }
            },
            bid_size: {
                if self.current_bid.is_some() {
                    self.current_bid.unwrap().size
                } else {
                    0.into()
                }
            },
            ask_price: {
                if self.current_ask.is_some() {
                    self.current_ask.unwrap().price
                } else {
                    0.into()
                }
            },
            ask_size: {
                if self.current_ask.is_some() {
                    self.current_ask.unwrap().size
                } else {
                    0.into()
                }
            }
        }
    }

}





#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    use crossbeam_channel::bounded;

    use super::*;

    #[test]
    fn test_merge_orderbook() {
        let (se, rec) = bounded(1);
        let (se1, rec1) = bounded(1);
        let mut order_book = BybitOrderBook {
            map_asks: BTreeMap::new(),
            map_bids: BTreeMap::new(),
            gw_2_ob: rec,
            ob_2_ts: se1,
            current_bid: None,
            current_ask: None,
            symbol: Some("BTCUSDT".to_string())
        };


    }
}