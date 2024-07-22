mod features;
mod utils;

use actix_web::{middleware::Logger, web, App, HttpServer};
use features::products::products_controller;
use mongodb::Client;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let uri = std::env::var("MONGODB_URI").expect("MongoDB URI is not provided");
    let client = Client::with_uri_str(uri).await.expect("failed to connect");

    HttpServer::new(move || {
        let logger = Logger::default();

        App::new().app_data(web::Data::new(client.clone())).service(
            web::scope("/api/v1")
                .wrap(logger)
                .configure(products_controller::configure),
        )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
