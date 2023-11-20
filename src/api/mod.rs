use reqwest::Url;
use serde::{Deserialize, de::DeserializeOwned};
use crate::states::Token;

pub use indexer::*;
pub use user::*;

mod indexer;
mod user;

#[derive(Deserialize, Debug, Clone)]
pub struct ApiError {
    pub err_code: String,
    pub description: String
}

impl ApiError {
    pub fn new(err_code: &str, description: &str) -> Self {
        Self {
            err_code: err_code.to_string(),
            description: description.to_string()
        }
    }
}

#[derive(Clone)]
pub struct TaiglaApi {
    client: reqwest::Client,
    address: Url,
    token: Token
}

impl TaiglaApi {
    pub fn new(address: &str, token: Token) -> Self {
        Self {
            client: reqwest::Client::new(),
            address: Url::parse(address).expect("Invalid url"),
            token
        }
    }

    pub async fn get<T: DeserializeOwned>(&self, url: &str) -> Result<T, ApiError> {
        let response = self.client
            .get(self.address.join(url).expect("Invalid url"))
            .header("Authorization", self.token.get())
            .send()
            .await
            .map_err(|e| {
                ApiError::new("ReqwestError", &format!("{:?}", e))
            })?;
        let status = response.status();
        let value = response
            .json::<T>()
            .await
            .map_err(|e| {
                ApiError::new("SerdeJsonParseError", &format!("{:?}", e))
            })?;
        if status.is_success() {
            Ok(value)
        } else {
            Ok(value)
        }
    }

    pub fn post(&self, url: &str) -> reqwest::RequestBuilder {
        self.client
            .post(self.address.join(url).expect("Invalid url"))
            .header("Authorization", self.token.get())
    }

    pub fn patch(&self, url: &str) -> reqwest::RequestBuilder {
        self.client
            .patch(self.address.join(url).expect("Invalid url"))
            .header("Authorization", self.token.get())
    }
}
