#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{AShareCode, AuctionStage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let targets = [AShareCode::new("600519.SH")?];
    let response = client
        .a_share_auction_snapshot(&targets, AuctionStage::Final)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
