mod features;
mod utils;

use actix_web::{middleware::Logger, web, App, HttpServer};
use features::products::products_controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new().service(
            web::scope("/api/v1")
                .wrap(logger)
                .configure(products_controller::configure),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
