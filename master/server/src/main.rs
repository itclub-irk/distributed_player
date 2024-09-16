mod auth_api;
mod config;
mod password;
mod playlist;
mod playlist_api;
mod toml_file;

use crate::toml_file::ReadAndWriteTOMLFile;
use axum::{
    middleware,
    routing::{delete, get, post},
    Router,
};
use config::NodeConfig;
use http::{
    header::{self, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method, Request, Response,
};
use std::{path::Path, sync::Arc};

use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

const CONFIG_FILE_NAME: &'static str = "node_config.toml";
const HTTP_SERVER_BIND_ADDRESS: &'static str = "0.0.0.0:3000";

fn main() {
    // configure_logger();
    let config_file_path = Path::new(CONFIG_FILE_NAME);
    let mut node_config =
        NodeConfig::read_from_file(&config_file_path).expect("Unable to parse node config!");

    if std::env::args().nth(1) == Some("set_password".to_string()) {
        loop {
            print!("Enter new password:");
            let raw_password_string =
                password::read_from_stdin().expect("Enter valid unicode string!");
            print!("Repeat new password:");
            if raw_password_string
                == password::read_from_stdin().expect("Enter valid unicode string!")
            {
                let hash = password::hash_password_string(raw_password_string.clone());
                node_config.auth.password_hash = hash;
                node_config
                    .write_to_file(&config_file_path)
                    .expect("Unable to write node config!");
                println!("Password changed successfully!");
                // log::info!("Password changed successfully!");
                break;
            } else {
                println!("Entered passwords do not match! Try again.")
            }
        }
        return;
    }
    run_server(Arc::new(node_config));
}

#[tokio::main]
async fn run_server(node_config: Arc<NodeConfig>) {
    tracing_subscriber::fmt::init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let app = Router::new()
        .nest_service("/assets", ServeDir::new("assets"))
        .route("/api/auth/login", post(auth_api::login_user_handler))
        .route(
            "/api/auth/is_logged_in",
            get(auth_api::is_logged_in).route_layer(middleware::from_fn_with_state(
                node_config.clone(),
                auth_api::auth_required,
            )),
        )
        .route(
            "/api/auth/logout",
            get(auth_api::logout_user_handler).route_layer(middleware::from_fn_with_state(
                node_config.clone(),
                auth_api::auth_required,
            )),
        )
        .route(
            "/api/playlist",
            get(playlist_api::get_playlists).route_layer(middleware::from_fn_with_state(
                node_config.clone(),
                auth_api::auth_required,
            )),
        )
        .route(
            "/api/playlist/:playlist_name",
            get(playlist_api::get_playlist).route_layer(middleware::from_fn_with_state(
                node_config.clone(),
                auth_api::auth_required,
            )),
        )
        .route(
            "/api/playlist/:playlist_name",
            post(playlist_api::update_playlist).route_layer(middleware::from_fn_with_state(
                node_config.clone(),
                auth_api::auth_required,
            )),
        )
        .route(
            "/api/playlist/:playlist_name",
            delete(playlist_api::delete_playlist).route_layer(middleware::from_fn_with_state(
                node_config.clone(),
                auth_api::auth_required,
            )),
        )
        .layer(cors)
        .with_state(node_config);

    let listener = tokio::net::TcpListener::bind(HTTP_SERVER_BIND_ADDRESS)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
