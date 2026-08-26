#[path = "configuration/client.rs"]
mod example_client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let response = client.calendar_trading_days().await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
