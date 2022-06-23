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
    tag: String,
    args: Vec<String>,
}


pub struct CoinFlexGateway {
}

pub struct CoinFlexHandler {
    //out:Sender,
    file_writer: FileWriter,
}
impl Handler for CoinFlexHandler {
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
impl MarketGateway for CoinFlexGateway {
    fn subscribe(&self,_se:CSender<String>) {//for linear perpetual contracts
        loop {
            let url = "wss://v2api.coinflex.com/v2/websocket";
            if let Err(error) = connect(url, |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("CoinFlex WebSocket connected on {:?}",Instant::now());

                let sub_cmd = SubCmd {
                    op:"subscribe".to_string(),
                    tag:"orderbook.ethusd".to_string(),
                    args: vec!["depthL25:ETH-USD-SWAP-LIN".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("CoinFlex Client has sent message Subscribe cmd ")
                }
                //Verified fact: The snapshot and update of this book channel is like Okex book channel
                let sub_cmd = SubCmd {
                    op:"subscribe".to_string(),
                    tag:"trade.ethusd".to_string(),
                    args: vec!["trade:ETH-USD-SWAP-LIN".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("CoinFlex Client has sent message Subscribe cmd ")
                }

                // this can get indexPrice
                let sub_cmd = SubCmd {
                    op:"subscribe".to_string(),
                    tag:"market.ethusd".to_string(),
                    args: vec!["market:ETH-USD-SWAP-LIN".to_string()],

                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("CoinFlex Client has sent message Subscribe cmd ")
                }




                CoinFlexHandler {
                    //out,
                    file_writer: FileWriter::new("CoinFlexeSubscriber".to_string()),
                }
            }) {
                // Inform the user of failure
                println!("Failed to create CoinFlex WebSocket due to: {:?}", error);
            }
            println!("CoinFlex Subscriber disconnected");
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
        let mut coinflex_gateway = CoinFlexGateway {
        };

    }
}
