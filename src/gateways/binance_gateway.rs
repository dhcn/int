use crate::gateways::traits::MarketGateway;
use crate::utils::file::FileWriter;
use serde::{Deserialize, Serialize};
use std::time::Instant;
use ws::{connect, Handler, Message, Result, Sender, Handshake};
use std::{thread, time};
use crate::utils::decode::dedecode_reader;
use crossbeam_channel::{ Sender as CSender};
use serde_json::Value;
use crate::utils::client::http_get;

#[derive(Serialize, Deserialize)]
struct SubCmd {
    method: String,
    params: Vec<String>,
    id:u64
}

pub struct BinanceGateway {
}

pub struct BinanceHandler {
    #[allow(dead_code)]
    out:Sender,
    file_writer: FileWriter,
    got_depth1000:bool,
    #[allow(dead_code)]
    se:CSender<String>
}
impl Handler for BinanceHandler {
    fn on_open(&mut self, _shake: Handshake) -> Result<()> {
        // just for record sub successful
        let mut subscribe_cmd = SubCmd {
                    method: String::from("SUBSCRIBE"),
                    params: Vec::new(),
                    id: 6
                };
        subscribe_cmd
            .params
            .push(String::from("Successful"));
        let cmd_str = serde_json::to_string(&subscribe_cmd).unwrap();
        self.file_writer
                    .append_data(cmd_str.as_bytes());
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        //println!("Received message  on {:?}::", Instant::now());

        match msg {
            Message::Text(ref string) => {
                //self.se.send(string.clone());
                self.file_writer
                    .append_data(string.as_bytes());
                if !self.got_depth1000{
                    let value:Value= serde_json::from_str(string).unwrap();
                    if value["stream"].is_string() && value["stream"].as_str().unwrap()=="btcusdt@depth@100ms"{
                        let res = http_get(String::from("https://fapi.binance.com/fapi/v1/depth?symbol=BTCUSDT&limit=1000"));
                        self.file_writer.append_data(res.as_slice());
                        //println!("{:?}",res.as_slice());
                        println!("http got_depth1000 successful");
                        self.got_depth1000=true;
                    }
                }
            }
            Message::Binary(ref data) => {
                println!(
                    "Binan Binary:{:?}",
                    dedecode_reader(data.to_vec()).unwrap()
                );
                //println!("get message!");
                //self.file_writer
                //    .append_data(inflate_bytes(data).unwrap().as_slice())
            }
        }
        Ok(())
    }

}
impl MarketGateway for BinanceGateway {
    fn subscribe(&self,se:CSender<String>) {
        loop {
            let url = "wss://stream.binance.com:9443/stream?streams=btcusdt@depth@100ms/btcusdt@aggTrade/btcusdt@ticker";
            if let Err(error) = connect(url, |out| {
                // Queue a message to be sent when the WebSocket is open
                println!("Binance WebSocket connected on {:?}",Instant::now());
                let mut subscribe_cmd = SubCmd {
                    method: String::from("SUBSCRIBE"),
                    params: Vec::new(),
                    id: 7
                };
                subscribe_cmd
                    .params
                    .push(String::from("btcusdt@depth@100ms")); //depth400
                subscribe_cmd
                    .params
                    .push(String::from("btcusdt@aggTrade")); //ticker data
                subscribe_cmd.params.push(String::from("btcusdt@ticker")); //trade data
                let cmd_str = serde_json::to_string(&subscribe_cmd).unwrap();
                println!("Binance SubCmd str:{}", cmd_str);
                if out.send(cmd_str).is_err() {
                    println!("Binnan Websocket couldn't submit subcmd message.");
                } else {
                    println!("Binnan  Client has sent message Subscribe cmd ");
                }
                BinanceHandler {
                    out,
                    file_writer: FileWriter::new("BinanSubscriber".to_string()),
                    got_depth1000:false,
                    se:se.clone(),//because a closure which implements Fn can be called multiple times
                }
            }) {
                // Inform the user of failure
                println!("Failed to create Binan WebSocket due to: {:?}", error);
            }
            println!("Binan Subscriber disconnected");
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
        let mut bian_gateway = BinanceGateway {
        };
        //bian_gateway.subscribe();
    }
}
