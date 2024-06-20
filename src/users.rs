use actix_web::{web, Responder, HttpResponse};
use mongodb::{Client, bson::{doc, oid::ObjectId }};
use serde::{Deserialize, Serialize};

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
    let collection: mongodb::Collection<User> = client.database("vida_submarina").collection("users");
    if collection.find_one(doc! {"email": &user.email}, None).await.unwrap().is_some(){
        return HttpResponse::Conflict().json("User already exists");
    }
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
pub async fn login(client: web::Data<Client>, user: web::Json<User>) -> impl Responder {
    let collection: mongodb::Collection<User> = client.database("vida_submarina").collection("users");
    let filter = doc! {
        "email": &user.email,
        "password": &user.password
    };
    match collection.find_one(filter, None).await {
        Ok(user) => match user {
            Some(user) => HttpResponse::Ok().json(user),
            None => HttpResponse::NotFound().json("User not found"),
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
pub async fn modify_user(client: web::Data<Client>, user: web::Json<User>) -> impl Responder {
    let collection: mongodb::Collection<User> = client.database("vida_submarina").collection("users");
    let filter = doc! {
        "email": &user.email
    };
    let update = doc! {
        "$set": {
            "username": &user.username,
            "email_2": &user.email_2,
            "password": &user.password
        }
    };
    match collection.update_one(filter, update, None).await {
        Ok(_) => HttpResponse::Ok().json("User modified"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

