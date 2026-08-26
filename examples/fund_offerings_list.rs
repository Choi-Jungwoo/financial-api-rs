#[path = "configuration/client.rs"]
mod example_client;

use financial_api::OfferingStatus;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let response = client.fund_offerings_list(OfferingStatus::Active).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
