use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::FromRow;
use crate::sqlx::Db;


#[derive(FromRow, serde::Serialize)]
pub struct Hotel {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    name: String,
    street: String,
    house_number: String,
    city: String,
    state: String,
    postal_code: String,
    country: String,
    phone_number: String,
    total_rooms: i32,
}

#[get("/hotels")]
pub async fn get_hotels(mut conn: Connection<Db>) -> Result<Json<Vec<Hotel>>, Status> {
    let result = sqlx::query_as::<_, Hotel>("SELECT id, name, street, house_number, city, state, postal_code, country, phone_number, total_rooms FROM hotels")
        .fetch_all(&mut *conn)
        .await;

    match result {
        Ok(hotels) => Ok(Json(hotels)),
        Err(error) => {
            error!("Failed to fetch hotels: {:?}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[get("/hotels/<id>")]
pub async fn get_hotel(id: i32, mut conn: Connection<Db>) -> Result<Json<Hotel>, Status> {
    let result = sqlx::query_as::<_, Hotel>("SELECT id, name, street, house_number, city, state, postal_code, country, phone_number, total_rooms FROM hotels WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await;

    match result {
        Ok(hotel) => Ok(Json(hotel)),
        Err(error) => {
            error!("Failed to fetch hotel {}: {:?}", id, error);
            match error {
                sqlx::Error::RowNotFound => Err(Status::NotFound),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}