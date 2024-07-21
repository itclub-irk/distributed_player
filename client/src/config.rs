use gethostname::gethostname;
use serde_derive::Deserialize;
use std::fs;
use std::path::Path;
use toml;

#[derive(Deserialize, Debug)]
pub struct NodeConfig {
    pub media: Media,
    pub node: Node,
}

#[derive(Deserialize, Debug)]
pub struct Media {
    pub folder: String,
}

#[derive(Deserialize, Debug)]
pub struct Node {
    pub name: Option<String>,
}

impl NodeConfig {
    pub fn read_from_file(file_name: &str) -> NodeConfig {
        let config_file_content = fs::read_to_string(file_name)
            .expect(&format!("Unable to read config file {}!", file_name));
        let mut conf: NodeConfig = toml::from_str(&config_file_content)
            .expect(&format!("Unable to parse config file {}!", file_name));

        if conf.node.name.is_none() {
            conf.node.name = match gethostname().into_string() {
                Ok(s) => Some(s),
                Err(_) => None,
            };
        };

        if !Path::new(&conf.media.folder).exists() {
            panic!("Media folder \"{}\" does not exists!", &conf.media.folder);
        };

        conf
    }
}
