include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/examples/configuration/client.rs"
));

use financial_api::CompanyId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = from_env()?;
    // This identifier is returned by fund_profile_detail for 025480.OF.
    let company_id = CompanyId::new("00079099")?;
    let response = client.fund_companies_detail(&company_id).await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
