use std::sync::mpsc::Receiver;

use tonic::{transport::Server, Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};

use futures_util::{future, pin_mut, StreamExt, SinkExt};


use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use orderbook::{Empty, Summary, Level};

use crate::{binance_models::BinanceResponse, bitstamp_models::{BitstampRequest, BitstampRequestData, BitstampResponse}, common_models::ChannelMessage};

// Import the generated proto-rust file into a module
pub mod orderbook {
    tonic::include_proto!("orderbook");
}

pub async fn binance(mut tx: UnboundedSender<ChannelMessage>) {
    let connect_addr = "wss://stream.binance.com:9443/ws/ethbtc@depth20@100ms".to_string();

    let url = url::Url::parse(&connect_addr).unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    read.for_each(|message| async {
        if let Ok(data) = message.unwrap().into_text() {
            let parsed: BinanceResponse = serde_json::from_str(&data).expect("Can't parse to JSON");
            // println!("parsed is = {:?}", parsed);
            println!("parsed is = binance");
            let msg = ChannelMessage::Binance(parsed);
            tx.send(msg);
        }
    }).await;
}

pub async fn bitstamp(mut tx: UnboundedSender<ChannelMessage>) {
    let connect_addr = "wss://ws.bitstamp.net".to_string();

    let url = url::Url::parse(&connect_addr).unwrap();

    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (mut write, read) = ws_stream.split();

    let event =  "bts:subscribe".to_string();

    let channel = "diff_order_book_btcusd".to_string();

    let data = BitstampRequestData {
        channel
    };

    let request = BitstampRequest {
        event,
        data
    };

    let json_request = serde_json::to_string(&request).unwrap();

    write.send(Message::text(json_request)).await;

    read.skip(1).for_each(|message| async {
        if let Ok(data) = message.unwrap().into_text() {
            let parsed: BitstampResponse = serde_json::from_str(&data).expect("Can't parse to JSON");
            // println!("parsed is = {:?}", parsed);
            println!("parsed is = bitstamp");
            let msg = ChannelMessage::Bistamp(parsed.data);
            tx.send(msg);
        }
    }).await;
}
