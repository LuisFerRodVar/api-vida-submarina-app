use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use mongodb::{Client, options::ClientOptions, bson::{doc, oid::ObjectId, Document}};
use serde::{Deserialize, Serialize};
use futures_util::stream::StreamExt; // Importar futures_util::stream::StreamExt

#[derive(Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    username: String,
    email: String,
    email_2: String,
    password: String,
}

pub async fn create_user(client: web::Data<Client>, user: web::Json<User>) -> impl Responder {
    let collection = client.database("vida_submarina").collection("users");
    let new_user = User {
        id: None,
        username: user.username.clone(),
        email: user.email.clone(),
        email_2: user.email_2.clone(),
        password: user.password.clone(),
    };
    match collection.insert_one(new_user, None).await {
        Ok(_) => HttpResponse::Created().json("User created"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
