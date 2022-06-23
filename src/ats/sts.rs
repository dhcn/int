/*
Simulated Trading System
 */

/*
Back Test System
 */


use std::collections::btree_map::BTreeMap;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crossbeam_channel::bounded;
use std::thread;
use crate::ats::traits::TradingSystem;
use crate::engine::order_book::OrderBook;
use crate::engine::trading_strategy::TradingStrategy;
use crate::engine::order_manager::OrderManager;


pub struct SimTradingSystem {
}
impl TradingSystem for SimTradingSystem{
    fn run(&mut self) {

        let (g2o_se, g2o_rec ) = bounded(1);

        let gateway_handle = thread::spawn(move || {
            let f = File::open("/Users/denghui/PycharmProjects/int/data/OkexSubscriber2020-05-24").unwrap();
            let reader = BufReader::new(f);
            for line in reader.lines() {
                let line = line.unwrap();
                let _=g2o_se.send(line).unwrap();
            }
        });

        let (o2t_se, o2t_rec ) = bounded(1);
        let mut order_book = OrderBook{
            map_asks: BTreeMap::new(),
            map_bids: BTreeMap::new(),
            gw_2_ob: g2o_rec,
            ob_2_ts: o2t_se,
            current_bid: None,
            current_ask: None,
        };
        let orderbook_handle = thread::spawn(move || {
            order_book.handle_input_from_gateway()
        });

        let (t2om_se, t2om_rec ) = bounded(1);
        let (om2t_se, om2t_rec ) = bounded(1);
        let mut ts =TradingStrategy::new(o2t_rec,
                                         t2om_se,om2t_rec);

        let ob2ts_handle = thread::spawn(move || {
            ts.handle_input_from_ob()
        });
        let (om2gw_se, _om2gw_rec ) = bounded(1);
        let (_gw2om_se, gw2om_rec ) = bounded(1);
        let mut om = OrderManager::new(t2om_rec,om2t_se,
                                       om2gw_se,gw2om_rec);
        let ts2om_handle = thread::spawn(move || {
            om.handle_input_from_ts()
        });



        let _ = gateway_handle.join().unwrap();
        let _ = orderbook_handle.join().unwrap();
        let _ = ob2ts_handle.join().unwrap();
        let _ = ts2om_handle.join().unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::sync::mpsc::channel;
    use std::thread;
    use crossbeam_channel::bounded;
    use crate::ats::sts::SimTradingSystem;
    use crate::ats::traits::TradingSystem;


    #[test]
    fn test_bts(){
        let mut sts = SimTradingSystem{};
        sts.run();
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
