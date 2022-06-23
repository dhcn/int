use crate::gateways::traits::MarketGateway;
use crate::utils::file::FileWriter;
use crate::utils::time::time_interval;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use std::{thread, time};
use ws::{connect, Handler, Message, Result};
use crate::utils::decode::dedecode_reader;
use crossbeam_channel::Sender;

#[derive(Serialize, Deserialize)]
struct ReqCmd {
    op: String,
    args: Vec<String>,
}

pub struct OkexGateway {
    pub address: String,
}

pub struct OkexSubscribeHandler {
    file_writer: FileWriter,
    #[allow(dead_code)]
    se:Sender<String>
}
impl Handler for OkexSubscribeHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        //println!("Received message  on {:?}::",Instant::now());

        match msg {
            Message::Text(ref string) => println!("Okex Text:{}", string.as_str()),
            Message::Binary(ref data) => {
                //println!("Binary:{:?}",String::from_utf8(inflate_bytes(data).unwrap()).unwrap().as_str());
                //println!("get message!");
                let text =dedecode_reader(data.to_vec()).unwrap();
                //self.se.send(text.clone());
                self.file_writer
                    .append_data(text.as_bytes())
            }
        }
        Ok(())
    }
}
impl MarketGateway for OkexGateway {
    fn subscribe(&self,se:Sender<String>) {
        loop {
            let se1 =se.clone();
            if let Err(error) = connect(self.address.as_str(), |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("okex connected WebSocket on {:?}",Instant::now());
                let mut subscribe_cmd = ReqCmd {
                    op: String::from("subscribe"),
                    args: Vec::new(),
                };
                subscribe_cmd
                    .args
                    .push(String::from("spot/depth_l2_tbt:BTC-USDT")); //depth400
                subscribe_cmd
                    .args
                    .push(String::from("spot/ticker:BTC-USDT")); //ticker data
                subscribe_cmd.args.push(String::from("spot/trade:BTC-USDT")); //trade data
                let cmd_str = serde_json::to_string(&subscribe_cmd).unwrap();
                println!("cmd:{}", cmd_str);
                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Okex Client has sent message Subscribe cmd ")
                }

                time_interval(25 * 1000, move || {
                    //println!("Okex ping....... ");
                    match out.ping(String::from("Ping").into_bytes()) {
                        Ok(()) => Ok(()),
                        Err(_error) => Err(()),
                    }
                });

                // The handler needs to take ownership of out, so we use move
                OkexSubscribeHandler {
                    file_writer: FileWriter::new("OkexSubscriber".to_string()),
                    se:se1.clone()//because a closure which implements Fn can be called multiple times
                }
            }) {
                // Inform the user of failure
                println!("Okex Failed to create WebSocket due to: {:?}", error);
            }
            println!("Okex Subscriber Disconnected");
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
        let mut okex_gateway = OkexGateway {
            address: String::from("wss://real.OKEx.com:8443/ws/v3"),
        };
        //okex_gateway.subscribe();
    }
}
