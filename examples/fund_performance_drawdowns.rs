#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{FundType, Thscode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let fund = Thscode::new("025480.OF")?;
    let response = client
        .fund_performance_drawdowns(FundType::Otc, &fund)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
