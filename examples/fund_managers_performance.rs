include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/configuration/client.rs"
));
mod example_fund {
    include!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/examples/configuration/fund.rs"
    ));
}

use financial_api::ManagerPerformanceRange;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = from_env()?;
    let manager_id = example_fund::manager_id(&client).await?;
    let response = client
        .fund_managers_performance(&manager_id, ManagerPerformanceRange::Month)
        .await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
