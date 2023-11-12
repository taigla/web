use serde::{Deserialize, Serialize};
use super::{TaiglaApi, ApiError};

#[derive(Debug, Deserialize, Clone)]
pub struct RequestProfileRow {
    pub id: u64,
    pub name: String
}

#[derive(Deserialize, Serialize)]
pub struct RequestProfile {
    pub id: u64,
    pub name: String,
    pub language: String,
    pub max_file_size: u64,
    pub min_file_size: u64,
    pub quality: String
}

impl TaiglaApi {
    pub async fn get_request_profiles(&self) -> Result<Vec<RequestProfileRow>, ApiError> {
        self.get("/api/v1/request-profiles")
            .await
    }

    pub async fn get_request_profile(&self, id: u64) -> Result<RequestProfile, ApiError> {
        self.get(&format!("/api/v1/request-profiles/{id}"))
            .await
    }

    pub async fn post_request_profile<T: Serialize>(&self, request_profile: T) -> Result<RequestProfile, ApiError> {
        self.post("/api/v1/request-profiles", &request_profile)
            .await
    }

    pub async fn patch_request_profile<T: Serialize>(&self, id: u64, request_profile: T) -> Result<RequestProfile, ApiError> {
        self.patch(&format!("/api/v1/request-profiles/{id}"),&request_profile)
            .await
    }
}
