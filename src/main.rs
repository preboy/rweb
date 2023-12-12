mod handler;
mod keys;
mod state;
mod util;

use actix_files as fs;
use actix_web::{middleware, web, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    log::info!("starting up");

    let data = web::Data::new(state::new());

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(data.clone())
            .service(web::scope("/user").configure(handler::user::config))
            .service(web::scope("/book").configure(handler::book::config))
            .service(fs::Files::new("/static", ".").show_files_listing())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
