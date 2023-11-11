use serde::Deserialize;
use super::{TaiglaApi, ApiError};

#[derive(Deserialize)]
pub enum WorkerState {
    Idle,
    #[serde(untagged)]
    Running {
        job: String
    }
}

#[derive(Deserialize)]
pub struct Job {
    pub uuid: String,
    pub interval: u32,
    pub name: String
}

impl TaiglaApi {
    pub async fn get_workers(&self) -> Result<Vec<WorkerState>, ApiError> {
        self.get::<Vec<WorkerState>>("/api/v1/job/workers")
            .await
    }

    pub async fn get_cronjobs(&self) -> Result<Vec<Job>, ApiError> {
        self.get::<Vec<Job>>("/api/v1/job/cronjobs")
            .await
    }
}
