/*
* LiquidProvider simulate the data source which system subscribe
*/
use crate::engine::common::{Action, Order};
use rand::distributions::{Distribution, Standard};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, BufRead, Lines,Result};

pub struct LiquidProvider {
    orders: Vec<Order>,
    order_id: usize,
    lp_2_gateway: Option<VecDeque<Order>>,
    lines:Option<Lines<BufReader<File>>>,
}

impl LiquidProvider {
    pub fn new(lp_2_gateway: Option<VecDeque<Order>>) -> Self {
        return LiquidProvider {
            #[allow(dead_code)]
            orders: Vec::new(),
            #[allow(dead_code)]
            order_id: 0,
            #[allow(dead_code)]
            lp_2_gateway: lp_2_gateway,
            lines: None
        };
    }
    #[allow(dead_code)]
    fn lookup_orders(&self, id: usize) -> (Option<&Order>, Option<isize>) {
        let mut count = 0;
        for ord in &self.orders {
            if ord.order_id == Some(id) {
                return (Some(ord), Some(count));
            }
            count += 1;
        }
        return (None, None);
    }
    #[allow(dead_code)]
    fn insert_manual_order(&mut self, order: Order) -> Option<Order> {
        if self.lp_2_gateway.is_none() {
            print!("simulation mode");
            return Some(order);
        }
        self.lp_2_gateway.as_mut().unwrap().push_front(order);
        None
    }
    pub fn read_tick_data_from_data_source(&mut self){
        let f = File::open("/Users/denghui/PycharmProjects/int/data/BybitSubscriber2021-03-23").unwrap();
        let reader = BufReader::new(f);
        self.lines = Some(reader.lines());

    }
    pub fn get_line(&mut self)->Option<Result<String>>{
        self.lines.as_mut().unwrap().next()

    }
    #[allow(dead_code)]
    fn generate_random_order(&mut self) -> Option<Order> {
        let seed: [u8; 32] = [0; 32];
        let mut rng: StdRng = StdRng::from_seed(seed);

        let price = rng.gen_range(8, 12);
        let quantity = rng.gen_range(1, 10) * 100;
        let side = Standard.sample(&mut rng);
        println!("{:?}", side);
        let order_id = rng.gen_range(0, self.order_id + 1);
        let order = self.lookup_orders(order_id).0;

        let mut new_order = false;
        let action: Option<Action> ;

        if order.is_none() {
            action = Some(Action::New);
            new_order = true;
        } else {
            action = Some([Action::Modify, Action::Delete][rng.gen_range(0, 1)]);
        }

        let order = Order {
            order_id: Some(self.order_id),
            trade_id:None,
            client_oid:None,
            price: price.into(),
            size: quantity.into(),
            side: side,
            action: action,
            status: None,
            timstamp:None,
        };
        println!("order is {:?}", order);
        println!("new_order:{}", new_order);
        if new_order {
            self.order_id = self.order_id + 1;
            self.orders.push(order.clone())
        }
        if self.lp_2_gateway.is_none() {
            print!("simulation mode");
            return Some(order);
        }
        self.lp_2_gateway.as_mut().unwrap().push_front(order);
        None
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    //use std::borrow::BorrowMut;
    use crate::engine::common::Side;

    #[test]
    fn test_liquid_provider() {
        let mut liquid_provider = LiquidProvider::new(Some(VecDeque::new()));
        liquid_provider.generate_random_order();
        println!("--->{}", liquid_provider.orders.len());
        assert_eq!(liquid_provider.orders.get(0).unwrap().order_id.unwrap(), 0);
        assert_eq!(liquid_provider.orders.get(0).unwrap().side, Side::Bid);
        assert_eq!(liquid_provider.orders.get(0).unwrap().size, 200.into());
        assert_eq!(liquid_provider.orders.get(0).unwrap().price, 10.into());
    }
    #[test]
    fn test_rejected_order() {}
}
