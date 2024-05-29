use rocket::{Build, Rocket};

mod db;
mod endpoints;
mod models;
#[macro_use]
extern crate rocket;

#[launch]
pub async fn rocket() -> Rocket<Build> {
    rocket::build()
        .attach(db::mongo::connect())
        .mount("/", routes![endpoints::task::post])
}
