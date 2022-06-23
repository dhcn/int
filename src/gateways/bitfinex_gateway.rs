use crate::gateways::traits::MarketGateway;
use crate::utils::file::FileWriter;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use ws::{connect, Handler, Message, Result};
use std::{thread, time};
use crate::utils::decode::dedecode_reader;
use crossbeam_channel::{Sender as CSender};

#[derive(Serialize, Deserialize)]
struct SubCmd {
    event: String,
    channel: String,
    symbol:String,

}
#[derive(Serialize, Deserialize)]
struct BookSubCmd {
    event: String,
    channel: String,
    symbol:String,
    prec: String,
    len:u64,
}

pub struct BitfinexGateway {
}

pub struct BitfinexHandler {
    //out:Sender,
    file_writer: FileWriter,
    #[allow(dead_code)]
    se:CSender<String>
}
impl Handler for BitfinexHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        //println!("Received message  on {:?}::", Instant::now());

        match msg {
            Message::Text(ref string) => {
                //self.se.send(string.clone());
                self.file_writer
                    .append_data(string.as_bytes());
            }
            Message::Binary(ref data) => {
                println!(
                    "Bitfinex Binary:{:?}",
                    dedecode_reader(data.to_vec()).unwrap()
                );
            }
        }
        Ok(())
    }

}
impl MarketGateway for BitfinexGateway {
    fn subscribe(&self,se:CSender<String>) {
        loop {
            let se1 =se.clone();
            let url = "wss://api-pub.bitfinex.com/ws/2";
            if let Err(error) = connect(url, |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("Bitfinex WebSocket connected on {:?}",Instant::now());

                let sub_cmd = SubCmd {
                    event: "subscribe".to_string(),
                    channel: "trades".to_string(),
                    symbol:"tBTCUSD".to_string(),

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bitfinex Client has sent message Subscribe cmd ")
                }
                //Verified fact: The snapshot and update of this book channel is like Okex book channel
                let sub_cmd = BookSubCmd {
                    event: "subscribe".to_string(),
                    channel: "book".to_string(),
                    symbol:"tBTCUSD".to_string(),
                    prec: "P0".to_string(),
                    len:250,
                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bitfinex Client has sent message Subscribe cmd ")
                }
                let sub_cmd = BookSubCmd {
                    event: "subscribe".to_string(),
                    channel: "book".to_string(),
                    symbol:"tBTCUSD".to_string(),
                    prec: "R0".to_string(),
                    len:250,
                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bitfinex Client has sent message Subscribe cmd ")
                }

                BitfinexHandler {
                    //out,
                    file_writer: FileWriter::new("BitfinexSubscriber".to_string()),
                    se:se1.clone()//because a closure which implements Fn can be called multiple times
                }
            }) {
                // Inform the user of failure
                println!("Failed to create Bitfinex WebSocket due to: {:?}", error);
            }
            println!("Bitfinex Subscriber disconnected");
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
        let mut bitfinex_gateway = BitfinexGateway {
        };
        //bitfinex_gateway.subscribe();
    }
}
