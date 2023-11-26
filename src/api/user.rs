use super::{TaiglaApi, ApiError};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct UserRow {
    pub name: String,
    pub id: u64,
    pub disable: bool
}

#[derive(Serialize)]
pub struct ParamChangePassword<'a> {
    new_password: &'a str
}

impl TaiglaApi {
    pub async fn get_users(&self) -> Result<Vec<UserRow>, ApiError> {
        self.get::<Vec<UserRow>>("/api/v1/users")
            .await
    }

    pub async fn post_change_password(&self, new_password: &str) -> Result<(), ApiError> {
        self.post("/api/v1/users/me/change-password", &ParamChangePassword { new_password: new_password })
            .await
    }
}
