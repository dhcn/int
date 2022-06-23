/*
* The MarketSimulator simulates the response mechanism of the trading system
*/
use std::collections::{ HashMap};
use std::str::FromStr;

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde_json::Value;

use crate::engine::common::{Action, Depth, Order, Side, Status};

pub struct MarketSimulator {
    pub(crate) ask_maps: HashMap<Decimal, Depth>,
    pub(crate) bid_maps: HashMap<Decimal, Depth>,
    pub(crate) ask_orders: HashMap<Decimal, Order>,
    pub(crate) bid_orders: HashMap<Decimal, Order>,
    //om_2_gw: Option<VecDeque<Order>>,
    //gw_2_om: Option<VecDeque<Order>>,
}

impl MarketSimulator {
    #[allow(dead_code)]
    fn lookup_orders(&mut self, _order: &Order) -> (Option<Order>, Option<usize>) {
        /*let result = self
            .orders
            .binary_search_by(|o| o.order_id.unwrap().cmp(&order.order_id.unwrap()));
        if result.is_ok() {
            let i = result.unwrap();
            let order = self.orders.get(i).unwrap().clone();
            return (Some(order), Some(i));
        } else {

        }*/
        return (None, None);
    }
    #[allow(dead_code)]
    fn handle_input_from_gw(&mut self) {
        /*if self.om_2_gw.is_some() {
            if self.om_2_gw.as_ref().unwrap().len() > 0 {
                let order = self.om_2_gw.as_mut().unwrap().pop_front().unwrap();
                self.handle_order_from_gw(order)
            }
        } else {
            println!("simu mode");
        }*/
    }
    pub fn handle_order_from_gw(&mut self, trade_order: Order) -> Option<Order> {
        //println!("handle_order_from_gw");
        if trade_order.side == Side::Bid {
            let price = &trade_order.price;
            let my_order = self.ask_orders.get_mut(price);
            if my_order.is_some(){
                let (order, left_quantity) = MarketSimulator::trade_with_order(my_order.unwrap(), trade_order.size);
                if left_quantity == 0.into() {
                    self.ask_orders.remove(price);
                }
                return order;
            }
            /*
            if self.ask_maps.contains_key(price) && self.ask_orders.contains_key(price) {
                let depth = self.ask_maps.get_mut(price).unwrap();
                let my_order = self.ask_orders.get_mut(price).unwrap();
                if depth.timestamp.unwrap() < my_order.timstamp.unwrap() {
                    let (left_quantity,remove_flag) = MarketSimulator::trade_with_depth(depth, trade_order.quantity);
                    if remove_flag{
                        self.ask_maps.remove(price);
                    }
                    let (order,left_quantity)=MarketSimulator::trade_with_order(my_order,left_quantity);
                    if left_quantity==0.into(){
                        self.ask_orders.remove(price);
                    }
                    return order

                }else {
                    let (order,left_quantity)=MarketSimulator::trade_with_order(my_order,trade_order.quantity);
                    if left_quantity==0.into(){
                        self.ask_orders.remove(price);
                    }
                    let (left_quantity,remove_flag) =MarketSimulator::trade_with_depth(depth,left_quantity);
                    if remove_flag{
                        self.ask_maps.remove(price);
                    }
                    return order
                }
            }else if self.ask_maps.contains_key(price) && !self.ask_orders.contains_key(price) {
                let depth = self.ask_maps.get_mut(price).unwrap();
                let (left_quantity,remove_flag)=MarketSimulator::trade_with_depth(depth, trade_order.quantity);
                if remove_flag{
                    self.ask_maps.remove(price);
                }
                return None
            }else if !self.ask_maps.contains_key(price) && self.ask_orders.contains_key(price) {
                let my_order = self.ask_orders.get_mut(price).unwrap();
                let (order,left_quantity)=MarketSimulator::trade_with_order(my_order,trade_order.quantity);
                if left_quantity==0.into(){
                    self.ask_orders.remove(price);
                }
                return order
            }*/
        }else if trade_order.side==Side::Ask{
            let price =&trade_order.price;
            let my_order = self.bid_orders.get_mut(price);
            if my_order.is_some(){
                let (order,left_quantity)=MarketSimulator::trade_with_order(my_order.unwrap(),trade_order.size);
                if left_quantity==0.into(){
                    self.bid_orders.remove(price);
                }
                return order;
            }
            /*
            if self.bid_maps.contains_key(price) && self.bid_orders.contains_key(price) {
                let depth = self.bid_maps.get_mut(price).unwrap();
                let my_order = self.bid_orders.get_mut(price).unwrap();
                if depth.timestamp.unwrap() < my_order.timstamp.unwrap() {
                    let (left_quantity,remove_flag) = MarketSimulator::trade_with_depth(depth, trade_order.quantity);
                    if remove_flag{
                        self.bid_maps.remove(price);
                    }
                    let (order,left_quantity)=MarketSimulator::trade_with_order(my_order,left_quantity);
                    if left_quantity==0.into(){
                        self.bid_orders.remove(price);
                    }
                    return order;
                }else {
                    let (order,left_quantity)=MarketSimulator::trade_with_order(my_order,trade_order.quantity);
                    if left_quantity==0.into(){
                        self.bid_orders.remove(price);
                    }
                    let (left_quantity,remove_flag) =MarketSimulator::trade_with_depth(depth,left_quantity);
                    if remove_flag{
                        self.bid_maps.remove(price);
                    }
                    return order
                }
            }else if self.bid_maps.contains_key(price) && !self.bid_orders.contains_key(price) {
                let depth = self.bid_maps.get_mut(price).unwrap();
                let (left_quantity,remove_flag) = MarketSimulator::trade_with_depth(depth, trade_order.quantity);
                if remove_flag{
                    self.bid_maps.remove(price);
                }
                return None
            }else if !self.bid_maps.contains_key(price) && self.bid_orders.contains_key(price) {
                let my_order = self.bid_orders.get_mut(price).unwrap();
                let (order,left_quantity)=MarketSimulator::trade_with_order(my_order,trade_order.quantity);
                if left_quantity==0.into(){
                    self.bid_orders.remove(price);
                }
                return order
            }*/

        }
        return None
    }
    pub fn trade_with_depth(depth: &mut Depth, trade_quantity: Decimal) -> (Decimal,bool) {
        let mut left_quantity = 0.into();
        let mut remove_flag =false;
        if depth.size > trade_quantity {
            depth.size = depth.size - trade_quantity ;
        } else if depth.size == trade_quantity  {
            remove_flag=true;
        } else if depth.size < trade_quantity  {
            left_quantity = trade_quantity  - depth.size;
            remove_flag=true;
        }
        (left_quantity,remove_flag)
    }
    pub fn trade_with_order(my_order: &mut Order, left_quantity: Decimal) -> (Option<Order>,Decimal) {
        if my_order.size > left_quantity {
            //println!("trade_order---1");
            my_order.size = my_order.size - left_quantity;
            (Some(Order {
                order_id: None,
                trade_id:None,
                client_oid: None,
                price:my_order.price,
                size: left_quantity,
                side: my_order.side,
                action: None,
                status: Some(Status::Filled),
                timstamp: None,
            }),my_order.size)
        } else if my_order.size == left_quantity {
            //println!("trade_order---2");
            //self.ask_orders.remove(&price);
            (Some(Order {
                order_id: None,
                trade_id:None,
                client_oid: None,
                price: my_order.price,
                size: left_quantity,
                side: my_order.side,
                action: None,
                status: Some(Status::Filled),
                timstamp: None,
            }),0.into())
        } else {
            //println!("trade_order---3");
            (Some(Order {
                order_id: None,
                trade_id:None,
                client_oid: None,
                price: my_order.price,
                size: my_order.size,
                side: my_order.side,
                action: None,
                status: Some(Status::Filled),
                timstamp: None,
            }),0.into())
        }
    }
    pub fn handle_depth_from_lp(&mut self, line: String) {
        let mut value: Value = serde_json::from_slice(line.as_bytes()).unwrap();
        let data_object = value.as_object_mut().unwrap();
        let data =data_object["data"].as_array().unwrap()[0].as_object().unwrap();
        let timestamp = data["timestamp"].as_str().unwrap();
        let timestamp = timestamp.parse::<DateTime<Utc>>().unwrap();
        let nanos = timestamp.timestamp_nanos();


        if data_object.contains_key("table") && data_object["table"].as_str() == Some("spot/depth_l2_tbt") {
            let mut order_data = data_object["data"].clone();
            let order_data = order_data.as_array_mut().unwrap().get_mut(0).unwrap().as_object_mut().unwrap();
            let asks_data: &Vec<Value> = order_data["asks"].as_array().unwrap();
            for ask_data in asks_data {
                let ask = ask_data.as_array().unwrap();
                //println!("ask:{:?}",ask);
                let depth = Depth {
                    price: Decimal::from_str(ask[0].as_str().unwrap()).unwrap(),
                    size: Decimal::from_str(ask[1].as_str().unwrap()).unwrap(),
                    side: Side::Ask,
                    count: Some(usize::from_str(ask[3].as_str().unwrap()).unwrap()),
                    timestamp: Some(nanos),
                };
                self.ask_maps.insert(depth.price, depth);
            }
            let bids_data = order_data["bids"].as_array().unwrap();
            for bid_data in bids_data {
                let bid = bid_data.as_array().unwrap();
                let depth = Depth {
                    price: Decimal::from_str(bid[0].as_str().unwrap()).unwrap(),
                    size: Decimal::from_str(bid[1].as_str().unwrap()).unwrap(),
                    side: Side::Bid,
                    count: Some(usize::from_str(bid[3].as_str().unwrap()).unwrap()),
                    timestamp: Some(nanos),
                };
                self.bid_maps.insert(depth.price, depth);
            }
        }
    }
    fn check_order(&mut self, _order: &Order) -> bool {
        true
    }
    pub fn handle_orders_from_om(&mut self, mut order: Order) -> Option<Order> {

        let mut orders=&mut HashMap::new();
        if !self.check_order(&order) {
            order.status = Some(Status::Rejected);
            return Some(order);
        }
        if order.side == Side::Ask {
            orders = &mut self.ask_orders;
        } else if order.side == Side::Bid {
            orders = &mut self.bid_orders;
        }
        let mut order = order.clone();
        if order.action == Some(Action::ToBeSent) {
            //println!("ms.ToBeSent");
            orders.insert(order.price, order.clone());
            order.status = Some(Status::Acked);
            return Some(order);
        } else if order.action == Some(Action::Modify) {
            //println!("ms.Modify");
            orders.insert(order.price, order.clone());
            order.status = Some(Status::Modified);
            return Some(order);
        } else if order.action == Some(Action::Cancel) {
            //println!("ms.Cancel");
            if !orders.contains_key(&order.price){
                panic!();
            }
            orders.remove(&order.price);
            order.status = Some(Status::Cancelled);
            return Some(order);
        }

        return None;
    }

    /*fn fill_all_orders(&mut self) {
        let mut orders_to_be_removed = Vec::new();
        let count = 0;
        for order in self.orders.iter() {
            order.status == Some(Status::Filled);
            orders_to_be_removed.push(count);
            if self.gw_2_om.is_some() {
                self.gw_2_om.as_mut().unwrap().push_front(order.clone());
            } else {
                println!("simu mode");
            }
        }
        for i in orders_to_be_removed.iter() {
            self.orders.remove(*i);
        }
    }*/
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_accept_order() {}
}
