#[macro_use]
extern crate rocket;

use rocket::{Build, Request, Response, Rocket};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::fs::FileServer;
use rocket::http::Header;

pub mod api;
mod sqlx;
mod mailer;


/*#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}*/

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[launch]
pub fn rocket() -> Rocket<Build> {
    let mut rc = rocket::build()
        .attach(sqlx::stage())
        .attach(CORS)
        .mount("/", FileServer::from("static/"));
        //.register("/", catchers![not_found]);
    // mount the api (/api)
    rc = api::build_api(rc, "/api");
    rc
}