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
    method: String,
    params: Vec<String>,
    id:u64
}

pub struct BitmexGateway {
}

pub struct BitmexHandler {
    //out:Sender,
    file_writer: FileWriter,
}
impl Handler for BitmexHandler {
    fn on_message(&mut self, msg: Message) -> Result<()> {
        //println!("Received message  on {:?}::", Instant::now());

        match msg {
            Message::Text(ref string) => {
                self.file_writer
                    .append_data(string.as_bytes());
            }
            Message::Binary(ref data) => {
                println!(
                    "Bitmex Binary:{:?}",
                    dedecode_reader(data.to_vec()).unwrap()
                );
            }
        }
        Ok(())
    }

}
impl MarketGateway for BitmexGateway {
    fn subscribe(&self,_se:CSender<String>) {
        loop {
            //经确认:orderBookL2这个channel提供全量lob的信息
            let url = "wss://www.bitmex.com/realtime?subscribe=trade:XBTUSD,orderBookL2:XBTUSD";
            if let Err(error) = connect(url, |_out| {
                // Queue a message to be sent when the WebSocket is open
                println!("Bitmex WebSocket connected on {:?}",Instant::now());
                BitmexHandler {
                    //out,
                    file_writer: FileWriter::new("BitmexSubscriber".to_string()),
                }
            }) {
                // Inform the user of failure
                println!("Failed to create Bitmex WebSocket due to: {:?}", error);
            }
            println!("Bitmex Subscriber disconnected");
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
        let mut bitmex_gateway = BitmexGateway {
        };
        //bitmex_gateway.subscribe();
    }
}
