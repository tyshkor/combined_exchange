use tokio::sync::mpsc::{self, UnboundedSender, UnboundedReceiver};


use crate::orderbook::orderbook_aggregator_server::{OrderbookAggregator, OrderbookAggregatorServer};
use crate::orderbook::{Empty, Summary, Level};

use crate::common_models::ChannelMessage;



pub async fn logic(mut rx: UnboundedReceiver<ChannelMessage>, mut tx: UnboundedSender<Summary>) {
    
}