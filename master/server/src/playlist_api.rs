use crate::{config::NodeConfig, playlist::Playlist, toml_file::ReadAndWriteTOMLFile};
use axum::{extract, http::StatusCode, Json};
use regex::Regex;
use serde_json;

use std::path;
use std::{
    fs::{self},
    sync::Arc,
};

static PLAYLISTS_DIR_NAME: &'static str = "cfg";
static DEFAULT_PLAYLIST_NAME: &'static str = "playlist";
static PLAYLIST_FILE_EXTENTION: &'static str = "toml";

fn get_configs_folder_path(config: &NodeConfig) -> path::PathBuf {
    path::Path::join(
        path::Path::new(config.media.folder.as_str()),
        PLAYLISTS_DIR_NAME,
    )
}

/// Returns names of playlists files in media folder
pub async fn get_playlists(
    extract::State(config): extract::State<Arc<NodeConfig>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let configs_folder = get_configs_folder_path(&config);

    let mut result: Vec<String> = vec![];
    let re = Regex::new(
        format!(r"{DEFAULT_PLAYLIST_NAME}_(?<name>.+).{PLAYLIST_FILE_EXTENTION}").as_str(),
    )
    .unwrap();

    for dir_entry_result in fs::read_dir(configs_folder).unwrap() {
        let Ok(playlist_entry) = dir_entry_result else {
            continue;
        };

        if playlist_entry.metadata().unwrap().is_dir() {
            continue;
        }
        let playlist_file_name_os_string = playlist_entry.file_name();
        let Some(playlist_file_name) = playlist_file_name_os_string.to_str() else {
            continue;
        };

        let Some(groups) = re.captures(playlist_file_name) else {
            continue;
        };
        result.push(groups["name"].to_string())
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({"playlists": result})),
    )
}

fn get_playlist_file_path(config: &NodeConfig, playlist_name: String) -> path::PathBuf {
    let configs_folder = get_configs_folder_path(&config);

    let playlist_file_path: path::PathBuf;
    if playlist_name == "default" {
        playlist_file_path =
            configs_folder.join(format!("{DEFAULT_PLAYLIST_NAME}.{PLAYLIST_FILE_EXTENTION}"));
    } else {
        let sanitized_playlist_name = playlist_name.replace(".", "");
        playlist_file_path = configs_folder.join(format!(
            "{DEFAULT_PLAYLIST_NAME}_{sanitized_playlist_name}.{PLAYLIST_FILE_EXTENTION}"
        ));
    }
    return playlist_file_path;
}

/// Return playlist data for given playlist name
pub async fn get_playlist(
    extract::Path(playlist_name): extract::Path<String>,
    extract::State(config): extract::State<Arc<NodeConfig>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let playlist_file_path = get_playlist_file_path(&config, playlist_name);

    match Playlist::read_from_file(&playlist_file_path) {
        Ok(playlist) => (StatusCode::OK, Json(serde_json::json!(playlist))),
        Err(_) => (StatusCode::BAD_REQUEST, Json(serde_json::json!({}))),
    }
}

/// Updates playlist with given name using given playlist content
pub async fn update_playlist(
    extract::Path(playlist_name): extract::Path<String>,
    extract::State(config): extract::State<Arc<NodeConfig>>,
    extract::Json(playlist): extract::Json<Playlist>,
) -> StatusCode {
    let playlist_file_path = get_playlist_file_path(&config, playlist_name);

    match playlist.write_to_file(&playlist_file_path) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

/// Deletes playlist with given name
pub async fn delete_playlist(
    extract::Path(playlist_name): extract::Path<String>,
    extract::State(config): extract::State<Arc<NodeConfig>>,
) -> StatusCode {
    if playlist_name == "default" {
        return StatusCode::BAD_REQUEST;
    };

    let playlist_file_path = get_playlist_file_path(&config, playlist_name);
    match fs::remove_file(playlist_file_path) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::BAD_REQUEST,
    }
}
