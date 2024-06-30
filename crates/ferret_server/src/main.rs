use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use include_dir::{include_dir, Dir};
use rand::{seq::IteratorRandom, thread_rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: load this in a more efficient matter straight from the fs; for now, it goes straight into memory
// since each image is only max 300kb
static IMAGE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../ferret_images/collection");

#[get("/v1/image/random")]
async fn random() -> impl Responder {
    let chosen_directory = IMAGE_DIR.dirs().choose(&mut thread_rng()).expect("Could not randomly choose directory");
    HttpResponse::Ok().body(chosen_directory.get_file(format!("{}/image.jpg", chosen_directory.path().to_str().unwrap())).expect("Could not get image.jpg").contents())
}

#[derive(Serialize, Deserialize)]
enum SpecificFetchError {
    InvalidUUID
}

#[get("/v1/image/uuid/{uuid}")]
async fn get_by_uuid(uuid: web::Path<String>) -> impl Responder {
    let uuid = Uuid::try_parse(&uuid);
    match uuid {
        Ok(uuid) => HttpResponse::Ok().body(IMAGE_DIR.get_file(format!("{}/image.jpg", uuid)).unwrap().contents()),
        Err(_) => HttpResponse::BadRequest().json(SpecificFetchError::InvalidUUID)
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(random).service(get_by_uuid))
        .bind(("::", 8080))?
        .run()
        .await
}
