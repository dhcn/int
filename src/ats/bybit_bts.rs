/*
Back Test System
 */

use crossbeam_channel::bounded;
use crate::ats::traits::TradingSystem;
use crate::engine::trading_strategy::TradingStrategy;
use crate::engine::order_manager::OrderManager;
use crate::engine::common::Order;
use crate::engine::common::Side;
use crate::engine::common::Status;
use rust_decimal::Decimal;
use serde_json::Value;
use std::str::FromStr;
use std::ops::Deref;
use crate::engine::liquidity_provider::LiquidProvider;
use chrono::{DateTime, Utc};
use crate::utils::time::get_nanos;
use crate::engine::bybit_market_simulator::BybitMarketSimulator;
use crate::engine::bybit_order_book::BybitOrderBook;
use rust_decimal::prelude::FromPrimitive;


pub struct BybitBackTestSystem {
}
impl TradingSystem for BybitBackTestSystem {
    fn run(&mut self) {
        let (_gw2ob_se, gw2ob_rec) = bounded(1);
        let (ob2ts_se, ob2ts_rec) = bounded(1);
        let (ts2om_se, ts2om_rec) = bounded(1);
        let (om2ts_se, om2ts_rec) = bounded(1);
        let (om2gw_se, _om2gw_rec) = bounded(1);
        let (_gw2om_se, gw2om_rec) = bounded(1);
        let mut order_book = BybitOrderBook {
            map_asks: Default::default(),
            map_bids: Default::default(),
            gw_2_ob: gw2ob_rec,
            ob_2_ts: ob2ts_se,
            current_bid: None,
            current_ask: None,
            symbol: Some("BTCUSDT".to_string())
        };
        let mut trading_stratey = TradingStrategy::new(ob2ts_rec,
                                                       ts2om_se,
                                                       om2ts_rec);

        let mut order_manager = OrderManager {
            ask_orders: Default::default(),
            bid_orders: Default::default(),
            ts_2_om: ts2om_rec,
            om_2_ts: om2ts_se,
            om_2_gw: om2gw_se,
            gw_2_om: gw2om_rec
        };
        let mut lp = LiquidProvider::new(Default::default());
        let mut ms = BybitMarketSimulator{
            asks_map: Default::default(),
            bids_map: Default::default(),
            ask_orders: Default::default(),
            bid_orders: Default::default()
        };
        //let f = File::open("/Users/denghui/PycharmProjects/int/data/OkexSubscriber2020-05-24").unwrap();
        //let reader = BufReader::new(f);
        lp.read_tick_data_from_data_source();
        let mut i: u128 = 0;
        //let mut flag = false;
        let mut count=0;
        let mut nanos_mean =0;
        //let mut depth_nanos =0;
        loop{
            let line =lp.get_line();
            i = i + 1;

            if line.is_none(){
                println!("file end");
                break;
            }
            let line = line.unwrap();

            let line=line.unwrap();
            //println!("line:{}",line);
            let mut value: Value = serde_json::from_slice(line.as_bytes()).unwrap();
            let data_object = value.as_object_mut().unwrap();


            if data_object.contains_key("topic") && data_object["topic"].as_str() == Some("orderBook_200.100ms.BTCUSDT")  {

                let _nanos_begin=get_nanos();
                //println!("{:?}", data_object);
                let timestamp = data_object["timestamp_e6"].as_str().unwrap();
                let _timestamp = timestamp.parse::<i64>().unwrap()*1000;

                //depth_nanos =timestamp;
                let begin_nanos=get_nanos();
                //println!("depth_nanos:{}",depth_nanos);
                //println!("begin_nanos:{}",begin_nanos);
                ms.handle_depth_from_lp(line.clone());
                let be = order_book.handle_depth_from_gateway(line);
                if be.is_some() {
                    let mut orders_sent = trading_stratey.handle_book_event(be.unwrap());

                    if orders_sent.is_some() {
                        let (mut ok_orders, error_orders) = order_manager.handle_orders_from_trading_strategy(orders_sent.as_mut().unwrap());
                        //println!("order_manager_orders:{:?}",order_manager.ask_orders);
                        //println!("order_manager_orders:{:?}",order_manager.bid_orders);
                        //println!("ok_orders:{:?}",ok_orders);
                        for order in error_orders.iter(){
                            trading_stratey.handle_market_response(order.deref().clone());
                        }
                        for order in ok_orders.iter_mut(){
                            order.timstamp=Some(Utc::now());//for mock time

                            //println!("ok_order:{:?}",order);
                            let execed_order= ms.handle_orders_from_om(order.deref().clone());
                            //println!("execed_order:{:?}",execed_order);
                            //println!("ms.bid_orders.len:{:?}",ms.bid_orders.len());
                            //println!("ms.ask_orders.len:{:?}",ms.ask_orders.len());
                            if execed_order.is_some() {
                                let ack_order =order_manager.handle_order_from_gateway(execed_order.unwrap());
                                //println!("ack order:{:?}",ack_order);
                                if ack_order.is_some(){
                                    trading_stratey.handle_market_response(ack_order.unwrap());
                                    //println!("ts.ask_orders:{:?}",trading_stratey.ask_orders);
                                    //println!("ts.bid_orders:{:?}",trading_stratey.bid_orders);

                                }else{
                                    panic!()
                                }
                            }else{
                                panic!()
                            }

                        }
                    }
                }

                if trading_stratey.bid_orders.len()==ms.bid_orders.len(){
                    //println!("trading_stratey.bid_orders:{:?}",trading_stratey.bid_orders);
                    //println!("ms.bid_orders:{:?}",ms.bid_orders);


                }else{
                    //println!("trading_stratey.bid_orders:{:?}",trading_stratey.bid_orders);
                    //println!("ms.bid_orders:{:?}",ms.bid_orders);
                    panic!()
                }
                if trading_stratey.ask_orders.len()==ms.ask_orders.len(){

                }else{
                    panic!()
                }
                nanos_mean=(nanos_mean*count+get_nanos()-begin_nanos)/(count+1);
                count=count+1;


                //println!("nanos_mean:{}",nanos_mean);

            } else if data_object.contains_key("topic") && data_object["topic"].as_str() == Some("trade.BTCUSDT"){
                let value: Value = serde_json::from_slice(line.as_bytes()).unwrap();
                //let mut trade_orders=vec![];
                let order_datas = value["data"].as_array().unwrap();
                for order_data in order_datas {
                    //println!("：{}",order_data);
                    let timestamp = order_data["timestamp"].as_str().unwrap();
                    let timestamp = timestamp.parse::<DateTime<Utc>>().unwrap();
                    let _order_nanos =timestamp.timestamp_nanos();

                    let order = Order {
                        order_id:None,
                        trade_id: Some(order_data["trade_id"].as_str().unwrap().to_string()),
                        client_oid:None,
                        price: Decimal::from_str(order_data["price"].as_str().unwrap()).unwrap(),
                        size: Decimal::from_f64(order_data["size"].as_f64().unwrap()).unwrap(),
                        side: if order_data["side"].as_str().unwrap() == "Sell" { Side::Ask } else { Side::Bid },
                        action: None,
                        status: Some(Status::Filled),
                        timstamp:Some(timestamp),
                    };
                    //println!("new trade_order from market:{:?}",order);
                    //println!("ms.bid_orders::::::{:?}",ms.bid_orders);
                    //println!("ms.ask_orders::::::{:?}",ms.ask_orders);
                    let exec_order= ms.handle_order_from_gw(order);
                    //println!("traded_order:{:?}",exec_order);

                    if exec_order.is_some(){

                        let exec_order =order_manager.handle_order_from_gateway(exec_order.unwrap());
                        //print!("exec traded_order:{:?}",exec_order);
                        if exec_order.is_some(){
                            println!("execed traded_order:{:?}",exec_order);
                            trading_stratey.handle_market_response(exec_order.unwrap());
                            //println!("traded ts.ask_orders:{:?}",trading_stratey.ask_orders);
                            //println!("traded ts.bid_orders:{:?}",trading_stratey.bid_orders);
                        }
                    }

                }
            }

        }

        println!("pnl:::::::{}",trading_stratey.get_pnl());
        //println!("i:{}",i);
        //println!("bid_size：{},bid_value:{}",bid_size,bid_value);
        //println!("sell_size：{},sell_value:{}",sell_size,sell_value);
        //println!("bids_map：{:?}",bids_map);
        //println!("asks_map：{:?}",asks_map);
    }

}


