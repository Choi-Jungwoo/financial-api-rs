#[path = "configuration/client.rs"]
mod example_client;

use financial_api::{ManagerId, ManagerPerformanceRange};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    // Replace this with a manager_id returned by fund_profile_detail.
    let manager_id = ManagerId::new("manager-001")?;
    let response = client
        .fund_managers_performance(&manager_id, ManagerPerformanceRange::Month)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
