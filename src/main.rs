mod db;
mod endpoints;
mod models;
#[macro_use]
extern crate rocket;

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(db::mongo::connect())
        .mount("/", routes![endpoints::task::post])
}
