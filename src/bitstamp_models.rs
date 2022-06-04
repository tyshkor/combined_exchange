use serde::{Deserialize, Serialize};
use super::common_models::Price;

#[derive(Serialize, Deserialize, Debug)]
pub struct BitstampRequest {
    pub event: String,
    pub data: BitstampRequestData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitstampRequestData {
    pub channel: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitstampResponse {
    pub event: String,
    pub data: BitstampResponseData,
    pub channel: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitstampResponseData {
    pub timestamp: String,
    pub microtimestamp: String,
    pub bids: Vec<Price>,
    pub asks: Vec<Price>,
}
