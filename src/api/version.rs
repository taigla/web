use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Version {
    pub version: String
}
