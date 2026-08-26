include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/configuration/client.rs"
));

use financial_api::{FundType, Thscode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = from_env()?;
    let fund = Thscode::new("025480.OF")?;
    let response = client
        .fund_holders_top(FundType::Otc, &fund, Some(10))
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
