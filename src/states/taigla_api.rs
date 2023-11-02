
pub struct TaiglaApi {
    client: reqwest::Client,
    address: String,
    token: String
}

impl TaiglaApi {
    pub fn new(address: &str, token: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            address: address.to_string(),
            token: token.to_string()
        }
    }

    pub fn get(&self, url: &str) -> reqwest::RequestBuilder {
        self.client
            .get(format!("{}{}", self.address, url))
            .header("Authorization", &self.token)
    }
}
