#[path = "configuration/client.rs"]
mod example_client;

use financial_api::Thscode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let fund = Thscode::new("510300.SH")?;
    let response = client.fund_market_snapshot(&fund).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
