use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::binance_models::BinanceResponse;
use crate::bitstamp_models::BitstampResponseData;

#[derive(Serialize, Deserialize, Debug)]
pub struct Price {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub left: f64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub right: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ChannelMessage {
    Bistamp(BitstampResponseData),
    Binance(BinanceResponse),
}