#[path = "configuration/client.rs"]
mod example_client;

use financial_api::CompanyId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = example_client::from_env()?;
    let company_id = CompanyId::new("80000222")?;
    let response = client.fund_companies_detail(&company_id).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
