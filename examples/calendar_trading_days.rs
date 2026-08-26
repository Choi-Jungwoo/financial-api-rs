include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/configuration/client.rs"
));

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = from_env()?;
    let response = client.calendar_trading_days().await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
