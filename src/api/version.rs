use serde::Deserialize;
use super::{TaiglaApi, ApiError};

#[derive(Deserialize)]
pub struct Version {
    version: String
}

impl TaiglaApi {
    pub async fn get_version(&self) -> Result<Version, ApiError> {
        self.get::<Version>("/api/v1/version")
            .await
    }
}
