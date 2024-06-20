use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::{Client, options::ClientOptions, bson::{doc, oid::ObjectId, Document}};
use futures_util::stream::StreamExt;
mod news;
mod species;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let client = Client::with_options(client_options).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(client.clone()))
            .route("/users", web::post().to(users::create_user))
            .route("/login", web::get().to(users::login))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
