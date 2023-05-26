use std::{fs, result};
use std::ops::Deref;
use std::path::{PathBuf};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::{Error, FromRow, PgConnection, PgPool, Pool, Postgres};
use sqlx::postgres::PgQueryResult;
use chrono::{Datelike, NaiveDate};
use rocket::futures::TryFutureExt;
use rocket::serde::json::serde_json::error::Category::Data;
use rocket::State;
use sqlx::Error::Database;
use sqlx::pool::PoolConnection;
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
    image_url: Option<String>,
    price_per_night: i32,
    room_count: i64, //da Postgres 64Bit int zurück gibt
    room_available_count: i64
}
#[derive(FromRow, serde::Serialize)]
pub struct QueryIntegerHelper{
    value: i64
}

#[derive(Debug, serde::Serialize)]
pub struct ErrorResponse {
    error: String,
}

#[derive(serde::Deserialize)]
pub struct Guest{
    id: Option<i32>,
    name: String,
    email: String,
    phone_number: String,
    address: String,
    passport_number: String
}

#[derive(serde::Deserialize)]
pub struct Booking{
    guests: Vec<Guest>,
    room_type_id: i64,
    check_in_date: String,
    check_out_date: String,
    hotel_id: i64
}

pub async fn get_max_id_off_table(table_name: &str, conn: &Connection<Db>) -> Result<i64, Status>{
    let result = sqlx::query_as::<_, QueryIntegerHelper>("Select max(id) from $1")
        .bind(table_name)
        .fetch_one(&conn)
        .await;
    match result {
        Ok(val) => Ok(val.value),
        Err(error) => {
            error!("Sumthing went wong: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

pub async fn get_next_free_room(hotel_id: i64, room_type_id: i64, from_date: &str, to_date: &str, conn: &Connection<Db>) -> Result<i64, Status>{
    let result = sqlx::query_as::<_, QueryIntegerHelper>("\
        select min(id) from rooms r2 \
        where r2.id not in (select b.room_id from bookings b where (b.check_in_date, b.check_out_date) overlaps ($1::DATE, $2::DATE))
        and hotel_id = $3 and room_type_id = $4) as room_available_count")
        .bind(from_date)
        .bind(to_date)
        .bind(hotel_id)
        .bind(room_type_id)
        .fetch_one(&**conn)
        .await;
    match result {
        Ok(val) => Ok(val.value),
        Err(error) => {
            error!("Sumthing went wong: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

pub async fn calc_total_price(room_type_id: i64, from_date: &str, to_date: &str, conn: &Connection<Db>) -> Result<i64, Status>{
    let result = sqlx::query_as::<_, QueryIntegerHelper>("\
        select prize_per_night from room_types  \
        where id = $1")
        .bind(room_type_id)
        .fetch_one( &**conn)
        .await;
    match result {
        Ok(val) => {
            let begin_date = NaiveDate::parse_from_str(from_date, "%Y-%m-%d").unwrap();
            let end_date  = NaiveDate::parse_from_str(to_date, "%Y-%m-%d").unwrap();
            let day_diff = (begin_date.day() as i64 -end_date.day() as i64);
            Ok(val.value*day_diff)},
        Err(error) => {
            error!("Sumthing went wong: {}", error);
            Err(Status::InternalServerError)
        }
    }
}
#[get("/carousel")]
pub fn get_carousel() -> Result<Json<Vec<String>>, Custom<Json<ErrorResponse>>> {
    let folder_path = PathBuf::from("static/images/carousel");
    let mut paths: Vec<String> = Vec::new();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Ok(file_name) = entry.file_name().into_string() {
                            let file_path = format!("/images/carousel/{}", file_name);
                            paths.push(file_path);
                        }
                    }
                }
            }
        }
        Ok(Json(paths))
    } else {
        let error_response = ErrorResponse {
            error: String::from("Failed to read directory"),
        };
        Err(Custom(Status::InternalServerError, Json(error_response)))
    }
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
                Error::RowNotFound => Err(Status::NotFound),
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
                                                                            and hotel_id = $3 and room_type_id = rt.id) as room_available_count
                                                                            from room_types rt
                                                                            where rt.price_per_night between $4 and $5 and rt.capacity >= $6")
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
                Error::RowNotFound => Err(Status::NotFound),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}

// #[post("/booking", data= "<booking>")]
// pub async fn save_booking(booking: Json<Booking>) -> Result<Json<i32>, Status> {
//
//
//      <Json(12), Status::NotFound>
// }



pub async fn check_room_availability(from_date: &str, to_date: &str, room_type_id: i32, hotel_id: i32, conn: Connection<Db>) -> Result<bool,Status> {
    let result = sqlx::query_as::<_, QueryIntegerHelper>("select count(*) from rooms r2 where r2.id not in (select b.room_id from bookings b where (b.check_in_date, b.check_out_date) overlaps ($1::DATE, $2::DATE))
                                                                            and hotel_id = $3 and room_type_id = $4) as room_available_count")
        .bind(from_date)
        .bind(to_date)
        .bind(hotel_id)
        .bind(room_type_id)
        .fetch_one(&mut conn.into_inner())
        .await;
    match result {
        Ok(nu) => Ok(nu.value > 0),
        Err(error) => {
            error!("Sumthing went wong: {}", error);
            Err(Status::InternalServerError)
        }

    }
}

pub async fn create_guests(guests: Vec<Guest>, mut conn: Connection<Db>) -> Result<Vec<i64>, Status>{
    let mut query_string = String::new();
    for guest in guests{
        query_string.push_str(format!("insert into guests (guest_name, guest_email,\
                                             guest_phone_number, guest_address, guest_passport_number) \
                                             values ({:?}, {:?}, {:?}, {:?}, {:?});",
                                             guest.name, guest.email, guest.phone_number, guest.address, guest.passport_number).as_str());
    }
    let result = sqlx::query(query_string.as_str())
        .execute(&mut *conn)
        .await;
    match result{
        Ok(val) => {
            let mut return_vec: Vec<i64> = Vec::new();
            let guests_old_max_id = get_max_id_off_table("guests", &conn).await.unwrap();
            for i in guests_old_max_id+1..guests_old_max_id+1+val.rows_affected() as i64 {
                return_vec.push(i)
            }
            Ok(return_vec)},
        Err(error) => {
            error!("Sumthing went wong: {}", error);
            Err(Status::InternalServerError)
        }
    }
}



pub async fn create_booking(mut booking: Booking, main_guest_id: i64, conn: &Connection<Db>) -> Result<i64, Status>{

    let total_price = calc_total_price(booking.room_type_id, booking.check_in_date.as_str(), booking.check_out_date.as_str(), &*conn).await.unwrap();

    let result = sqlx::query(format!("Insert into bookings\
                                                                (room_id, guest_id, check_in_date, check_out_date, total_price) \
                                                                values ( {}, {}, {}::DATE, {}::DATE, {})",
                                                                get_next_free_room(booking.hotel_id, booking.room_type_id, booking.check_in_date.as_str(), booking.check_out_date.as_str(), &*conn).await.unwrap(),
                                                                main_guest_id, booking.check_in_date, booking.check_out_date, total_price).as_str())
        .execute(&**conn)
        .await;
    match result{
        Ok(val) => {
            let bookings_old_max_id = get_max_id_off_table("bookings", &*conn).await.unwrap();
            Ok(bookings_old_max_id + 1)},
        Err(error) => {
            error!("Sumthing went wong: {}", error);
            Err(Status::InternalServerError)
        }
    }
}


#[get("/hotels/<id>", data="<booking>")]
pub async fn get_test(booking: Booking, id: i64, conn: State<Connection<Db>>) -> Result<Json<Hotel>, Status> {

    create_booking(booking, id, &conn);
    let result = sqlx::query_as::<_, Hotel>("SELECT id, name, street, house_number, city, state, postal_code, country, phone_number, total_rooms FROM hotels WHERE id = $1")
        .bind(id)
        .fetch_one(&mut *conn)
        .await;

    match result {
        Ok(hotel) => Ok(Json(hotel)),
        Err(error) => {
            error!("Failed to fetch hotel {}: {:?}", id, error);
            match error {
                Error::RowNotFound => Err(Status::NotFound),
                _ => Err(Status::InternalServerError),
            }
        }
    }
}


