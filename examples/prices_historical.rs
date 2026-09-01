use financial_api::{Adjustment, Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .prices_historical(
            "600519.SH",
            1_716_105_600_000,
            1_716_192_000_000,
            Adjustment::None,
            0,
        )
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
