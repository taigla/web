use reqwest::Url;

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
