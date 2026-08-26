#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{FundType, HolderMergeScope, Thscode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let fund = Thscode::new("000037.OF")?;
    let response = client
        .fund_holders_detail(FundType::Otc, &fund, HolderMergeScope::All)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
