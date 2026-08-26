include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/configuration/client.rs"
));

use financial_api::{AShareCode, NaturalDate};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = from_env()?;
    let target = AShareCode::new("600519.SH")?;
    let from = NaturalDate::parse("2026-01-01")?;
    let to = NaturalDate::parse("2026-08-25")?;
    let response = client
        .corp_actions_adjustment_factors(&target, Some(from), Some(to))
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
