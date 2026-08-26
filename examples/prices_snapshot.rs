#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{AShareCode, PriceSnapshotSelection};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let selection = PriceSnapshotSelection::targets(vec![AShareCode::new("600519.SH")?])?;
    let response = client.prices_snapshot(&selection).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
