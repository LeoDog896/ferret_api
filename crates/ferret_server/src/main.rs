use actix_web::{get, http::header::{CacheControl, CacheDirective}, web::{self, Redirect}, App, Either, HttpResponse, HttpServer, Responder};
use ferret_image::ImageInfo;
use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;
use rand::{seq::IteratorRandom, thread_rng};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// TODO: load this in a more efficient matter straight from the fs; for now, it goes straight into memory
// since each image is only max 300kb
static IMAGE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/../../ferret_images/collection");
static UUID_LIST: Lazy<Vec<String>> = Lazy::new(|| IMAGE_DIR.dirs().map(|dir| dir.path().to_string_lossy().to_string()).collect::<Vec<_>>());

#[derive(Serialize, Deserialize)]
enum HandleError {
    InvalidUUID,
    NoImagesPresent,
    FileNotFound(Uuid, ReturnType),
    InfoNotUTF8(Uuid),
    JsonNotValidSchema(Uuid),
}

#[derive(Serialize, Deserialize)]
struct RequestError {
    error: HandleError,
}

#[derive(Serialize, Deserialize, Debug)]
enum ReturnType {
    Image,
    Data,
}

impl ReturnType {
    fn file_name(&self) -> &str {
        match self {
            Self::Image => "image.jpg",
            Self::Data => "image.json",
        }
    }
}

fn get_dir(uuid: &str, return_type: ReturnType) -> HttpResponse {
    let uuid = Uuid::try_parse(uuid);
    match uuid {
        Ok(uuid) => {
            let Some(file) = IMAGE_DIR.get_file(format!("{}/{}", uuid, return_type.file_name()))
            else {
                return HttpResponse::BadRequest().json(RequestError {
                    error: HandleError::FileNotFound(uuid, return_type),
                });
            };

            match return_type {
                ReturnType::Image => {
                    let mut response = HttpResponse::Ok();
                    
                    response.insert_header(CacheControl(vec![CacheDirective::MaxAge(u32::MAX)]));

                    response.body(file.contents())
                },
                ReturnType::Data => {
                    let Some(contents_utf8) = file.contents_utf8() else {
                        return HttpResponse::InternalServerError().json(RequestError {
                            error: HandleError::InfoNotUTF8(uuid),
                        });
                    };

                    let Ok(parsed_serde) = serde_json::from_str::<ImageInfo>(contents_utf8) else {
                        return HttpResponse::InternalServerError().json(RequestError {
                            error: HandleError::JsonNotValidSchema(uuid),
                        });
                    };

                    let mut response = HttpResponse::Ok();
                    
                    response.insert_header(CacheControl(vec![CacheDirective::MaxAge(60 * 60 * 24)]));

                    response.json(parsed_serde)
                }
            }
        }
        Err(_) => HttpResponse::BadRequest().json(RequestError {
            error: HandleError::InvalidUUID,
        }),
    }
}

#[get("/v1/data/uuid/{uuid}")]
async fn data_get_by_uuid(uuid: web::Path<String>) -> impl Responder {
    get_dir(&uuid, ReturnType::Data)
}

#[get("/v1/data/random")]
async fn data_random() -> Either<HttpResponse, Redirect> {
    let Some(chosen_directory) = IMAGE_DIR.dirs().choose(&mut thread_rng()) else {
        return Either::Left(HttpResponse::InternalServerError().json(RequestError {
            error: HandleError::NoImagesPresent,
        }));
    };

    Either::Right(Redirect::to(format!("/v1/data/uuid/{}", chosen_directory.path().to_string_lossy())).temporary())
}

#[get("/v1/image/random")]
async fn image_random() -> Either<HttpResponse, Redirect> {
    let Some(chosen_directory) = IMAGE_DIR.dirs().choose(&mut thread_rng()) else {
        return Either::Left(HttpResponse::InternalServerError().json(RequestError {
            error: HandleError::NoImagesPresent,
        }));
    };

    Either::Right(
        Redirect::to(format!("/v1/image/uuid/{}", chosen_directory.path().to_string_lossy())).temporary()
    )
}

#[get("/v1/image/uuid/{uuid}")]
async fn image_get_by_uuid(uuid: web::Path<String>) -> impl Responder {
    get_dir(&uuid, ReturnType::Image)
}

#[get("/v1/list")]
async fn list() -> impl Responder {
    HttpResponse::Ok().json(UUID_LIST.clone())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(data_random)
            .service(image_random)
            .service(data_get_by_uuid)
            .service(image_get_by_uuid)
    })
    .bind(("::", 8080))?
    .run()
    .await
}
