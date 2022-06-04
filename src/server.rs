use std::sync::mpsc::Receiver;

use tonic::{transport::Server, Request, Response, Status};
use tokio_stream::wrappers::ReceiverStream;
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};

use futures_util::{future, pin_mut, StreamExt, SinkExt};


use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use combined_exchange::{
    binance_models::BinanceResponse, 
    bitstamp_models::{BitstampRequest, BitstampRequestData, BitstampResponse}, 
    common_models::ChannelMessage,
    exchanges::{binance, bitstamp},
    grpc::grpc,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let (mut tx, rx) = mpsc::unbounded_channel::<ChannelMessage>();

    let mut handles = vec![];

    let binance_tx = tx.clone();
    let bitstamp_tx = tx.clone();

    handles.push(tokio::spawn(async { binance(binance_tx).await }));
    handles.push(tokio::spawn(async { bitstamp(bitstamp_tx).await }));
    handles.push(tokio::spawn(async move{ grpc(rx).await }));

    future::join_all(handles).await;

    Ok(())
}
