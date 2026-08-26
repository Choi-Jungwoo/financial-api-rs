include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/configuration/client.rs"
));

use financial_api::{AShareCode, NaturalDate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = from_env()?;
    let target = AShareCode::new("600519.SH")?;
    let start = NaturalDate::parse("2026-08-01")?;
    let end = NaturalDate::parse("2026-08-25")?;
    let response = client
        .special_data_hot_stock_rank_trend(&target, start, end)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
