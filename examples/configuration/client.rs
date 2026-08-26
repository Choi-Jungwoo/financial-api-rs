use std::io::ErrorKind;

use financial_api::Client;

pub fn from_env() -> Result<Client, Box<dyn std::error::Error>> {
    match dotenv::dotenv() {
        Ok(_) => {}
        Err(dotenv::Error::Io(error)) if error.kind() == ErrorKind::NotFound => {}
        Err(error) => return Err(error.into()),
    }

    Ok(Client::from_env()?)
}
