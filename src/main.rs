mod model;
#[cfg(test)]
mod test;

use dotenv::dotenv;
use model::User;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mongodb::{Client, Collection, IndexModel, bson::doc, options::IndexOptions};

const DB_NAME: &str = "myApp";
const COLL_NAME: &str = "users";


#[post("/add_user")]
async fn add_user(client: web::Data<Client>, form: web::Json<User>) -> HttpResponse {
    let collection = client.database(DB_NAME).collection(COLL_NAME);
    let result = collection.insert_one(form.into_inner()).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("user added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}


#[get("/get_user/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
    let username = username.into_inner();
    let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
    match collection.find_one(doc! { "username": &username }).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => {
            HttpResponse::NotFound().body(format!("No user found with username {username}"))
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

async fn create_username_index(client: &Client) {
    let options = IndexOptions::builder().unique(true).build();
    let model = IndexModel::builder()
        .keys(doc! { "username": 1 })
        .options(options)
        .build();
    client
        .database(DB_NAME)
        .collection::<User>(COLL_NAME)
        .create_index(model)
        .await
        .expect("Creating an index should success");
}


#[get("/")]
async fn test_func() -> impl Responder {
    HttpResponse::Ok().body("testing server")
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Checking manual function")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb::/localhost/27017".into());

    let client = Client::with_uri_str(uri)
        .await
        .expect("Failed to connect");

    create_username_index(&client).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .service(add_user)
            .service(get_user)
            .service(test_func)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8000))?.run().await
}
