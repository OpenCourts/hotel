#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket::fs::FileServer;

pub mod api;
pub mod model;
mod sqlx;

#[launch]
pub fn rocket() -> Rocket<Build> {
    let app_data = model::AppState::init();
    let mut rc = rocket::build().manage(app_data)
        .attach(sqlx::stage())
        .mount("/", FileServer::from("static/"));
    // mount the api (/api)
    rc = api::build_api(rc, "/api");
    rc
}