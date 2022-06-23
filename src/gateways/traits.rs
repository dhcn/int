use crossbeam_channel::Sender;

pub trait MarketGateway {
    fn subscribe(&self,se:Sender<String>);
}
pub trait TransactionGateway {
    fn send_order(&self);
}

