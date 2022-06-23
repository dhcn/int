use crate::gateways::traits::MarketGateway;
use crate::utils::file::FileWriter;
use std::time::Instant;
use ws::{connect, Handler, Message, Result, Sender};
use std::{thread, time};
use crate::utils::decode::dedecode_reader;
use serde_json::{Value, Map};
use crossbeam_channel::{Sender as CSender};



pub struct CoinbaseGateway {
}

pub struct CoinbaseHandler {
    #[allow(dead_code)]
    out:Sender,
    file_writer: FileWriter,
}
impl Handler for CoinbaseHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        //println!("Received message  on {:?}::", Instant::now());

        match msg {
            Message::Text(ref string) => {
                self.file_writer
                    .append_data(string.as_bytes());
            }
            Message::Binary(ref data) => {
                println!(
                    "Coinbase Binary:{:?}",
                    dedecode_reader(data.to_vec()).unwrap()
                );
            }
        }
        Ok(())
    }

}
impl MarketGateway for CoinbaseGateway {
    fn subscribe(&self,_se:CSender<String>) {
        loop {
            let url = "wss://ws-feed.pro.coinbase.com";
            if let Err(error) = connect(url, |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("Coinbase WebSocket connected on {:?}",Instant::now());

                let mut sub_cmd = Map::new();
                sub_cmd.insert("type".to_string(),Value::String("subscribe".to_string()));

                let mut product_ids:Vec<Value> = Vec::new();
                product_ids.push(Value::String("BTC-USD".to_string()));
                sub_cmd.insert("product_ids".to_string(),Value::Array(product_ids));

                let mut channels:Vec<Value> = Vec::new();
                channels.push(Value::String("level2".to_string()));
                channels.push(Value::String("ticker".to_string()));
                channels.push(Value::String("matches".to_string()));
                sub_cmd.insert("channels".to_string(),Value::Array(channels));

                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Coinbase Client has sent message Subscribe cmd ")
                }


                CoinbaseHandler {
                    out,
                    file_writer: FileWriter::new("CoinbaseSubscriber".to_string()),
                }
            }) {
                // Inform the user of failure
                println!("Failed to create Coinbase WebSocket due to: {:?}", error);
            }
            println!("Coinbase Subscriber disconnected");
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
        let mut cb_gateway = CoinbaseGateway {
        };
        //cb_gateway.subscribe();
    }
}
