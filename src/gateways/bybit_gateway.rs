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
    op: String,
    args: Vec<String>,
}


pub struct BybitGateway {
}

pub struct BybitHandler {
    //out:Sender,
    file_writer: FileWriter,
}
impl Handler for BybitHandler {
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
impl MarketGateway for BybitGateway {
    fn subscribe(&self,_se:CSender<String>) {//for linear perpetual contracts
        loop {
            let url = "wss://stream.bytick.com/realtime_public";
            if let Err(error) = connect(url, |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("Bybit WebSocket connected on {:?}",Instant::now());

                let sub_cmd = SubCmd {
                    op: "subscribe".to_string(),
                    args: vec!["orderBook_200.100ms.BTCUSDT".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bybit Client has sent message Subscribe cmd ")
                }

                let sub_cmd = SubCmd {
                    op: "subscribe".to_string(),
                    args: vec!["instrument_info.100ms.BTCUSDT".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bybit Client has sent message Subscribe cmd ")
                }
                //Verified fact: The snapshot and update of this book channel is like Okex book channel
                let sub_cmd = SubCmd {
                    op: "subscribe".to_string(),
                    args: vec!["trade.BTCUSDT".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bybit Client has sent message Subscribe cmd ")
                }

                time_interval(25 * 1000, move || {
                    //println!("Bybit ping....... ");
                    let sub_cmd = SubCmd {
                        op: "ping".to_string(),
                        args: vec![],

                    };
                    let cmd_str = serde_json::to_string(&sub_cmd).unwrap();
                    match out.send(cmd_str) {
                        Ok(()) => Ok(()),
                        Err(_error) => Err(()),
                    }
                });

                BybitHandler {
                    //out,
                    file_writer: FileWriter::new("BybitSubscriber".to_string()),
                }
            }) {
                // Inform the user of failure
                println!("Failed to create Bybit WebSocket due to: {:?}", error);
            }
            println!("Bybit Subscriber disconnected");
            let millis = time::Duration::from_millis(1000 * 2);
            thread::sleep(millis);
        }
    }
}
impl BybitGateway{
    pub fn isubscribe(&self) {//for inverse perpetual contracts
        loop {
            let url = "wss://stream.bytick.com/realtime";
            if let Err(error) = connect(url, |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("Bybit WebSocket connected on {:?}",Instant::now());

                let sub_cmd = SubCmd {
                    op: "subscribe".to_string(),
                    args: vec!["orderBook_200.100ms.BTCUSD".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bybit Client has sent message Subscribe cmd ")
                }

                let sub_cmd = SubCmd {
                    op: "subscribe".to_string(),
                    args: vec!["instrument_info.100ms.BTCUSD".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bybit Client has sent message Subscribe cmd ")
                }
                //Verified fact: The snapshot and update of this book channel is like Okex book channel
                let sub_cmd = SubCmd {
                    op: "subscribe".to_string(),
                    args: vec!["trade.BTCUSD".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bybit Client has sent message Subscribe cmd ")
                }

                time_interval(25 * 1000, move || {
                    //println!("Bybit ping....... ");
                    let sub_cmd = SubCmd {
                        op: "ping".to_string(),
                        args: vec![],

                    };
                    let cmd_str = serde_json::to_string(&sub_cmd).unwrap();
                    match out.send(cmd_str) {
                        Ok(()) => Ok(()),
                        Err(_error) => Err(()),
                    }
                });

                BybitHandler {
                    //out,
                    file_writer: FileWriter::new("BybitiSubscriber".to_string()),
                }
            }) {
                // Inform the user of failure
                println!("Failed to create Bybit ip WebSocket due to: {:?}", error);
            }
            println!("Bybit ip Subscriber disconnected");
            let millis = time::Duration::from_millis(1000 * 2);
            thread::sleep(millis);
        }
    }

    pub fn esubscribe(&self) {//for inverse perpetual contracts
        loop {
            let url = "wss://stream.bytick.com/realtime";
            if let Err(error) = connect(url, |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("Bybit WebSocket connected on {:?}",Instant::now());

                let sub_cmd = SubCmd {
                    op: "subscribe".to_string(),
                    args: vec!["orderBook_200.100ms.ETHUSD".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bybit Client has sent message Subscribe cmd ")
                }

                let sub_cmd = SubCmd {
                    op: "subscribe".to_string(),
                    args: vec!["instrument_info.100ms.ETHUSD".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bybit Client has sent message Subscribe cmd ")
                }
                //Verified fact: The snapshot and update of this book channel is like Okex book channel
                let sub_cmd = SubCmd {
                    op: "subscribe".to_string(),
                    args: vec!["trade.ETHUSD".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bybit Client has sent message Subscribe cmd ")
                }

                time_interval(25 * 1000, move || {
                    //println!("Bybit ping....... ");
                    let sub_cmd = SubCmd {
                        op: "ping".to_string(),
                        args: vec![],

                    };
                    let cmd_str = serde_json::to_string(&sub_cmd).unwrap();
                    match out.send(cmd_str) {
                        Ok(()) => Ok(()),
                        Err(_error) => Err(()),
                    }
                });

                BybitHandler {
                    //out,
                    file_writer: FileWriter::new("BybiteSubscriber".to_string()),
                }
            }) {
                // Inform the user of failure
                println!("Failed to create Bybit eth WebSocket due to: {:?}", error);
            }
            println!("Bybit ip Subscriber disconnected");
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
        let mut bybit_gateway = BybitGateway {
        };
        //bybit_gateway.subscribe()
    }
}
