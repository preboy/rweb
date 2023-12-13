mod handler;
mod keys;
mod state;
mod util;

use std::sync::Arc;

use axum::Router;

#[tokio::main]
async fn main() {
    init_logger();

    log::error!("server is staring up");
    log::warn!("server is staring up");
    log::debug!("server is staring up");
    log::info!("server is staring up");
    log::trace!("server is staring up");

    let app_state = Arc::new(state::new());

    let app: Router = Router::new()
        .nest("/book", handler::book::router())
        .nest("/user", handler::user::router())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn init_logger() {
    std::env::set_var("RUST_LOG", "debug");

    // env_logger::init();
    tracing_subscriber::fmt::init();
}
