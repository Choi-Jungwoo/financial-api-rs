#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{AssetType, TickerListRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let request = TickerListRequest::new().asset_types([AssetType::AShare]);
    let response = client.tickers_list(&request).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
