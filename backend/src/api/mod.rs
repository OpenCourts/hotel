pub mod handler;

use rocket::{Build, Rocket};

pub fn build_api(rc: Rocket<Build>, base: &str) -> Rocket<Build> {
    rc.mount(
        base,
        routes![
        handler::get_hotels,
        handler::get_hotel,
        handler::get_room_types,
        handler::get_carousel,
        handler::post_booking
        ],
    )
}