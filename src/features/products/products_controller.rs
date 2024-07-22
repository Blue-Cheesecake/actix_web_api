use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::{bson::doc, Client, Collection};

use crate::{
    features::products::product_model::ProductModel, utils::common_dto::CommonSimpleMessageDto,
};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .service(get_product_list)
            .service(create_product),
    );
}

#[get("")]
async fn get_product_list(client: web::Data<Client>) -> impl Responder {
    let collection: Collection<ProductModel> = client.database("myself").collection("products");
    let mut cursor = collection
        .find(doc! {})
        .await
        .expect("error on finding document");

    let mut products: Vec<ProductModel> = Vec::new();

    loop {
        if cursor.advance().await.unwrap() {
            products.push(cursor.deserialize_current().unwrap());
        }

        break;
    }

    HttpResponse::Ok().json(products)
}

#[post("")]
async fn create_product(
    client: web::Data<Client>,
    data: web::Json<ProductModel>,
) -> impl Responder {
    let collection: Collection<ProductModel> = client.database("myself").collection("products");
    let result = collection.insert_one(data.into_inner()).await;

    match result {
        Ok(_) => HttpResponse::Created().json(CommonSimpleMessageDto::from("Product created")),
        Err(_) => HttpResponse::InternalServerError()
            .json(CommonSimpleMessageDto::from("An error is occurred")),
    }
}
