use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs;
use toml;

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

impl NodeConfig {
    pub fn read_from_file(file_name: &str) -> NodeConfig {
        let config_file_content = fs::read_to_string(file_name)
            .expect(&format!("Unable to read node config file {}!", file_name));

        toml::from_str(&config_file_content)
            .expect(&format!("Unable to parse node config file {}!", file_name))
    }

    pub fn write_to_file(&self, file_name: &str) {
        let config_file_content = toml::to_string(&self).expect("Unable to serialize node config!");
        fs::write(file_name, config_file_content)
            .expect(&format!("Unable to write node config file {}!", file_name));
    }
}
