use financial_api::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = Client::from_env()?;
    // This identifier is returned by fund_profile_detail for 025480.OF.
    let response = client.fund_companies_detail("00079099").await?;

    println!("request_id={}", response.request_id());
    println!("{:#?}", response.data());
    Ok(())
}
