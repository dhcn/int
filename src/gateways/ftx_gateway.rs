use crate::gateways::traits::MarketGateway;
use crate::utils::file::FileWriter;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use ws::{connect, Handler, Message, Result};
use std::{thread, time};
use crate::utils::decode::dedecode_reader;
use crossbeam_channel::{ Sender as CSender};

#[derive(Serialize, Deserialize)]
struct SubCmd {
    op:String,
    channel: String,
    market: String,
}


pub struct FTXGateway {
}

pub struct FTXHandler {
    //out:Sender,
    file_writer: FileWriter,
}
impl Handler for FTXHandler {
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
impl MarketGateway for FTXGateway {
    fn subscribe(&self,_se:CSender<String>) {//for linear perpetual contracts
        loop {
            let url = "wss://ftx.com/ws/";
            if let Err(error) = connect(url, |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("FTX WebSocket connected on {:?}",Instant::now());

                let sub_cmd = SubCmd {
                    op:"subscribe".to_string(),
                    channel:"orderbook".to_string(),
                    market:"BTC/USD".to_string(),

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("FTX Client has sent message Subscribe cmd ")
                }
                //Verified fact: The snapshot and update of this book channel is like Okex book channel
                let sub_cmd = SubCmd {
                    op:"subscribe".to_string(),
                    channel:"trades".to_string(),
                    market:"BTC/USD".to_string(),

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("FTX Client has sent message Subscribe cmd ")
                }

                FTXHandler {
                    //out,
                    file_writer: FileWriter::new("FTXSubscriber".to_string()),
                }
            }) {
                // Inform the user of failure
                println!("Failed to create FTX WebSocket due to: {:?}", error);
            }
            println!("FTX Subscriber disconnected");
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
        let mut ftx_gateway = FTXGateway {
        };

    }
}
