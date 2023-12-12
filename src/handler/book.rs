use actix_web::{web, HttpResponse};

// basic handler that responds with a static string
pub async fn root() -> &'static str {
    "Hello, World!"
}

// this function could be located in different module
pub fn config(cfg: &mut web::ServiceConfig) {}
