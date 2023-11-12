use serde::{Serialize, Deserialize};
use super::{TaiglaApi, ApiError};

#[derive(Serialize)]
pub struct LoginParam<'a> {
    pub username: &'a str,
    pub password: &'a str
}

#[derive(Deserialize)]
pub struct AuthResponse {
    pub token: String
}

impl TaiglaApi {
    pub async fn post_login<'a>(&self, credential: &LoginParam<'a>) -> Result<AuthResponse, ApiError> {
        self.post("/api/v1/auth/login", credential)
            .await
    }
}
