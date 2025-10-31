use dotenv::dotenv;
use mongodb:: Client;


pub async fn load_file_connect_db() -> Client {
    dotenv().ok();

   let uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb::/localhost/27017".into());
   let client: Client = Client::with_uri_str(uri).await.expect("Failed to connect");

   return client;
}
