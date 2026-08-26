#[path = "configuration/client.rs"]
mod example_client;

use financial_api::HotListPeriod;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let response = client
        .special_data_hot_stock_list(HotListPeriod::Day)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
