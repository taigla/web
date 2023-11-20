use serde::{Deserialize, Serialize};
use super::{TaiglaApi, ApiError};

#[derive(Debug, Deserialize)]
pub struct Invite {
    pub name: String,
    pub key: String,
    pub remaining_use: u32
}

#[derive(Debug, Serialize)]
pub struct InviteCreate {
    pub name: String,
    pub remaining_use: u32
}

#[derive(Debug, Deserialize)]
pub struct InviteCreateResp {
    pub invite: String
}


impl TaiglaApi {
    pub async fn get_invites(&self) -> Result<Vec<Invite>, ApiError> {
        self.get::<Vec<Invite>>("/api/v1/invites")
            .await
    }

    pub async fn post_invite(&self, invite: InviteCreate) -> Result<InviteCreateResp, ApiError> {
        self.post("/api/v1/invites", &invite)
            .await
    }
}
