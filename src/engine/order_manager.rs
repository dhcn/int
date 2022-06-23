use crate::engine::common::{Action, Order, Status,Side};
use std::collections::{HashMap};
use crossbeam_channel::{Receiver, Sender};
use std::ops::{Deref};
use rust_decimal::Decimal;
use crate::engine::common::Action::ToBeSent;


#[derive(Debug)]
pub struct OrderManager {
    pub ask_orders: HashMap<Decimal,Order>,
    pub bid_orders: HashMap<Decimal,Order>,
    pub ts_2_om: Receiver<Vec<Order>>,
    pub om_2_ts: Sender<Order>,
    pub om_2_gw: Sender<Vec<Order>>,
    pub gw_2_om: Receiver<Order>,
}
impl OrderManager {
    pub fn new(ts_2_om: Receiver<Vec<Order>>,
            om_2_ts: Sender<Order>,
            om_2_gw: Sender<Vec<Order>>,
            gw_2_om: Receiver<Order>)->OrderManager{
        OrderManager{
            ask_orders: HashMap::new(),
            bid_orders: HashMap::new(),
            ts_2_om: ts_2_om,
            om_2_ts: om_2_ts,
            om_2_gw: om_2_gw,
            gw_2_om: gw_2_om,
        }
    }
    pub fn handle_input_from_ts(&mut self) {
        while let Ok(mut r) = self.ts_2_om.recv() {
            let(ok_orders,error_orders)=self.handle_orders_from_trading_strategy(&mut r);
            let _=self.om_2_gw.send(ok_orders).unwrap();
            for o in error_orders.iter(){
                let _= self.om_2_ts.send(o.deref().clone()).unwrap();
            }
        }
    }
    pub fn handle_orders_from_trading_strategy(&mut self, orders:&mut Vec<Order>)->(Vec<Order>,Vec<Order>) {
        let mut ok_orders=vec![];
        let mut error_orders=vec![];
        for order in orders.iter_mut(){
            if self.check_order_valid(order) {
                if order.side==Side::Ask{
                    self.ask_orders.insert(order.price,order.deref().clone());
                }else{
                    self.bid_orders.insert(order.price,order.deref().clone());
                }

                ok_orders.push(order.deref().clone());

            }else{
                error_orders.push(order.deref().clone());
            }
        }
        return (ok_orders,error_orders);
    }
    #[allow(dead_code)]
    fn handle_input_from_market(&mut self) {
        while let Ok(r) = self.gw_2_om.recv(){
            let order =self.handle_order_from_gateway(r);
            if order.is_some(){
                let o = order.unwrap();
                let _=self.om_2_ts.send(o).unwrap();
            }
        }
    }
    pub fn handle_order_from_gateway(&mut self, exec_order: Order)->Option<Order> {
        //println!("exec_order1111:{:?}",exec_order);

        if exec_order.status.unwrap() == Status::Filled {
            return Some(exec_order);
        }
        let orders;
        if exec_order.side==Side::Ask{
            orders=&mut self.ask_orders;
        }else{
            orders=&mut self.bid_orders;
        }
        if orders.contains_key(&exec_order.price) &&
            orders.get(&exec_order.price).unwrap().side == exec_order.side {
            //println!("11111");
            if exec_order.status.unwrap() == Status::Acked {
                orders.remove(&exec_order.price);
                return Some(exec_order);
            } else if exec_order.status.unwrap() == Status::Rejected {
                let order = orders.get(&exec_order.price).unwrap();
                let price = exec_order.price;
                if order.action == Some(ToBeSent) {
                    orders.remove(&price);
                } else if order.status == Some(Status::ToModify) {
                    orders.remove(&price);
                } else if order.status == Some(Status::ToCancell) {
                    orders.remove(&price);
                }
                return Some(exec_order);
            } else if exec_order.status.unwrap() == Status::Modified {
                let order = orders.get(&exec_order.price).unwrap();
                let price = exec_order.price;
                if order.action == Some(Action::Cancel) {
                    orders.remove(&price);
                }
                return Some(exec_order);
            } else if exec_order.status.unwrap() == Status::Cancelled {
                let order = orders.get(&exec_order.price).unwrap();
                if order.action == Some(Action::Cancel) {
                    orders.remove(&exec_order.price);
                }
                return Some(exec_order);
            }
        }
        //println!("111112");
        None
    }

    fn check_order_valid(&self, order: &Order) -> bool {
        if order.size < 0.into() {
            return false;
        }
        if order.price < 0.into() {
            return false;
        }
        true
    }



}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::common::Side;
    use crossbeam_channel::bounded;

    #[test]
    fn test_receive_order_from_trading_strategy() {
        /*
        let (se,rec)= bounded(1);
        let (se1,rec1)= bounded(1);
        let mut order_manager = OrderManager {
            orders: HashMap::new(),
            ts_2_om: ,
            om_2_ts: ,
            om_2_gw: ,
            gw_2_om: Default::default(),
        };
        let order1 = Order {
            id: 10,
            price: 219.into(),
            quantity: 10.into(),
            side: Side::Bid,
            action: None,
            status: None,
        };
        order_manager.handle_order_from_trading_strategy(order1);
        assert_eq!(order_manager.orders.len(), 1);
        order_manager.handle_order_from_trading_strategy(order1);
        assert_eq!(order_manager.orders.len(), 2);
         */

    }
    #[test]
    fn test_receive_order_from_trading_strategy_error() {
        //let (se,rec)= bounded(1);
        //let (se1,rec1)= bounded(1);
        /*
        let mut order_manager = OrderManager {
            orders: Default::default(),
            ts_2_om: Default::default(),
            om_2_ts: Some(se),
            om_2_gw: Some(se1),
            gw_2_om: Default::default(),
        };
        let order1 = Order {
            id: 10,
            price: 219.into(),
            quantity: 10.into(),
            side: Side::Bid,
            action: None,
            status: None,
        };*/
        //order_manager.handle_order_from_trading_strategy(order1);
        //assert_eq!(order_manager.orders.len(), 1);
    }
    #[test]
    fn test_receive_from_gateway_filled() {}
    #[test]
    fn test_receive_from_gateway_acked() {}
}
