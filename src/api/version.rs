use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Version {
    pub version: String
}
