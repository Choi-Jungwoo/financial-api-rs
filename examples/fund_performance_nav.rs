include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/configuration/client.rs"
));

use financial_api::{FundNavType, FundRange, FundType, Thscode};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = from_env()?;
    let fund = Thscode::new("025480.OF")?;
    let response = client
        .fund_performance_nav(
            FundType::Otc,
            &fund,
            Some(FundRange::Month),
            FundNavType::Unit,
        )
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
