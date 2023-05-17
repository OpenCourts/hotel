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

#[derive(FromRow, serde::Serialize)]
pub struct RoomType {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<i32>,
    name: String,
    description: String,
    size: i32,
    capacity: i32,
    amenities: String,
    price_per_night: i32
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

#[get("/hotels/<id>/rooms")]
pub async fn get_hotel_rooms(id: i32, mut conn: Connection<Db>) -> Result<Json<Hotel>, Status> {
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

#[get("/search?<from_date>&<to_date>&<price_upper>&<price_lower>&<capacity>")]
pub async fn get_room_types(from_date: &str, to_date: &str, price_upper: Option<i32>, price_lower: Option<i32>, capacity: Option<i32>, mut conn: Connection<Db>) -> Result<Json<Vec<RoomType>>, Status> {
    let result = sqlx::query_as::<_, RoomType>(" Select * from room_types
                                                                where id in (select distinct  room_type_id  from rooms
                                                                where id not in (
                                                                    select room_id from bookings
                                                                    where check_in_date >= to_date($1, 'YYYY-MM-DD')
                                                                    and check_out_date <= to_date($2, 'YYYY-MM-DD'))
                                                                and room_type_id  in (
                                                                    select id from room_types
                                                                    where room_types.price_per_night between $3 and $4
                                                                    and capacity >= $5))")

        .bind(from_date)
        .bind(to_date)
        .bind(match price_lower {
            Some(val) => val,
            None => 1
        })
        .bind(match price_upper {
            Some(val) => val,
            None => 500
        })
        .bind(match capacity {
            Some(val) => val,
            None => 0
        })
        .fetch_all(&mut *conn)
        .await;

    match result {
        Ok(room) => Ok(Json(room)),
        Err(error) => {
            error!("Failed to process Request:{:?}", error);
            match error {
                sqlx::Error::RowNotFound => Err(Status::NotFound),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}

