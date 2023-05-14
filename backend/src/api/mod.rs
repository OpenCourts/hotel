pub mod handler;

use rocket::{Build, Rocket};

pub fn build_api(rc: Rocket<Build>, base: &str) -> Rocket<Build> {
    rc.mount(
        base,
        routes![
            handler::get_hotels,
            handler::get_hotel
        ],
    )
}