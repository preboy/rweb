mod handler;
mod keys;
mod state;
mod util;

use std::sync::Arc;

use axum::Router;

use bytes::{Bytes, BytesMut, Buf, BufMut};


include!(concat!(env!("OUT_DIR"), "/snazzy.items.rs"));

#[tokio::main]
async fn main() {
    init_logger();

    log::info!("server is staring up ...");

    let app_state = Arc::new(state::new().await);

    let app: Router = Router::new()
        .nest("/book", handler::book::router())
        .nest("/user", handler::user::router())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn init_logger() {
    unsafe {
        std::env::set_var("RUST_LOG", "debug");
        std::env::set_var("RUST_BACKTRACE", "full");
    }

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .init();
}
