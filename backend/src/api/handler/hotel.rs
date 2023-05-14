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
    address: String,
    phone_number: String,
}


#[get("/hotels")]
pub async fn get_hotels(mut conn: Connection<Db>) -> Result<Json<Vec<Hotel>>, Status> {
    let result = sqlx::query_as::<_, Hotel>("SELECT id, name, address, phone_number FROM hotels")
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
    let result = sqlx::query_as::<_, Hotel>("SELECT id, name, address, phone_number FROM hotels WHERE id = $1")
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