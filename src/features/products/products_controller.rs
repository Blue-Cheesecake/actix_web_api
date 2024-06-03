use actix_web::{get, post, web, HttpResponse, Responder};

use crate::utils::common_dto::CommonSimpleMessageDto;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .service(get_product_list)
            .service(create_product),
    );
}

#[get("")]
async fn get_product_list() -> impl Responder {
    HttpResponse::Ok().json(CommonSimpleMessageDto::from("Prodocut list"))
}

#[post("")]
async fn create_product() -> impl Responder {
    HttpResponse::Created().json(CommonSimpleMessageDto::from("Product created"))
}
