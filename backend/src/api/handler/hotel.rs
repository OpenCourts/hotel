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
    id: i32,
    name: String,
    description: String,
    size: i32,
    capacity: i32,
    amenities: String,
    image_url: String,
    price_per_night: i32,
    room_count: i64, //da Postgres 64Bit int zurück gibt
    room_available_count: i64
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

#[get("/search?<from_date>&<to_date>&<price_upper>&<price_lower>&<capacity>&<hotel_id>")]
pub async fn get_room_types(from_date: &str, to_date: &str, price_upper: Option<i32>, price_lower: Option<i32>, capacity: Option<i32>,hotel_id: i32, mut conn: Connection<Db>) -> Result<Json<Vec<RoomType>>, Status> {
    let result = sqlx::query_as::<_, RoomType>("select rt.*, (select
                                                                            count(*) from rooms r
                                                                            where
                                                                            hotel_id = $3 and room_type_id = rt.id) as room_count,
                                                                            (select count(*) from rooms r2 where r2.id not in (select b.room_id from bookings b where (b.check_in_date, b.check_out_date) overlaps ($1::DATE, $2::DATE))
                                                                             and r2.room_type_id in (select rt2.id from room_types rt2 where rt2.price_per_night between $4 and $5 and capacity >= $6)
                                                                            and hotel_id = $3 and room_type_id = rt.id) as room_available_count
                                                                            from room_types rt ")
        .bind(from_date)
        .bind(to_date)
        .bind(hotel_id)
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
        Ok(rooms) => Ok(Json(rooms)),
        Err(error) => {
            error!("Failed to process Request:{:?}", error);
            match error {
                sqlx::Error::RowNotFound => Err(Status::NotFound),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}


