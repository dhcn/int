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
    data:Value,
}


pub struct BitstampGateway {
}

pub struct BitstampHandler {
    //out:Sender,
    file_writer: FileWriter,
}
impl Handler for BitstampHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {

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
impl MarketGateway for BitstampGateway {
    fn subscribe(&self,_se:CSender<String>) {//for linear perpetual contracts
        loop {
            let url = "wss://ws.bitstamp.net";
            if let Err(error) = connect(url, |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("Bitstamp WebSocket connected on {:?}",Instant::now());

                let sub_cmd = SubCmd {
                    event:"bts:subscribe".to_string(),
                    data:json!({"channel":"order_book_btcusd"})//这个订阅是非增量快照订阅，
                    // 虽然订阅数据量较大，但压缩后数据量并不大
                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();
                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bitstamp Client has sent message Subscribe cmd ")
                }

                let sub_cmd = SubCmd {
                    event:"bts:subscribe".to_string(),
                    data:json!({"channel":"live_trades_btcusd"})
                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Bitstamp Client has sent message Subscribe cmd ")
                }

                BitstampHandler {
                    //out,
                    file_writer: FileWriter::new("BitstampSubscriber".to_string()),
                }
            }) {
                // Inform the user of failure
                println!("Failed to create Phemex WebSocket due to: {:?}", error);
            }
            println!("Bitstamp Subscriber disconnected");
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
        let mut bitstamp_gateway = BitstampGateway {
        };

    }
}
