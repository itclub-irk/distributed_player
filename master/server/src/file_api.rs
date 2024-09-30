use crate::config::NodeConfig;
use axum::extract::Query;
use axum::{extract, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use std::path::{self, PathBuf};
use std::sync::Arc;
use walkdir::WalkDir;

#[derive(Deserialize, Debug)]
pub struct GetListFilesParams {
    requested_path: Option<String>,
    folders_only: Option<bool>,
}
#[derive(Serialize)]
pub struct FolderEntry {
    name: String,
    path: PathBuf,
    is_file: bool,
}

/// Returns list of files for requested path. If GET parameter `requested_path` is empty - list of media folder files will be returned.
/// If GET parameter `folders_only` is true - only folders will be returned
pub async fn get_list_files(
    params: extract::Query<GetListFilesParams>,
    extract::State(config): extract::State<Arc<NodeConfig>>,
) -> (StatusCode, Json<serde_json::Value>) {
    let Query(params) = params;
    let requested_path = params.requested_path.unwrap_or_default();
    let folders_only = params.folders_only.unwrap_or_default();

    let media_folder_path = path::Path::new(config.media.folder.as_str());
    let full_path = path::Path::join(
        path::Path::new(config.media.folder.as_str()),
        requested_path.replace("..", "."),
    );
    let mut result: Vec<FolderEntry> = vec![];

    for e in WalkDir::new(&full_path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let is_file = e.metadata().unwrap().is_file();
        if folders_only && is_file {
            continue;
        }

        let file_name = e.file_name().to_str().unwrap().to_string();
        let file_path = e
            .path()
            .strip_prefix(&media_folder_path)
            .unwrap()
            .to_owned();

        result.push(FolderEntry {
            name: file_name,
            path: file_path,
            is_file,
        });
    }

    (StatusCode::OK, Json(serde_json::json!({"files": result})))
}
