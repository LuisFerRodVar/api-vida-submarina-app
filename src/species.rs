use actix_web::{web, Responder, HttpResponse};
use mongodb::{bson::{doc , oid::ObjectId, Document},  Client};
use serde::{Deserialize, Serialize};
use futures_util::stream::StreamExt; 

#[derive(Serialize, Deserialize)]
pub struct Species {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>,
    name: String,
    description: String,
    thumbnail: String,
    group: String,
}

pub async fn create_specie(client: web::Data<Client>, new: web::Json<Species>) -> impl Responder {
    let collection: mongodb::Collection<Species> = client.database("vida_submarina").collection("species");
    let new_specie = Species {
        id: None,
        name: new.name.clone(),
        description: new.description.clone(),
        thumbnail: new.thumbnail.clone(),
        group: new.group.clone(),
    };
    match collection.insert_one(new_specie, None).await {
        Ok(_) => HttpResponse::Created().json("Specie created"),
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
    
}

pub async fn get_species(client: web::Data<Client>) -> impl Responder {
    let collection: mongodb::Collection<Document> = client.database("vida_submarina").collection("species");
    match collection.find(None, None).await {
        Ok(mut cursor) => {
            let mut species = Vec::new();
            while let Some(result) = cursor.next().await {
                match result {
                    Ok(document) => {
                        let specie: Species = mongodb::bson::from_document(document).unwrap();
                        species.push(specie);
                    },
                    Err(e) => return HttpResponse::InternalServerError().json(e.to_string()),
                }
            }
            HttpResponse::Ok().json(species)
        },
        Err(err) => HttpResponse::InternalServerError().json(err.to_string()),
    }
}
