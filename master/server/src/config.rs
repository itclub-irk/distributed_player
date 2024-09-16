use crate::toml_file::ReadAndWriteTOMLFile;
use serde::{Deserialize, Serialize};
// use serde_derive::Deserialize;

#[derive(Deserialize, Serialize, Debug)]
pub struct NodeConfig {
    pub media: Media,
    pub auth: Auth,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Media {
    pub folder: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Auth {
    pub password_hash: String,
    pub jwt_secret: String,
    pub jwt_maxage: i64,
    pub jwt_expires_in: String,
}

impl ReadAndWriteTOMLFile<NodeConfig> for NodeConfig {}
