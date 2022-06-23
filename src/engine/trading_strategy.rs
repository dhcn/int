use crate::engine::common::{Action, BookEvent, Order, Side, Status, get_oid};
use std::collections::{HashMap};
use std::ops::{Deref};
use rust_decimal::Decimal;
use crossbeam_channel::{Receiver, Sender};
use std::str::FromStr;
use crate::engine::common::Action::ToBeSent;

#[derive(Debug)]
pub struct TradingStrategy {
    pub ask_orders: HashMap<Decimal,Order>,
    pub bid_orders: HashMap<Decimal,Order>,
    //pub order_id: usize,
    pub position: Decimal,
    pub pnl: Decimal,
    pub cash: Decimal,
    pub current_bid: Decimal,
    pub current_ask: Decimal,
    ob_2_ts: Receiver<BookEvent>,
    ts_2_om: Sender<Vec<Order>>,
    om_2_ts: Receiver<Order>,
    min_quantity:Decimal,
}

impl TradingStrategy {
    pub fn new(
        ob_2_ts: Receiver<BookEvent>,
        ts_2_om: Sender<Vec<Order>>,
        om_2_ts: Receiver<Order>,
    ) -> TradingStrategy {
        TradingStrategy {
            ask_orders: HashMap::new(),
            bid_orders:HashMap::new(),
            //order_id: 0,
            position: 0.into(),
            pnl: 0.into(),
            cash: 100.into(),
            current_bid: 0.into(),
            current_ask: 0.into(),
            ob_2_ts: ob_2_ts,
            ts_2_om: ts_2_om,
            om_2_ts: om_2_ts,
            min_quantity:Decimal::from_str("0.00001").unwrap()
        }
    }
    pub fn handle_input_from_ob(&mut self) {
        while let Ok(r) = self.ob_2_ts.recv() {
            let action_orders=self.handle_book_event(r);
            if action_orders.is_some(){
                let _=self.ts_2_om.send(action_orders.unwrap()).unwrap();
            }
        }
    }
    pub fn handle_book_event(&mut self, book_event: BookEvent)->Option<Vec<Order>> {

        self.current_bid = book_event.bid_price;
        self.current_ask = book_event.ask_price;
        if self.signal(&book_event) {
            let  (ask_orders,bid_orders) =self.create_orders();
            //println!("to plan ask_orders:{:?}",ask_orders);
            //println!("to plan bid_orders:{:?}",bid_orders);
            let action_orders=self.merge_orders(ask_orders,bid_orders);
            //println!("to send action_orders:{:?}",action_orders);
            return action_orders;
        }
        None
    }
    fn signal(&self, book_event: &BookEvent) -> bool {

        if book_event.ask_price > book_event.bid_size {
            if self.cash>0.into() ||self.position>0.into() {
                return true;
            }
        }
        return false;
    }
    fn create_orders(&mut self)->(HashMap<Decimal,Order>,HashMap<Decimal,Order>) {
        let mut ask_orders=HashMap::new();
        let mut bid_orders=HashMap::new();
        if self.cash>0.into(){
            if self.cash/self.current_bid>self.min_quantity*Decimal::new(10,0){
                let bid_quantity =  (self.cash/self.current_bid).round_dp(5)-Decimal::from_str("0.00001").unwrap();
                let order = Order {
                    order_id: None,
                    trade_id:None,
                    client_oid: Some(get_oid("okex")),
                    price: self.current_bid,
                    size: bid_quantity,
                    side: Side::Bid,
                    action: Some(Action::ToBeSent),
                    status: None,
                    timstamp:None
                };
                bid_orders.insert(order.price,order);
            }
            if self.position>self.min_quantity{
                let order = Order {
                    order_id: None,
                    trade_id:None,
                    client_oid:Some(get_oid("okex")),
                    price: self.current_ask,
                    size: self.position.round_dp(5),
                    side: Side::Ask,
                    action: None,
                    status: None,
                    timstamp:None,
                };
                ask_orders.insert(order.price,order);
            }
        }
        return (ask_orders,bid_orders);
    }

