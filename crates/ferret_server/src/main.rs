use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use ferret_image::ImageInfo;
use include_dir::{include_dir, Dir};
use rand::{seq::IteratorRandom, thread_rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: load this in a more efficient matter straight from the fs; for now, it goes straight into memory
// since each image is only max 300kb
static IMAGE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../ferret_images/collection");

#[derive(Serialize, Deserialize)]
enum HandleError {
    InvalidUUID,
    NoImagesPresent,
}

#[derive(Serialize, Deserialize)]
struct RequestError {
    error: HandleError,
}

#[get("/v1/data/random")]
async fn data_random() -> impl Responder {
    let Some(chosen_directory) = IMAGE_DIR.dirs().choose(&mut thread_rng()) else {
        return HttpResponse::InternalServerError().json(RequestError {
            error: HandleError::NoImagesPresent,
        });
    };

    HttpResponse::Ok().json(
        serde_json::from_str::<ImageInfo>(chosen_directory
            .get_file(format!(
                "{}/image.json",
                chosen_directory
                    .path()
                    .to_str()
                    .expect("Could not convert chosen directory path to string")
            ))
            .expect("Could not get image.json")
            .contents_utf8().expect("Could not convert image.json to unicode"))
            .unwrap(),
    )
}

#[get("/v1/image/random")]
async fn image_random() -> impl Responder {
    let Some(chosen_directory) = IMAGE_DIR.dirs().choose(&mut thread_rng()) else {
        return HttpResponse::InternalServerError().json(RequestError {
            error: HandleError::NoImagesPresent,
        });
    };

    HttpResponse::Ok().body(
        chosen_directory
            .get_file(format!(
                "{}/image.jpg",
                chosen_directory
                    .path()
                    .to_str()
                    .expect("Could not convert chosen directory path to string")
            ))
            .expect("Could not get image.jpg")
            .contents(),
    )
}

#[get("/v1/image/uuid/{uuid}")]
async fn image_get_by_uuid(uuid: web::Path<String>) -> impl Responder {
    let uuid = Uuid::try_parse(&uuid);
    match uuid {
        Ok(uuid) => HttpResponse::Ok().body(
            IMAGE_DIR
                .get_file(format!("{}/image.jpg", uuid))
                .unwrap()
                .contents(),
        ),
        Err(_) => HttpResponse::BadRequest().json(RequestError {
            error: HandleError::InvalidUUID,
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(data_random)
            .service(image_random)
            .service(image_get_by_uuid)
    })
    .bind(("::", 8080))?
    .run()
    .await
}
