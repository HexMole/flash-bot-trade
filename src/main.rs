use ethers::{
    prelude::*,
    types::transaction::{eip2718::TypedTransaction, eip2930::AccessList},
};
use ethers_flashbots::FlashbotsMiddleware;
use eyre::Result;
use reqwest::Url;
use std::time::{SystemTime, UNIX_EPOCH};
use dotenv::dotenv;
use trade_bot::utils::*;
use console::style;
// use subway_rs::{abi, banner, numeric, relayer, telemetry, uniswap, utils};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok(); 
    // Get the http provider for flashbots use
    let http_provider = get_http_provider()?;

    // Create the websocket clieant
    let client = create_websocket_client().await?;

    // Get the latest block
    let last_block = client
        .get_block(BlockNumber::Latest)
        .await?
        .unwrap()
        .number
        .unwrap();
    println!("[START] Sandwich bot initializing on block {}", style(last_block).cyan());

    Ok(())

}