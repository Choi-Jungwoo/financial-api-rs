#[path = "configuration/client.rs"]
mod example_client;

use financial_api::AShareCode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let targets = [AShareCode::new("600519.SH")?];
    let response = client.a_share_valuations_snapshot(&targets).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
