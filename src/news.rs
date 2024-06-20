use actix_web::{web, Responder, HttpResponse};
use mongodb::{bson::{doc, document, oid::ObjectId, Document},  Client};
use serde::{Deserialize, Serialize};
use futures_util::stream::StreamExt; 

#[derive(Serialize, Deserialize)]
pub struct News{
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    title: String,
    content: String,
    thumbnail: String,
}

pub async fn create_news(client: web::Data<Client>, new: web::Json<News>) -> impl Responder {
    let collection: mongodb::Collection<News> = client.database("vida_submarina").collection("news");
    let new_new = News {
        id: None,
        title: new.title.clone(),
        content: new.content.clone(),
        thumbnail: new.thumbnail.clone(),
    };
    match collection.insert_one(new_new, None).await {
        Ok(_) => HttpResponse::Created().json("New created"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

pub async fn get_news(client: web::Data<Client>) -> impl Responder {
    let collection: mongodb::Collection<Document> = client.database("vida_submarina").collection("news");

    match collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut news = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(document) => {
                        let new: News = mongodb::bson::from_document(document).unwrap();
                        news.push(new);
                    },
                    Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
                }
            }
            HttpResponse::Ok().json(news)
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}

