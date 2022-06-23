#![feature(map_first_last)]
#[macro_use]
extern crate log;
use std::thread;
use std::thread::JoinHandle;

use crossbeam_channel::{Sender, Receiver, unbounded};

use int::ats::bybit_bts::{BybitBackTestSystem};
use int::ats::traits::TradingSystem;
use int::gateways::binance_gateway::BinanceGateway;
use int::gateways::bitfinex_gateway::BitfinexGateway;
use int::gateways::bitmex_gateway::BitmexGateway;
use int::gateways::coinbase_gateway::CoinbaseGateway;
use int::gateways::huobi_gateway::HuobiGateway;
use int::gateways::okex_gateway::OkexGateway;
use int::gateways::traits::MarketGateway;

use int::gateways::converge_recorder::ConvergeRecorder;
use int::gateways::bybit_gateway::BybitGateway;
use int::gateways::phemex_gateway::PhemexGateway;
use int::gateways::kraken_gateway::KrakenGateway;
use int::gateways::bitstamp_gateway::BitstampGateway;
use int::gateways::coinflex_gateway::CoinFlexGateway;
use int::gateways::ftx_gateway::FTXGateway;

pub mod engine;
pub mod utils;

fn main() {
    //subscribe_run();
    bbts_run()//event backtest
}
#[allow(dead_code)]
fn bbts_run() {
    let mut bbts = BybitBackTestSystem {};
    bbts.run();
}



fn subscribe_run() {
    env_logger::init();
    info!("subscriber starting up");
    let (converge_se, converge_rec ) = unbounded();
    let con_handle=converge_subscribe(converge_rec);
    let coinbase_handle = coinbase_subscribe(converge_se.clone());
    //let ok_handle = ok_subscribe(converge_se.clone());
    let huobi_handle = huobi_subcribe(converge_se.clone());
    let binan_handle = binan_subscribe(converge_se.clone());
    //let bitfinex_handle = bitfinex_subscribe(converge_se.clone());
    let bybit_handle = bybit_subscribe(converge_se.clone());
    let bybit_ihandle = bybit_isubscribe();
    let bybit_ehandle = bybit_esubscribe();
    let phemex_handle = phemex_subscribe(converge_se.clone());
    let phemex_ehandle = phemex_esubscribe(converge_se.clone());
    let bitmex_handle = bitmex_subscribe(converge_se.clone());
    let kraken_handle = kraken_subscribe(converge_se.clone());
    let bitstamp_handle = bitstamp_subscribe(converge_se.clone());
    let coinflex_ehandle = coinflex_esubscribe(converge_se.clone());
    let ftx_handle = ftx_subscribe(converge_se.clone());

    let _ = coinbase_handle.join().unwrap();
    //let _ = ok_handle.join().unwrap();
    let _ = huobi_handle.join().unwrap();
    let _ = binan_handle.join().unwrap();
    //let _ = bitfinex_handle.join().unwrap();
    let _ = bybit_handle.join().unwrap();
    let _ = bybit_ihandle.join().unwrap();
    let _ = bybit_ehandle.join().unwrap();
    let _ = phemex_handle.join().unwrap();
    let _ = phemex_ehandle.join().unwrap();
    let _ = bitmex_handle.join().unwrap();
    let _ = kraken_handle.join().unwrap();
    let _ = bitstamp_handle.join().unwrap();
    let _ = coinflex_ehandle.join().unwrap();
    let _ = ftx_handle.join().unwrap();

    let _ = con_handle.join().unwrap();

}

#[allow(dead_code)]
fn ok_subscribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let okex_gateway: OkexGateway = OkexGateway {
            address: "wss://real.OKEx.com:8443/ws/v3".to_string(),
        };
        okex_gateway.subscribe(se);
    })
}

fn binan_subscribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let binan_gateway: BinanceGateway = BinanceGateway {};
        binan_gateway.subscribe(se);
    })
}

fn huobi_subcribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let huobi_gateway: HuobiGateway = HuobiGateway {
            address: String::from("wss://api.huobi.pro/ws"),
        };
        huobi_gateway.subscribe(se);
    })
}

fn bitmex_subscribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let bitemex_gateway: BitmexGateway = BitmexGateway {};
        bitemex_gateway.subscribe(se);
    })
}
fn bybit_subscribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let bybit_gateway: BybitGateway = BybitGateway {};
        bybit_gateway.subscribe(se);
    })
}

fn bybit_isubscribe() -> JoinHandle<()> {
    thread::spawn(move || {
        let bybit_gateway: BybitGateway = BybitGateway {};
        bybit_gateway.isubscribe();
    })
}

fn bybit_esubscribe() -> JoinHandle<()> {
    thread::spawn(move || {
        let bybit_gateway: BybitGateway = BybitGateway {};
        bybit_gateway.esubscribe();
    })
}

fn phemex_subscribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let phemex_gateway:PhemexGateway = PhemexGateway {};
        phemex_gateway.subscribe(se);
    })
}
fn phemex_esubscribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let phemex_gateway:PhemexGateway = PhemexGateway {};
        phemex_gateway.esubscribe(se);
    })
}
#[allow(dead_code)]
fn bitfinex_subscribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let bitfinex_gateway = BitfinexGateway {};
        bitfinex_gateway.subscribe(se)
    })
}

fn coinbase_subscribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let coinbase_gateway = CoinbaseGateway {};
        coinbase_gateway.subscribe(se);
    })
}
fn kraken_subscribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let kraken_gateway = KrakenGateway {};
        kraken_gateway.subscribe(se);
    })
}
fn bitstamp_subscribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let bitstamp_gateway = BitstampGateway {};
        bitstamp_gateway.subscribe(se);
    })
}
fn coinflex_esubscribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let coinflex_gateway = CoinFlexGateway {};
        coinflex_gateway.subscribe(se);
    })
}
fn ftx_subscribe(se:Sender<String>) -> JoinHandle<()> {
    thread::spawn(move || {
        let ftx_gateway = FTXGateway {};
        ftx_gateway.subscribe(se);
    })
}
fn converge_subscribe(rec:Receiver<String>)->JoinHandle<()>{
    thread::spawn(move || {
        let mut converge_recorder = ConvergeRecorder::new(rec);
        converge_recorder.run();
    })
}



