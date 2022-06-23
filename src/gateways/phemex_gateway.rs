use crate::gateways::traits::MarketGateway;
use crate::utils::file::FileWriter;
use crate::utils::time::time_interval;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use ws::{connect, Handler, Message, Result};
use std::{thread, time};
use crate::utils::decode::dedecode_reader;
use crossbeam_channel::{ Sender as CSender};

#[derive(Serialize, Deserialize)]
struct SubCmd {
    id:u32,
    method: String,
    params: Vec<String>,
}


pub struct PhemexGateway {
}

pub struct PhemexHandler {
    //out:Sender,
    file_writer: FileWriter,
}
impl Handler for PhemexHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        //println!("Received message  on {:?}::", Instant::now());

        match msg {
            Message::Text(ref string) => {
                self.file_writer
                    .append_data(string.as_bytes());
            }
            Message::Binary(ref data) => {
                println!(
                    "Bybit Binary:{:?}",
                    dedecode_reader(data.to_vec()).unwrap()
                );
            }
        }
        Ok(())
    }

}
impl MarketGateway for PhemexGateway {
    fn subscribe(&self,_se:CSender<String>) {//for linear perpetual contracts
        loop {
            let url = "wss://phemex.com/ws";
            if let Err(error) = connect(url, |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("phemex WebSocket connected on {:?}",Instant::now());

                let sub_cmd = SubCmd {
                    id:10071,
                    method:"orderbook.subscribe".to_string(),
                    params: vec!["BTCUSD".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("phemex Client has sent message Subscribe cmd ")
                }
                //Verified fact: The snapshot and update of this book channel is like Okex book channel
                let sub_cmd = SubCmd {
                    id:10072,
                    method:"trade.subscribe".to_string(),
                    params: vec!["BTCUSD".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Phemex Client has sent message Subscribe cmd ")
                }

                // this can get indexPrice
                let sub_cmd = SubCmd {
                    id:10073,
                    method:"market24h.subscribe".to_string(),
                    params: vec![],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Phemex Client has sent message Subscribe cmd ")
                }


                time_interval(5 * 1000, move || {
                    //println!("Bybit ping....... ");
                    let sub_cmd = SubCmd {
                        id:10007,
                        method:"server.ping".to_string(),
                        params: vec![],

                    };
                    let cmd_str = serde_json::to_string(&sub_cmd).unwrap();
                    match out.send(cmd_str) {
                        Ok(()) => Ok(()),
                        Err(_error) => Err(()),
                    }
                });

                PhemexHandler {
                    //out,
                    file_writer: FileWriter::new("PhemexSubscriber".to_string()),
                }
            }) {
                // Inform the user of failure
                println!("Failed to create Phemex WebSocket due to: {:?}", error);
            }
            println!("Phemex Subscriber disconnected");
            let millis = time::Duration::from_millis(1000 * 2);
            thread::sleep(millis);
        }
    }
}
impl PhemexGateway {
    pub fn esubscribe(&self,_se:CSender<String>) {//for linear perpetual contracts
        loop {
            let url = "wss://phemex.com/ws";
            if let Err(error) = connect(url, |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("phemex WebSocket connected on {:?}",Instant::now());

                let sub_cmd = SubCmd {
                    id:10075,
                    method:"orderbook.subscribe".to_string(),
                    params: vec!["ETHUSD".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("phemex Client has sent message Subscribe cmd ")
                }
                //Verified fact: The snapshot and update of this book channel is like Okex book channel
                let sub_cmd = SubCmd {
                    id:10076,
                    method:"trade.subscribe".to_string(),
                    params: vec!["ETHUSD".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Phemex Client  has sent message Subscribe cmd ")
                }

                // this can get indexPrice
                let sub_cmd = SubCmd {
                    id:10077,
                    method:"market24h.subscribe".to_string(),
                    params: vec![],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Phemex Client market24h.subscribe has sent message Subscribe cmd ")
                }


                time_interval(5 * 1000, move || {
                    //println!("Bybit ping....... ");
                    let sub_cmd = SubCmd {
                        id:10007,
                        method:"server.ping".to_string(),
                        params: vec![],

                    };
                    let cmd_str = serde_json::to_string(&sub_cmd).unwrap();
                    match out.send(cmd_str) {
                        Ok(()) => Ok(()),
                        Err(_error) => Err(()),
                    }
                });

                PhemexHandler {
                    //out,
                    file_writer: FileWriter::new("PhemexeSubscriber".to_string()),
                }
            }) {
                // Inform the user of failure
                println!("Failed to create Phemex WebSocket due to: {:?}", error);
            }
            println!("Phemex Subscriber disconnected");
            let millis = time::Duration::from_millis(1000 * 2);
            thread::sleep(millis);
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;

    #[test]
    fn test_okex_gateway() {
        let mut phemex_gateway = PhemexGateway {
        };

    }
}
