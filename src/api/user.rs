use super::{TaiglaApi, ApiError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserRow {
    pub name: String,
    pub id: u64,
    pub disable: bool
}

impl TaiglaApi {
    pub async fn get_users(&self) -> Result<Vec<UserRow>, ApiError> {
        self.get::<Vec<UserRow>>("/api/v1/users")
            .await
    }
}
