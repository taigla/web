use std::{pin::Pin, future::Future};

use reqwest::Url;
use crate::hooks::Fetcher;

pub struct TaiglaApi {
    client: reqwest::Client,
    address: Url,
    token: String
}

impl TaiglaApi {
    pub fn new(address: &str, token: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            address: Url::parse(address).expect("Invalid url"),
            token: token.to_string()
        }
    }

    pub fn get(&self, url: &str) -> reqwest::RequestBuilder {
        self.client
            .get(self.address.join(url).expect("Invalid url"))
            .header("Authorization", &self.token)
    }

    pub fn post(&self, url: &str) -> reqwest::RequestBuilder {
        self.client
            .post(self.address.join(url).expect("Invalid url"))
            .header("Authorization", &self.token)
    }

    pub fn patch(&self, url: &str) -> reqwest::RequestBuilder {
        self.client
            .patch(self.address.join(url).expect("Invalid url"))
            .header("Authorization", &self.token)
    }
}

impl Fetcher for TaiglaApi {
    fn get(&self, url: &str) -> Pin<Box<dyn Future<Output = serde_json::Value>>> {
        let client = self.client.clone();
        let url = self.address.join(url).expect("Invalid url");
        Box::pin(async move {
            client.get(url)
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap()
        })
    }
}