    fn merge_orders(&mut self, mut ask_orders:HashMap<Decimal,Order>,mut bid_orders:HashMap<Decimal,Order>)->Option<Vec<Order>>{
        let mut action_orders=Vec::new();
        for order_entry in self.ask_orders.iter_mut(){
            let mut order = order_entry.1;
            if !ask_orders.contains_key(&order.price){
                order.status=Some(Status::ToCancell);
                order.action=Some(Action::Cancel);
                action_orders.push(order.deref().clone())
            }
        }
        for order_entry in self.bid_orders.iter_mut(){
            let mut order = order_entry.1;
            if !bid_orders.contains_key(&order.price){
                order.status=Some(Status::ToCancell);
                order.action=Some(Action::Cancel);
                action_orders.push(order.deref().clone())
            }
        }

        for order_entry in ask_orders.iter_mut(){
            let mut order = order_entry.1;
            if self.ask_orders.contains_key(&order.price) {
                let modified_order = self.ask_orders.get_mut(&order.price).unwrap();
                if modified_order.price!=order.price{
                    modified_order.status=Some(Status::ToModify);
                    modified_order.action=Some(Action::Modify);
                    order.status=Some(Status::ToModify);
                    order.action=Some(Action::Modify);
                    action_orders.push(order.deref().clone());
                }
            }else{
                order.status=Some(Status::New);
                order.action=Some(Action::ToBeSent);
                //self.order_id +=1;
                //order.order_id=Some(self.order_id);
                self.ask_orders.insert(order.price,order.deref().clone());
                action_orders.push(order.deref().clone());
            }
        }
        for order_entry in bid_orders.iter_mut(){
            let mut order = order_entry.1;
            //println!("bid_order:{:?}",order);
            if self.bid_orders.contains_key(&order.price) {
                //println!("1--bid_order:{:?}",order);
                let modified_order = self.bid_orders.get_mut(&order.price).unwrap();
                if modified_order.price!=order.price{
                    modified_order.status=Some(Status::ToModify);
                    modified_order.action=Some(Action::Modify);
                    order.status=Some(Status::ToModify);
                    order.action=Some(Action::Modify);
                    action_orders.push(order.deref().clone());
                }
            }else{
                //println!("2--bid_order:{:?}",order);
                order.status=Some(Status::New);
                order.action=Some(Action::ToBeSent);
                //self.order_id +=1;
                //order.order_id=Some(self.order_id);
                self.bid_orders.insert(order.price,order.deref().clone());
                action_orders.push(order.deref().clone());
            }
        }
        //println!("to send orders:{:?}",action_orders);
        return Some(action_orders);


    }
    fn sync_order(&mut self,exec_order:Order){
        //println!("begin:::{:?}", self);
        let mut orders:&mut HashMap<Decimal,Order>=&mut HashMap::new();
        if exec_order.side==Side::Ask{
            orders= & mut self.ask_orders;
        }
        else if exec_order.side==Side::Bid{
            orders= & mut self.bid_orders;
        }

        if exec_order.status.unwrap()== Status::Acked {
            let order = orders.get_mut(&exec_order.price).unwrap();

            order.status = Some(Status::Acked);
            order.action = Some(Action::NoAction);

        }else if exec_order.status.unwrap() == Status::Rejected {
            let order = orders.get_mut(&exec_order.price).unwrap();
            let mut to_remove=false;
            if order.action==Some(ToBeSent){
                to_remove=true;

            }else if order.status==Some(Status::ToModify){
                order.status=Some(Status::Rejected);
                order.action=Some(Action::NoAction);
            }else if order.status==Some(Status::ToCancell){
                order.status=Some(Status::Rejected);
                order.action=Some(Action::NoAction);
            }
            if to_remove{
                orders.remove(&exec_order.price);
            }


        }else if exec_order.status.unwrap() == Status::Modified {
            let order = orders.get_mut(&exec_order.price).unwrap();
            order.status=Some(Status::Modified);
            order.action=Some(Action::NoAction);
            order.size =exec_order.size;
        }else if exec_order.status.unwrap() == Status::Cancelled {
            let order = orders.get(&exec_order.price).unwrap().deref().clone();
            if order.action==Some(Action::Cancel){
                orders.remove(&order.price);
            }

        }else if exec_order.status.unwrap() == Status::Filled {
            let order = orders.get_mut(&exec_order.price).unwrap();

            order.size =order.size -exec_order.size;
            let order = order.deref().clone();
            if order.size ==0.into(){
                orders.remove(&order.price);
            }
            let pos: Decimal = if order.side == Side::Bid {
                println!("bid price:::::::::::::::::::::{},{}",self.current_bid,order.price);
                exec_order.size
            } else {
                println!("ask price:::::::::::::::::::::{},{}",self.current_ask,order.price);
                Decimal::new(0,0) - exec_order.size
            };

            self.position += pos;
            self.pnl -= pos * order.price;
            self.cash -= pos * order.price;
            println!("exec_order:::::::::::::{:?}",exec_order);
            println!("pnl:::::::::::::{}",self.get_pnl());
            println!("position::::::::{}",self.position);
            println!("cash::::::::::::{}",self.cash);
        }



    }
    pub fn handle_response_from_om(&mut self) {
        while let Ok(r) = self.om_2_ts.recv() {
            self.handle_market_response(r)
        }
    }
    pub fn handle_market_response(&mut self, order_exec: Order) {
        //println!("ack order:::::::{:?}",order_exec);
        self.sync_order(order_exec);
        //println!("ts.bid_orders:::::::{:?}",self.bid_orders);
        //println!("ts.ask_orders:::::::{:?}",self.ask_orders);

        //println!("cash:::::::{}",self.cash);
        if self.cash<0.into(){
            panic!();
        }
    }
    pub fn get_pnl(&self)->Decimal{
        self.pnl + self.position * (self.current_bid+self.current_ask) / Decimal::new(2,0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_full() {
        //let mut trading_strategy = TradingStrategy::new(None, None, None);
        //test_receive_top_of_book(&mut trading_strategy);
        //test_rejected_order(&mut trading_strategy);
        //test_filled_order(&mut trading_strategy)
    }
    //#[test]
    fn test_receive_top_of_book(trading_strategy: &mut TradingStrategy) {
        let book_event = BookEvent {
            bid_price: 12.into(),
            bid_size: 100.into(),
            ask_price: 11.into(),
            ask_size: 150.into(),
        };
        trading_strategy.handle_book_event(book_event);
        /*
        assert_eq!(trading_strategy.orders.len(), 2);
        assert_eq!(trading_strategy.orders.get(0).unwrap().side, Side::Ask);
        assert_eq!(trading_strategy.orders.get(1).unwrap().side, Side::Bid);
        assert_eq!(trading_strategy.orders.get(0).unwrap().price, 12.into());
        assert_eq!(trading_strategy.orders.get(1).unwrap().price, 11.into());
        assert_eq!(trading_strategy.orders.get(0).unwrap().quantity, 100.into());
        assert_eq!(trading_strategy.orders.get(1).unwrap().quantity, 100.into());
        assert_eq!(
            trading_strategy.orders.get(0).unwrap().action.unwrap(),
            Action::ToBeSent
        );
        assert_eq!(
            trading_strategy.orders.get(1).unwrap().action.unwrap(),
            Action::ToBeSent
        );*/
    }
    //#[test]
    fn test_rejected_order(trading_strategy: &mut TradingStrategy) {
        let order_exec = Order {
            order_id: None,
            trade_id:None,
            client_oid:None,
            price: 12.into(),
            size: 100.into(),
            side: Side::Ask,
            action: None,
            status: Some(Status::Rejected),
            timstamp: None
        };
        trading_strategy.handle_market_response(order_exec);
        //assert_eq!(trading_strategy.orders.get(0).unwrap().side, Side::Ask);
        //assert_eq!(trading_strategy.orders.get(0).unwrap().price, 12.into());
        //assert_eq!(trading_strategy.orders.get(0).unwrap().quantity, 100.into());
        //assert_eq!(trading_strategy.orders.get(0).unwrap().status.unwrap(),Status::New);
    }

    //#[test]
    fn test_filled_order(trading_strategy: &mut TradingStrategy) {
        let order_exec = Order {
            order_id: None,
            trade_id:None,
            client_oid:None,
            price: 11.into(),
            size: 100.into(),
            side: Side::Ask,
            status: Some(Status::Filled),
            action: None,
            timstamp: None
        };
        trading_strategy.handle_market_response(order_exec);
        let order_exec = Order {
            order_id: Some(2),
            trade_id:None,
            client_oid: None,
            price: 12.into(),
            size: 100.into(),
            side: Side::Bid,
            status: Some(Status::Filled),
            action: None,
            timstamp: None
        };
        trading_strategy.handle_market_response(order_exec);
        assert_eq!(trading_strategy.position, 0.into());
        assert_eq!(trading_strategy.cash, 100.into());
        assert_eq!(trading_strategy.pnl, 1100.into());
    }
}
