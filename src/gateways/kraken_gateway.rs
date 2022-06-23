use crate::gateways::traits::MarketGateway;
use crate::utils::file::FileWriter;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use ws::{connect, Handler, Message, Result};
use std::{thread, time};
use crate::utils::decode::dedecode_reader;
use crossbeam_channel::{ Sender as CSender};
use serde_json::Value;
use serde_json::json;


#[derive(Serialize, Deserialize)]
struct SubCmd {
    event:String,
    reqid: i32,
    pair: Vec<String>,
    subscription:Value,
}


pub struct KrakenGateway {
}

pub struct KrakenHandler {
    //out:Sender,
    file_writer: FileWriter,
}
impl Handler for KrakenHandler {
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
impl MarketGateway for KrakenGateway {
    fn subscribe(&self,_se:CSender<String>) {//for linear perpetual contracts
        loop {
            let url = "wss://ws.kraken.com";
            if let Err(error) = connect(url, |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("Kraken WebSocket connected on {:?}",Instant::now());

                let sub_cmd = SubCmd {
                    event:"subscribe".to_string(),
                    reqid:10075,
                    pair: vec!["XBT/USD".to_string()],
                    subscription: json!({ "depth": 500,"name":"book" })
                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Kraken Client has sent message Subscribe cmd ")
                }

                let sub_cmd = SubCmd {
                    event:"subscribe".to_string(),
                    reqid:10075,
                    pair: vec!["XBT/USD".to_string()],
                    subscription: json!({"name":"trade"})
                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Kraken Client has sent message Subscribe cmd ")
                }

                // this can get indexPrice
                let sub_cmd = SubCmd {
                    event:"subscribe".to_string(),
                    reqid:10075,
                    pair: vec!["XBT/USD".to_string()],
                    subscription: json!({"name":"ticker"})
                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Kraken Client has sent message Subscribe cmd ")
                }


                KrakenHandler {
                    //out,
                    file_writer: FileWriter::new("KrakenSubscriber".to_string()),
                }
            }) {
                // Inform the user of failure
                println!("Failed to create Phemex WebSocket due to: {:?}", error);
            }
            println!("Kraken Subscriber disconnected");
            let millis = time::Duration::from_millis(1000 * 5);
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
        let mut kraken_gateway = KrakenGateway {
        };

    }
}
