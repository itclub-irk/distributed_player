use crate::config::NodeConfig;
use chrono;
use itertools::Itertools;
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::{error::Error, fs, path::Path};

#[derive(Deserialize, Debug)]
pub struct Playlist {
    pub working_hours: Option<Vec<WorkingHours>>,
    pub working_hours_exceptions: Option<HashMap<chrono::NaiveDate, WorkingHours>>,
    pub shuffle: Option<bool>,
    pub folders: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct WorkingHours(
    #[serde(with = "toml_datetime_compat")] chrono::NaiveTime,
    #[serde(with = "toml_datetime_compat")] chrono::NaiveTime,
);

fn read_playlist_from_file(file_path: &Path) -> Result<Playlist, Box<(dyn Error + 'static)>> {
    let config_file_content = fs::read_to_string(file_path);
    if let Ok(c) = config_file_content {
        let parsed = toml::from_str::<Playlist>(&c);
        if let Ok(p) = parsed {
            return Ok(p);
        };
        return Err(format!("Cannot parse playlist file {}", file_path.display()).into());
    }
    Err(format!("Cannot open playlist file {}", file_path.display()).into())
}

impl Playlist {
    pub fn read_from_config(node_config: &NodeConfig) -> Option<Playlist> {
        let media_folder = node_config.media.folder.as_str();
        let node_name = &node_config.node.name;

        let cfg_folder = Path::new(media_folder).join("cfg");
        let default_playlist_file_path = cfg_folder.clone().join("playlist.toml");
        let default_playlist = read_playlist_from_file(&default_playlist_file_path);

        if default_playlist.is_err() {
            return None;
        }

        let default_playlist = default_playlist.unwrap();

        match node_name {
            Some(n) => {
                let node_playlist_file_path =
                    cfg_folder.clone().join(format!("playlist_{}.toml", n));
                let node_playlist = read_playlist_from_file(&node_playlist_file_path);

                match node_playlist {
                    Ok(p) => Some(p.merge(&default_playlist)),
                    Err(_) => Some(default_playlist),
                }
            }
            None => return Some(default_playlist),
        }
    }

    fn merge(&self, other: &Playlist) -> Playlist {
        let mut working_hours_exceptions: Option<HashMap<chrono::NaiveDate, WorkingHours>> = None;
        if self.working_hours_exceptions.is_none() && other.working_hours_exceptions.is_some() {
            working_hours_exceptions = other.working_hours_exceptions.clone();
        } else if self.working_hours_exceptions.is_some()
            && other.working_hours_exceptions.is_none()
        {
            working_hours_exceptions = self.working_hours_exceptions.clone();
        } else if self.working_hours_exceptions.is_some()
            && other.working_hours_exceptions.is_some()
        {
            let mut ex1: HashMap<chrono::NaiveDate, WorkingHours> =
                other.working_hours_exceptions.clone().unwrap();
            let ex2 = self.working_hours_exceptions.clone().unwrap();
            ex1.extend(ex2);
            working_hours_exceptions = Some(ex1);
        }

        let mut folders: Vec<String> = self.folders.clone().unwrap_or(vec![]);
        folders.extend(other.folders.clone().unwrap_or(vec![]));

        if self.folders.is_none() && other.folders.is_some() {}

        let result = Playlist {
            working_hours: self.working_hours.clone().or(other.working_hours.clone()),
            working_hours_exceptions,
            shuffle: self.shuffle.or(other.shuffle),
            folders: Some(folders.into_iter().unique().collect()),
        };
        return result;
    }
}
