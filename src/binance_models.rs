use serde::{Deserialize, Serialize};
use super::common_models::Price;

#[derive(Serialize, Deserialize, Debug)]
pub struct BinanceResponse {
    pub lastUpdateId: u64,
    pub bids: Vec<Price>,
    pub asks: Vec<Price>,
}

