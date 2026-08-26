#[path = "configuration/client.rs"]
mod example_client;

use financial_api::Thscode;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let indices = [Thscode::new("000300.SH")?, Thscode::new("399006.SZ")?];
    let response = client.index_prices_snapshot(&indices).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
