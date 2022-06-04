pub mod binance_models;
pub mod bitstamp_models;
pub mod common_models;
pub mod exchanges;
pub mod grpc;
pub mod logic;
// Import the generated proto-rust file into a module
pub mod orderbook {
    tonic::include_proto!("orderbook");
}