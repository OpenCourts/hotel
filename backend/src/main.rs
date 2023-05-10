extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    hotel::rocket().launch().await?;
    Ok(())
}