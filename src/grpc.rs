use std::sync::mpsc::Receiver;

use tonic::{transport::Server, Request, Response, Status};
use tokio_stream::wrappers::{ReceiverStream, UnboundedReceiverStream};
use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};

use futures_util::{future, pin_mut, StreamExt, SinkExt};


use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use orderbook::{Empty, Summary, Level};

use crate::common_models::ChannelMessage;
use crate::logic::logic;
use crate::{binance_models::BinanceResponse, bitstamp_models::{BitstampRequest, BitstampRequestData, BitstampResponse}};

// Import the generated proto-rust file into a module
pub mod orderbook {
    tonic::include_proto!("orderbook");
}

pub struct MyOrderbookAggregator {
    rx: UnboundedReceiver<ChannelMessage>
}

// Implement the service function(s) defined in the proto
// for the Greeter service (SayHello...)
#[tonic::async_trait]
impl OrderbookAggregator for MyOrderbookAggregator {

    type BookSummaryStream = UnboundedReceiverStream<Result<Summary, Status>>;

    async fn book_summary(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<Self::BookSummaryStream>, Status> {
        let (mut tx_u, rx_u) = mpsc::unbounded_channel::<Result<Summary, Status>>();

        // logic(rx).await;

        tokio::spawn(async move{
            loop {
                if let Some(msg) = self.rx.recv().await {
                tx_u.send(Ok(Summary {
                        spread: (1.47858 as f64),
                        bids: vec![],
                        asks: vec![],
                    }));
                }
            }
        });

        Ok(Response::new(UnboundedReceiverStream::new(rx_u)))
    }
}

pub async fn grpc(mut rx: UnboundedReceiver<ChannelMessage>) {
    let addr = "[::1]:10000".parse().unwrap();

    println!("RouteGuideServer listening on: {}", addr);

    let orderbook = MyOrderbookAggregator {
        rx
    };

    let svc = OrderbookAggregatorServer::new(orderbook);

    Server::builder().add_service(svc).serve(addr).await;
}