#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;
    use std::thread;
    use crossbeam_channel::bounded;
    use crate::ats::bybit_bts::{BybitBackTestSystem};
    use crate::ats::traits::TradingSystem;
    use crate::engine::common::{Order, Side};
    use std::ops::Deref;


    #[test]
    fn test_bts(){
        let mut bts = BybitBackTestSystem{};
        bts.run();
    }
    #[test]
    fn test_deref_result(){
        let mut order_list = vec![];
        let order = Order{
            order_id: None,
            trade_id:None,
            price: 1.into(),
            size: 2.into(),
            side: Side::Bid,
            action: None,
            status: None,
            client_oid: None,
            timstamp: None
        };
        order_list.push(order);
        let mut order = Order{
            order_id: None,
            trade_id:None,
            price: 2.into(),
            size: 3.into(),
            side: Side::Bid,
            action: None,
            status: None,
            client_oid: None,
            timstamp: None
        };
        order_list.push(order);
        for o in order_list.iter_mut(){
            o.price=0.into();
            order = o.deref().clone();
        }
        println!("list:{:?}",order_list);
    }


    #[test]
    fn test_send() {


        //for city in cities.iter() {
        //  println!("{}, ", city);
        //}
        let (se, rec ) = bounded(1);;
        let handle = thread::spawn(move || {
            let cities = ["Toronto", "New York", "Melbourne"];

            for city in cities.iter(){
                //println!("{}",city.to_string());
                se.send(city.to_string());
            }
        });
        while  let Ok(r)=rec.recv() {
            println!("received {}",r);

        }
        //handle.join();

    }
}
