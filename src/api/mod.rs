use reqwest::Url;
use serde::{Deserialize, de::DeserializeOwned, Serialize};

pub use indexer::*;
pub use user::*;
pub use job::*;
pub use invite::*;
pub use token::Token;

mod token;
mod indexer;
mod user;
mod job;
mod invite;

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
        if status.is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| {
                    ApiError::new("SerdeJsonParseError", &format!("{:?}", e))
                })
        } else {
            let err = response
                .json::<ApiError>()
                .await
                .map_err(|e| {
                    ApiError::new("SerdeJsonParseError", &format!("{:?}", e))
                })?;
            Err(err)
        }
    }

    pub async fn post<T: DeserializeOwned, U: Serialize>(&self, url: &str, body: &U) -> Result<T, ApiError> {
        let response = self.client
            .post(self.address.join(url).expect("Invalid url"))
            .header("Authorization", self.token.get())
            .json(body)
            .send()
            .await
            .map_err(|e| {
                ApiError::new("ReqwestError", &format!("{:?}", e))
            })?;
        let status = response.status();
        if status.is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| {
                    ApiError::new("SerdeJsonParseError", &format!("{:?}", e))
                })
        } else {
            let err = response
                .json::<ApiError>()
                .await
                .map_err(|e| {
                    ApiError::new("SerdeJsonParseError", &format!("{:?}", e))
                })?;
            Err(err)
        }
    }

    pub async fn patch<T: DeserializeOwned, U: Serialize>(&self, url: &str, body: &U) -> Result<T, ApiError> {
        let response = self.client
            .patch(self.address.join(url).expect("Invalid url"))
            .header("Authorization", self.token.get())
            .json(body)
            .send()
            .await
            .map_err(|e| {
                ApiError::new("ReqwestError", &format!("{:?}", e))
            })?;
        let status = response.status();
        if status.is_success() {
            response
                .json::<T>()
                .await
                .map_err(|e| {
                    ApiError::new("SerdeJsonParseError", &format!("{:?}", e))
                })
        } else {
            let err = response
                .json::<ApiError>()
                .await
                .map_err(|e| {
                    ApiError::new("SerdeJsonParseError", &format!("{:?}", e))
                })?;
            Err(err)
        }
    }
}
