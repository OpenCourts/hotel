pub mod handler;
pub mod response;

use rocket::{Build, Rocket};

pub fn build_api(rc: Rocket<Build>, base: &str) -> Rocket<Build> {
    rc.mount(
        base,
        routes![
            handler::get_hotels,
        ],
    )
}