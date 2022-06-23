
use std::collections::{BTreeMap};

use crossbeam_channel::{Receiver, Sender};
use rust_decimal::Decimal;
use rust_decimal::prelude::{FromStr};
use serde_json::{Map, Value};

use crate::engine::common::{BookEvent, Depth, Side};



pub struct OrderBook

{
    pub map_asks: BTreeMap<Decimal,Depth>,
    pub map_bids: BTreeMap<Decimal,Depth>,
    pub gw_2_ob: Receiver<String>,
    pub ob_2_ts: Sender<BookEvent>,
    pub current_bid: Option<Depth>,
    pub current_ask: Option<Depth>,
}

impl OrderBook {
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
        if data_object.contains_key("table") && data_object["table"].as_str() == Some("spot/depth_l2_tbt") {
            if let Some(action) = data_object["action"].as_str() {
                let mut order_data = data_object["data"].clone();
                let order_data = order_data.as_array_mut().unwrap().get_mut(0).unwrap().as_object_mut().unwrap();
                //println!("order_data:{:?}", order_data.len());
                if action == "partial" {
                    self.init_book(order_data);
                } else if action == "update" {
                    let be=self.update_book(order_data);
                    return  be;
                }
            }
        }
        None
    }
    pub(crate) fn init_book(&mut self, order_data: &mut Map<String, Value>) {
        self.map_asks.clear();
        self.map_bids.clear();

        let asks_data: & Vec<Value> = order_data["asks"].as_array().unwrap();
        for ask_data in asks_data {
            let ask = ask_data.as_array().unwrap();
            //println!("ask:{:?}",ask);
            let depth = Depth {
                price: Decimal::from_str(ask[0].as_str().unwrap()).unwrap(),
                size: Decimal::from_str(ask[1].as_str().unwrap()).unwrap(),
                side: Side::Ask,
                count: Some(usize::from_str(ask[3].as_str().unwrap()).unwrap()),
                timestamp:None
            };
            self.map_asks.insert(depth.price,depth);
        }
        let bids_data = order_data["bids"].as_array().unwrap();
        for bid_data in bids_data {
            let bid = bid_data.as_array().unwrap();
            let depth = Depth {
                price: Decimal::from_str(bid[0].as_str().unwrap()).unwrap(),
                size: Decimal::from_str(bid[1].as_str().unwrap()).unwrap(),
                side: Side::Bid,
                count: Some(usize::from_str(bid[3].as_str().unwrap()).unwrap()),
                timestamp:None
            };
            self.map_bids.insert(depth.price,depth);
        }
    }
    pub(crate) fn update_book(&mut self, order_data: &mut Map<String, Value>)->Option<BookEvent> {
        let asks_data: & Vec<Value> = order_data["asks"].as_array().unwrap();


        for ask_data in asks_data {
            let ask = ask_data.as_array().unwrap();
            let depth = Depth {
                price: Decimal::from_str(ask[0].as_str().unwrap()).unwrap(),
                size: Decimal::from_str(ask[1].as_str().unwrap()).unwrap(),
                side: Side::Ask,
                count: Some(usize::from_str(ask[3].as_str().unwrap()).unwrap()),
                timestamp:None
            };
            if depth.size ==0.into(){
                self.map_asks.remove(&depth.price);
            }else{
                self.map_asks.insert(depth.price,depth);
            }

        }
        // For Control Algorithm complexity O(n,m)
        let bids_data: &Vec<Value> = order_data["bids"].as_array().unwrap();


        for bid_data in bids_data {
            let bid = bid_data.as_array().unwrap();
            let depth = Depth {
                price: Decimal::from_str(bid[0].as_str().unwrap()).unwrap(),
                size: Decimal::from_str(bid[1].as_str().unwrap()).unwrap(),
                side: Side::Bid,
                count: Some(usize::from_str(bid[3].as_str().unwrap()).unwrap()),
                timestamp:None
            };
            if depth.size ==0.into(){
                self.map_bids.remove(&depth.price);
            }else{
                self.map_bids.insert(depth.price,depth);
            }

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
    /*
    fn get_list(&mut self, side: Side) -> BTreeMap<Decimal,Depth> {
        let mut lookup_list;
        if side == Side::Bid {
            lookup_list = self.list_bids;
        } else {
            lookup_list = self.list_asks;
        }
        return lookup_list;
    }*/
    /*
    fn find_order_in_a_list(
        &mut self,
        order: Order,
        lookup_list: Option<&Vec<Order>>,
    ) -> Option<Order> {
        let used_list;
        if lookup_list.is_none() {
            used_list = self.get_list(order);
        } else {
            used_list = lookup_list.unwrap()
        }

        for order in lookup_list.unwrap().iter() {
            if order.id == order.id {
                return Some(*order);
            }
        }
        print!("Order not found id={}", order.id);

        return None;
    }







    fn handle_order(&mut self, o: Order) {
        if o.action.unwrap() == Action::New {
            self.handle_new(o);
        } else if o.action.unwrap() == Action::Modify {
            self.handle_modify(o);
        } else if o.action.unwrap() == Action::Delete {
            self.handle_delete(o);
        } else {
            println!("Error-Cannot handle this action");
        }
        return self.check_generate_top_of_book_event();
    }
    fn handle_new(&mut self, o: Depth) {
        if o.side == Side::Bid {
            self.list_bids.push(o);
            self.list_bids.sort_by(|o1, o2| o1.price.cmp(&o2.price));
        } else {
            self.list_asks.push(Depth);
            self.list_asks.sort_by(|o1, o2| o1.price.cmp(&o2.price))
        }
    }
    fn handle_modify(&mut self, o: Order) {
        let mut order = self.find_order_in_a_list(o, None).unwrap();
        if order.quantity > o.quantity {
            order.quantity = o.quantity
        } else {
            println!("incorrect size")
        }
    }
    fn handle_delete(&mut self, o: Order) {}*/
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
        let mut order_book = OrderBook {
            map_asks: BTreeMap::new(),
            map_bids: BTreeMap::new(),
            gw_2_ob: rec,
            ob_2_ts: se1,
            current_bid: None,
            current_ask: None,
        };

        let f = File::open("/Users/denghui/PycharmProjects/int/data/OkexSubscriber2020-05-24").unwrap();
        let reader = BufReader::new(f);
        let mut i = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let mut value: Value = serde_json::from_slice(line.as_bytes()).unwrap();
            let data_object = value.as_object_mut().unwrap();
            //println!("line:{:?}", data_object);
            if data_object.contains_key("table") && data_object["table"].as_str() == Some("spot/depth_l2_tbt") {
                if let Some(action) = data_object["action"].as_str() {
                    let mut order_data = data_object["data"].clone();
                    let order_data = order_data.as_array_mut().unwrap().get_mut(0).unwrap().as_object_mut().unwrap();
                    let mut bids_data = order_data["bids"].clone();
                    let bids_data = bids_data.as_array_mut().unwrap();
                    let mut asks_data = order_data["asks"].clone();
                    let asks_data = asks_data.as_array_mut().unwrap();
                    //println!("{:?}", bids_data);
                    //println!("{:?}", asks_data);
                    if action == "partial" {
                        //println!("partial" );
                        order_book.init_book(order_data);
                        //println!("list_asks:{:?}", order_book.list_asks);
                        //println!("list_bids:{:?}", order_book.list_bids);
                        //assert_eq!(order_book.list_asks.len(), asks_data.len());
                        //assert_eq!(order_book.list_bids.len(), bids_data.len());
                        //assert!(check_order(&order_book.list_asks, true));
                        //assert!(check_order(&order_book.list_bids, false));
                        //assert!(check_data(asks_data, &order_book.list_asks));
                        //println!("bids_data:{:?}",bids_data);
                        //assert!(check_data(bids_data, &order_book.list_bids));
                    } else if action == "update" {
                        //println!("update" );
                        order_book.update_book(order_data);
                        //println!("update done" );
                        //println!("list_asks:{:?}", order_book.list_asks);
                        //println!("list_bids:{:?}", order_book.list_bids);
                        //assert!(check_order(&order_book.list_asks, true));
                        //assert!(check_order(&order_book.list_bids, false));
                        //assert!(check_data(asks_data, &order_book.list_asks));
                        //assert!(check_data(bids_data, &order_book.list_bids));
                        i = i + 1;
                        println!("i:{}", i);
                        if i > 1000 {
                            return;
                        }
                    }
                }
            }
        }
    }
}