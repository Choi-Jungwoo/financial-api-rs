#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{DragonTigerBoard, NaturalDate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let date = NaturalDate::parse("2026-08-25")?;
    let response = client
        .special_data_dragon_tiger_list(DragonTigerBoard::All, Some(date))
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
