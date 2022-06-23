/*
* The MarketSimulator simulates the response mechanism of the trading system
*/
use std::collections::{HashMap};
use std::str::FromStr;

use rust_decimal::Decimal;
use serde_json::{Value, Map};

use crate::engine::common::{Action, Depth, Order, Side, Status};
use rust_decimal::prelude::FromPrimitive;


pub struct BybitMarketSimulator {
    pub(crate) asks_map: HashMap<Decimal, Depth>,
    pub(crate) bids_map: HashMap<Decimal, Depth>,
    pub(crate) ask_orders: HashMap<Decimal, Order>,
    pub(crate) bid_orders: HashMap<Decimal, Order>,
    //om_2_gw: Option<VecDeque<Order>>,
    //gw_2_om: Option<VecDeque<Order>>,
}

impl BybitMarketSimulator {

    pub fn handle_order_from_gw(&mut self, trade_order: Order) -> Option<Order> {
        //println!("handle_order_from_gw");
        if trade_order.side == Side::Bid {
            let price = &trade_order.price;
            let my_order = self.ask_orders.get_mut(price);
            if my_order.is_some(){
                let (order, left_quantity) = BybitMarketSimulator::trade_with_order(my_order.unwrap(), &trade_order);
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
                let (order,left_quantity)=BybitMarketSimulator::trade_with_order(my_order.unwrap(),&trade_order);
                if left_quantity==0.into(){
                    self.bid_orders.remove(price);
                }
                return order;
            }
            /*
            //
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
    pub fn trade_with_order(my_order: &mut Order, trade_order:&Order) -> (Option<Order>,Decimal) {
        let left_quantity=trade_order.size;
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
                timstamp: trade_order.timstamp,
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
                timstamp: trade_order.timstamp,
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
                timstamp: trade_order.timstamp,
            }),0.into())
        }
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
            if self.asks_map.contains_key(&depth.price){
                println!("blank sell insert:{:?}",depth);
                panic!();
            }
            self.asks_map.insert(depth.price,depth);
        }else if depth.side==Side::Bid{
            if self.bids_map.contains_key(&depth.price){
                println!("blank buy insert:{:?}",depth);
                panic!();
            }
            self.bids_map.insert(depth.price,depth);
        }

    }
    fn update_depth(&mut self, depth_data:&mut Value){
        //println!("{:?}",depth_data);
        let depth_value = depth_data.as_object_mut().unwrap();
        //println!("{:?}",depth_value);
        let depth = Depth {
                price: Decimal::from_str(depth_value["price"].as_str().unwrap()).unwrap(),
                size: Decimal::from_f64(depth_value["size"].as_f64().unwrap()).unwrap(),
                side: if depth_value["side"].as_str()==Some("Sell"){Side::Ask}else {Side::Bid},
                count: None,
                timestamp:None
            };
        if depth.side==Side::Ask{
            if !self.asks_map.contains_key(&depth.price){
                println!("blank sell update{:?}",depth);
            }
            self.asks_map.insert(depth.price,depth);
        }else if depth.side==Side::Bid{
            if !self.bids_map.contains_key(&depth.price){
                println!("blank buy update{:?}",depth);
                println!("bids_map{:?}",self.bids_map);
                panic!();
            }
            self.bids_map.insert(depth.price,depth);
        }

    }
    fn remove_depth(&mut self, depth_value:&mut Value){
        let price = Decimal::from_str(depth_value["price"].as_str().unwrap()).unwrap();
        if depth_value["side"].as_str()==Some("Sell"){
            if self.asks_map.contains_key(&price){
                self.asks_map.remove(&price);
            }else{
                println!("blank buy delete:{}",price);
            }

        }else if depth_value["side"].as_str()==Some("Buy"){
            if self.bids_map.contains_key(&price){
                self.bids_map.remove(&price);
            }else{
                println!("blank sell delete:{}",price);
            }

        }

    }
    pub(crate) fn init_book(&mut self, order_data: &mut Vec<Value>) {
        self.asks_map.clear();
        self.bids_map.clear();

        for depth_data in order_data {
            self.insert_depth(depth_data);
        }
    }
    pub(crate) fn update_book(&mut self, delta_data: &mut Map<String, Value>){
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
    }

    pub fn handle_depth_from_lp(&mut self, line: String) {
        let mut value: Value = serde_json::from_slice(line.as_bytes()).unwrap();
        let data_object = value.as_object_mut().unwrap();
        if data_object.contains_key("topic") && data_object["topic"].as_str() == Some("orderBook_200.100ms.BTCUSDT") {
            if let Some(sub_type) = data_object["type"].as_str() {
                if sub_type=="snapshot"{
                    let order_data = data_object["data"].as_object_mut().unwrap()["order_book"].as_array_mut().unwrap();
                    self.init_book(order_data);
                }else if sub_type=="delta"{
                    let delta_data = data_object["data"].as_object_mut().unwrap();
                    self.update_book(delta_data);
                }
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
