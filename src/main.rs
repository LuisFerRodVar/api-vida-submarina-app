use actix_web::{web, App, HttpServer };
use mongodb::{Client, options::ClientOptions };
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
            .route("/users", web::put().to(users::modify_user))
            .route("/news", web::post().to(news::create_news))
            .route("/news", web::get().to(news::get_news))
            .route("/species", web::post().to(species::create_specie))
            .route("/species", web::get().to(species::get_species))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
