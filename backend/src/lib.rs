#[macro_use]
extern crate rocket;

use rocket::{Build, /*Request,*/ Rocket};
use rocket::fs::FileServer;

pub mod api;
mod sqlx;
mod mailer;


/*#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}*/

#[launch]
pub fn rocket() -> Rocket<Build> {
    let mut rc = rocket::build()
        .attach(sqlx::stage())
        .mount("/", FileServer::from("static/"));
        //.register("/", catchers![not_found]);
    // mount the api (/api)
    rc = api::build_api(rc, "/api");
    rc
}