extern crate rocket;

use rocket::fs::FileServer;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    let _rocket = rocket::build()
        .mount("/", FileServer::from("static/"))
        .launch()
        .await?;
    Ok(())
}