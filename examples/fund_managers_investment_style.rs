#[path = "configuration/client.rs"]
mod example_client;
#[path = "configuration/fund.rs"]
mod example_fund;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let manager_id = example_fund::manager_id(&client).await?;
    let response = client.fund_managers_investment_style(&manager_id).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
