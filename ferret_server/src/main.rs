#[macro_use]
extern crate rocket;

#[get("/ferrets/<id>")]
fn ferret_by_id(id: &str) -> String {
    format!("Grabbing ferret by {}", id)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![ferret_by_id])
}
