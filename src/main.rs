/*
 * GET /health
 * GET /items
 * GET /items/{id}
 * POST /items with JSON {"name": "", ..., "value": }
 * DELETE /items/{id}
 */


use actix_web::{HttpResponse, App, HttpServer, Responder, get};
use serde_json::json;

#[get("/")]
async fn root_page() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "This is ROOT PAGE",
        "Status": "Ok",
    }))
}

#[get("/health")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Health Status is good",
        "Status": "Ok",
    }))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Server is running on http://localhost:8080/");

    HttpServer::new(|| {
        App::new()
            .service(root_page)
            .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
