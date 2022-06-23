use crate::gateways::traits::MarketGateway;
use crate::utils::file::FileWriter;
use crate::utils::decode::{gzdecode_reader};
use serde::{Deserialize, Serialize};
use std::time::Instant;
use ws::{connect, Handler, Message, Result, Sender};
use serde_json::{Value, Number};
use std::{thread, time};
use crossbeam_channel::{Sender as CSender};

#[derive(Serialize, Deserialize)]
struct PongCmd {
    pong: Number
}
#[derive(Serialize, Deserialize)]
struct ReqCmd {
    req :String,
    id: String,
}
#[derive(Serialize, Deserialize)]
struct SubCmd {
    sub: String,
    id: String,
}

pub struct HuobiGateway {
    pub address: String,
    //flag_sender:&'static CSender<&'static [u8]>,
    //data_receiver:&'static Receiver<&'static [u8]>
}

pub struct HuobiHandler {
    out:Sender,
    file_writer: FileWriter,
    #[allow(dead_code)]
    start_req_mbp: bool,
    #[allow(dead_code)]
    got_req_mbp: bool,
    #[allow(dead_code)]
    se:CSender<String>,
    //flag_sender: &'static  CSender<&'static [u8]>,
    //data_receiver: &'static  Receiver<&'static [u8]>
}
impl Handler for HuobiHandler{
    fn on_message(&mut self, msg: Message) -> Result<()> {
        //println!("Received message  on {:?}::", Instant::now());
        match msg {
            Message::Text(ref string) => println!("Huobi Text:{}", string.as_str()),
            Message::Binary(ref data) => {
                //println!("get Binary!");
                let text = gzdecode_reader(data.to_vec()).unwrap();
                //println!("Binary:{}",text);
                if text.starts_with("{\"ping\":"){
                    let v:Value = serde_json::from_str(text.as_str()).unwrap();
                    if v.as_object().unwrap().contains_key("ping"){
                        let pong_cmd = PongCmd {
                            pong: Number::from(v["ping"].as_u64().unwrap())
                        };
                        let cmd_str = serde_json::to_string(&pong_cmd).unwrap();
                        //println!("Huobi pong:{}",cmd_str);
                        if self.out.send(cmd_str).is_err() {
                            println!("Huobi pong error")
                        }
                    }
                }
                /*
                if !self.start_req_mbp{
                    self.start_req_mbp=true;
                    if text.starts_with("{\"ch\":\"market.btcusdt.mbp.150\""){

                    }
                }
                if self.start_req_mbp && !self.got_req_mbp{
                    if let Ok(r)=self.data_receiver.recv(){
                        self.got_req_mbp=true;
                        self.file_writer.append_data(r);
                    }
                }*/
                //self.se.send(text.clone());
                self.file_writer
                    .append_data(text.as_bytes())
            },
        }
        Ok(())
    }
}
impl MarketGateway for HuobiGateway {
    fn subscribe(&self,se:CSender<String>) {
        loop {
            let se1 = se.clone();
            if let Err(error) = connect(self.address.as_str(), move|out| {
                // Queue a message to be sent when the WebSocket is open
                println!("Huobi WebSocket connected on {:?}",Instant::now());
                let sub_cmd = SubCmd {
                    sub: String::from("market.btcusdt.bbo"),
                    id: "id1".to_string(),
                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Huobi Client has sent message Subscribe cmd market.btcusdt.bbo")
                }

                let sub_cmd = SubCmd {
                    sub: String::from("market.btcusdt.trade.detail"),
                    id: "id2".to_string(),
                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Huobi Client sent message Subscribe cmd market.btcusdt.trade.detail")
                }

                let sub_cmd = SubCmd {
                    sub: String::from("market.btcusdt.mbp.150"),
                    id: "id3".to_string(),
                };
                let cmd_str = serde_json::to_string(&sub_cmd).unwrap();

                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Huobi Client sent message Subscribe cmd market.btcusdt.mbp.150")
                }
                let req_cmd = ReqCmd {
                    req: String::from("market.btcusdt.mbp.150"),
                    id: "id4".to_string(),
                };
                let cmd_str = serde_json::to_string(&req_cmd).unwrap();
                if out.send(cmd_str).is_err() {
                    println!("Websocket couldn't queue an initial message.")
                } else {
                    println!("Huobi Client sent message Req cmd market.btcusdt.mbp.150")
                }
                //let flag_sender = &self.flag_sender;
                //let data_receiver = &self.data_receiver;
                HuobiHandler {
                    out,
                    file_writer: FileWriter::new("HuobiSubscriber".to_string()),
                    start_req_mbp:false,
                    got_req_mbp:false,
                    se:se1.clone(),//because a closure which implements Fn can be called multiple times
                    //flag_sender,
                    //data_receiver
                }
            }) {
                // Inform the user of failure
                println!("Failed to create WebSocket due to: {:?}", error);
            }
            println!("Huobi Subscriber disconnected");
            let millis = time::Duration::from_millis(1000 * 5);
            thread::sleep(millis);
        }
    }
}
/*
fn req_mbp(flag_receiver:Receiver<&'static [u8]>,data_sender:CSender<&'static [u8]>){
    thread::spawn(move || {
        if let Err(error) = connect("wss://api.huobi.pro/feed", |out| {
            // Queue a message to be sent when the WebSocket is open
            let req_cmd = ReqCmd {
                req: String::from("market.btcusdt.mbp.150"),
                id: "id4".to_string(),
            };
            let cmd_str = serde_json::to_string(&req_cmd).unwrap();
            if out.send(cmd_str).is_err() {
                println!("Websocket couldn't queue an initial message.")
            } else {
                println!("Huobi Client sent message Req cmd market.btcusdt.mbp.150")
            }

            // The handler needs to take ownership of out, so we use move
            move |msg| {
                // Handle messages received on this connection
                println!("Client got message '{}'. ", msg);
                match msg {
                    Message::Text(ref string) => println!("Huobi Text:{}", string.as_str()),
                    Message::Binary(ref data) => {
                        data_sender.send(data.);
                        out.close(CloseCode::Normal);
                    }
                }
                Ok(())

                // Close the connection

            }
        }) {
            // Inform the user of failure
            println!("Failed to create WebSocket due to: {:?}", error);
        }
    });

}*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::BorrowMut;

    #[test]
    fn test_okex_gateway() {
        let mut huobi_gateway = HuobiGateway {
            address: String::from("wss://real.OKEx.com:8443/ws/v3"),
        };
        //huobi_gateway.subscribe();
    }
}
