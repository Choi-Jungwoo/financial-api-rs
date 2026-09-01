use financial_api::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .special_data_anomaly_analysis_stock(["600519.SH"])
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
