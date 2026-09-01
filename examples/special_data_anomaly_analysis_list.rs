use financial_api::{AnomalyTag, Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    let response = client
        .special_data_anomaly_analysis_list(&[AnomalyTag::LimitUp])
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
