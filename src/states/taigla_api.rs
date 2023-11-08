use std::{pin::Pin, future::Future};
use reqwest::Url;
use serde::Deserialize;
use serde_json::Value;
use crate::hooks::{Fetcher, QueryError};
use crate::states::Token;

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub err_code: String,
    pub code: u16
}

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

    pub fn get(&self, url: &str) -> reqwest::RequestBuilder {
        self.client
            .get(self.address.join(url).expect("Invalid url"))
            .header("Authorization", self.token.get())
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

impl Fetcher for TaiglaApi {
    fn get(&self, url: &str) -> Pin<Box<dyn Future<Output = Result<Result<Value, Value>, QueryError>>>> {
        let client = self.client.clone();
        let url = self.address.join(url).expect("Invalid url");
        let token = self.token.get();

        Box::pin(async move {
            let response = client.get(url)
                .header("Authorization", token)
                .send()
                .await
                .map_err(|e| {
                    QueryError::new("ReqwestError", &format!("{:?}", e))
                })?;
            let status = response.status();
            let value = response
                .json::<serde_json::Value>()
                .await
                .map_err(|e| {
                    QueryError::new("SerdeJsonParseError", &format!("{:?}", e))
                })?;
            if status.is_success() {
                Ok(Ok(value))
            } else {
                Ok(Err(value))
            }
        })
    }
}
