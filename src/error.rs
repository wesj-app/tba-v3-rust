use reqwest;
use serde_json;
use crate::error::Error::{TBA, Reqwest};

#[derive(Debug)]
pub enum Error {
    TBA(u32, String),
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        match e.status() {
            Some(sc) => match sc.as_u16() {
                401 => TBA(401, "X-TBA-Auth-Key invalid".to_owned()),
                404 => TBA(404, "Endpoint not found".to_owned()),
                _ => Reqwest(e)
            }
            _ => Reqwest(e)
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use crate::Error;
    use crate::apis::TBAApiClient;
    use crate::configuration::Configuration;

    #[tokio::test]
    async fn should_handle_401() {
        let config = Configuration::from_api_key("invalidkey".to_owned());
        let api = TBAApiClient::new(Rc::new(config));
        let result = api.get_status(None).await;
        println!("{:?}", result);

        match result {
            Ok(_) => panic!("Request should not have succeeded!"),
            Err(Error::TBA(401, _)) => println!("OK"),
            Err(_) => panic!("Unexpected error type!")
        }
    }
}