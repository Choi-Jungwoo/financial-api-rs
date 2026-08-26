#[path = "configuration/client.rs"]
mod example_client;

use financial_api::AnomalyTag;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let response = client
        .special_data_anomaly_analysis_list(&[AnomalyTag::LimitUp])
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
