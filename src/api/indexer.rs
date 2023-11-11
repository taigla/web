use serde::{Deserialize, Serialize};
use super::{TaiglaApi, ApiError};

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct IndexerRow {
    pub id: u64,
    pub name: String,
    pub priority: u8
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Indexer {
    pub id: u64,
    pub name: String,
    pub url: String,
    pub api_key: Option<String>,
    pub priority: u8
}

#[derive(Debug, Serialize)]
pub struct IndexerCreate {
    pub name: String,
    pub url: String,
    pub api_key: String,
    pub priority: u8
}

impl TaiglaApi {
    pub async fn get_indexers(&self) -> Result<Vec<IndexerRow>, ApiError> {
        self.get::<Vec<IndexerRow>>("/api/v1/indexers")
            .await
    }

    pub async fn get_indexer(&self, id: u64) -> Result<Indexer, ApiError> {
        self.get::<Indexer>(&format!("/api/v1/indexers/{id}"))
            .await
    }

    pub async fn patch_indexer<T: Serialize>(&self, id: u64, indexer: T) -> Result<Indexer, ApiError> {
        self.patch(&format!("/api/v1/indexers/{id}"), &indexer)
            .await
    }

    pub async fn post_indexer<T: Serialize>(&self, indexer: T) -> Result<Indexer, ApiError> {
        self.post("/api/v1/indexers", &indexer)
            .await
    }
}
