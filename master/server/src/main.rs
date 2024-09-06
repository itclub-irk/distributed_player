mod auth_api;
mod config;
mod password;

use std::sync::Arc;

use auth_api::{auth_required, login_user_handler, logout_user_handler};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use config::NodeConfig;

const CONFIG_FILE_NAME: &'static str = "node_config.toml";
const HTTP_SERVER_BIND_ADDRESS: &'static str = "0.0.0.0:3000";

fn main() {
    // configure_logger();

    let mut node_config = NodeConfig::read_from_file(CONFIG_FILE_NAME);

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
                node_config.write_to_file(CONFIG_FILE_NAME);
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

    let app = Router::new()
        .route(
            "/",
            get(root).route_layer(middleware::from_fn_with_state(
                node_config.clone(),
                auth_required,
            )),
        )
        .route("/api/auth/login", post(login_user_handler))
        .route("/api/auth/logout", post(logout_user_handler))
        .with_state(node_config);

    let listener = tokio::net::TcpListener::bind(HTTP_SERVER_BIND_ADDRESS)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
