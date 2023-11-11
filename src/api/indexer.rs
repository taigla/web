use serde::Deserialize;
use super::{TaiglaApi, ApiError};

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct IndexerRow {
    pub id: u64,
    pub name: String,
    pub priority: u8
}

#[derive(Deserialize)]
pub struct Indexer {
    pub id: u64,
    pub name: String,
    pub url: String,
    pub api_key: Option<String>,
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

    // pub async fn patch_indexer(&self, indexer: Inde)
}
