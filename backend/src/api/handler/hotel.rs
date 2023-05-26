use std::fs;
use std::ops::{Deref, Index};
use std::path::{PathBuf};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket_db_pools::Connection;
use sqlx::{Error, FromRow};
use chrono::{Datelike, NaiveDate};
use rocket::State;
use tera::Context;
use crate::mailer;
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
pub struct QueryInteger64Helper{
    value: i64
}

#[derive(FromRow, serde::Serialize)]
pub struct QueryInteger32Helper{
    value: i32
}

#[derive(FromRow, serde::Serialize)]
pub struct QueryIntegerTuple64Helper{
    value1: i64,
    value2: i64
}

#[derive(FromRow, serde::Serialize)]
pub struct QueryIntegerTuple32Helper{
    value1: i32,
    value2: i32
}

pub struct BookingCreationReturnHelper{
    booking_id: i32,
    room_number: i32
}

pub struct RoomBookingHelperReturnHelper{
    room_id: i32,
    room_number: i32
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

pub async fn check_room_availability(from_date: &str, to_date: &str, room_type_id: i32, hotel_id: i32, conn: &Db) -> Result<bool,Status> {
    let result = sqlx::query_as::<_, QueryInteger64Helper>("select count(*) as value from rooms r2 where r2.id not in (select b.room_id from bookings b where (b.check_in_date, b.check_out_date) overlaps ($1::DATE, $2::DATE))
                                                                            and hotel_id = $3 and room_type_id = $4")
        .bind(from_date)
        .bind(to_date)
        .bind(hotel_id)
        .bind(room_type_id)
        .fetch_one(&**conn)
        .await;
    match result {
        Ok(nu) => Ok(nu.value > 0),
        Err(error) => {
            error!("Something went wrong: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

pub async fn create_guests(guests: &[Guest], conn: &Db) -> Result<Vec<i32>, Status>{
    let mut query_string = String::new();
    for guest in guests {
        query_string.push_str(format!("insert into guests (guest_name, guest_email, guest_phone_number, guest_address, guest_passport_number) values ('{}', '{}', '{}', '{}', '{}');", guest.name, guest.email, guest.phone_number, guest.address, guest.passport_number).as_str());
    }
    let result = sqlx::query(query_string.as_str())
        .execute(&**conn)
        .await;
    match result {
        Ok(val) => {
            let mut return_vec: Vec<i32> = Vec::new();
            let guests_old_max_id = get_max_id_off_table("guests", conn).await.unwrap();
            for i in guests_old_max_id..guests_old_max_id+val.rows_affected() as i32 {
                println!("Guestvec hinzugefügt: {}", i);
                return_vec.push(i)
            }
            Ok(return_vec)
        }
        Err(error) => {
            error!("Something went wrong: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

pub async fn create_booking(booking: &Booking,main_guest_id: i32, conn: &Db) -> Result<BookingCreationReturnHelper, Status>{
    println!("in create_booking");
    let mut return_struct= BookingCreationReturnHelper{ booking_id: 0, room_number: 0};
    let total_price = calc_total_price(booking.room_type_id, booking.check_in_date.as_str(), booking.check_out_date.as_str(), conn).await.unwrap();
    let room_struct = get_next_free_room(booking.hotel_id, booking.room_type_id, booking.check_in_date.as_str(), booking.check_out_date.as_str(), conn).await.unwrap();
    let result = sqlx::query(format!("Insert into bookings (room_id, guest_id, check_in_date, check_out_date, total_price) values ( {}, {}, '{}'::DATE, '{}'::DATE, {})", room_struct.room_id, main_guest_id, booking.check_in_date, booking.check_out_date, total_price).as_str())
        .execute(&**conn)
        .await;
    match result {
        Ok(val) => {
            let bookings_id = get_max_id_off_table("bookings", conn).await.unwrap();
            return_struct.booking_id=bookings_id;
            return_struct.room_number=room_struct.room_number;
            Ok(return_struct)
        }
        Err(error) => {
            error!("Something went wrong: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

pub async fn create_guest_booking(guest_ids: &[i32], booking_id: i32, conn: &Db) -> Result<i32, Status>{
    println!("in create_guest_booking");
    let mut query_string = String::new();
    for id in guest_ids {
        query_string.push_str(format!("insert into guests_bookings (guest_id, booking_id) values ({}, {});", *id, booking_id).as_str());
    }
    let result = sqlx::query(query_string.as_str())
        .execute(&**conn)
        .await;
    match result {
        Ok(val) => Ok(booking_id),

        Err(error) => {
            error!("Something went wrong: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

#[post("/booking", data="<booking>")]
pub async fn post_booking(booking: Json<Booking>, conn: &State<Db>) -> Result<String, Status> {
    let loc_booking = booking.into_inner();
    let check = check_room_availability(&loc_booking.check_in_date.as_str(), &loc_booking.check_out_date.as_str(), loc_booking.room_type_id as i32, loc_booking.hotel_id as i32, &conn)
        .await;
    let mut return_id: i32 = -1;
    let mut return_status = Status::InternalServerError;
    let mut return_msg = String::new();
    match check {
        Ok(val) => {if val {
            let guest_ids = create_guests(loc_booking.guests.deref() , &conn)
                .await;
            match guest_ids{
                Ok(vec_ids) => {
                    println!("derefed first element of guest vec: {}", *vec_ids.index(0));
                    let creation = create_booking(&loc_booking, *vec_ids.index(0), &conn).await;

                    match creation {
                        Ok(booking_struct) => {
                            let result = create_guest_booking(&vec_ids, booking_struct.booking_id, &conn)
                                .await;
                            match result{
                                Ok(still_booking_id) => {
                                    let begin_date = NaiveDate::parse_from_str(loc_booking.check_in_date.as_str(), "%Y-%m-%d").unwrap();
                                    let end_date  = NaiveDate::parse_from_str(loc_booking.check_out_date.as_str(), "%Y-%m-%d").unwrap();
                                    let day_diff = end_date.day() as i32 - begin_date.day() as i32;
                                    let price = calc_total_price(loc_booking.room_type_id, loc_booking.check_in_date.as_str(), loc_booking.check_out_date.as_str(), &conn)
                                        .await?;
                                    let mut context = Context::new();
                                    context.insert("recipient", loc_booking.guests.deref().index(0).name.as_str());
                                    context.insert("booking_id", format!("{}", booking_struct.booking_id).as_str());
                                    context.insert("room_number", format!("{}", booking_struct.room_number).as_str());
                                    context.insert("booking_date", loc_booking.check_in_date.as_str());
                                    context.insert("booking_duration", format!("{} Day(s)", day_diff).as_str());
                                    context.insert("booking_amount", format!("{} Monetos", price).as_str());
                                    mailer::send_email(loc_booking.guests.deref().index(0).email.as_str(),"Your Booking at Six Star hotel", "booking.html", context)
                                        .await.expect("PANIC!!!");
                                    /*atch mailres {
                                        Ok(smh) => {
                                            println!("Mail ist raus");
                                            Ok(())
                                        },
                                        Err(val)=>{
                                            error!("{}",val);
                                            Err(Status::ImATeapot)
                                        }
                                    };*/
                                    //return_id = still_booking_id
                                },
                                Err(err_stat)  => return_status = err_stat
                            }
                            return_id = booking_struct.booking_id
                        },
                        Err(err_stat) => return_status = err_stat
                    }
                },
                Err(err_stat) => return_status = err_stat
            }
            return_msg = format!("Created Bocking with id {}", return_id)
        } else {
            return_msg.push_str("Room no longer available, please try again later");
            return_status = Status::Conflict;
        }
        Ok(return_msg)},
        Err(smh) => Err(return_status)
    }
}

pub async fn get_max_id_off_table(table_name: &str, conn: &Db) -> Result<i32, Status>{
    println!("in max_id_off_table");
    let result = sqlx::query_as::<_, QueryInteger32Helper>(format!("Select max(id) as value from {}", table_name).as_str())
        .fetch_one(&**conn)
        .await;
    match result {
        Ok(val) => Ok(val.value),
        Err(error) => {
            error!("Something went wrong: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

pub async fn get_next_free_room(hotel_id: i64, room_type_id: i64, from_date: &str, to_date: &str, conn: &Db) -> Result<RoomBookingHelperReturnHelper, Status>{
    println!(" in next_free_room");
    let mut res_struct = RoomBookingHelperReturnHelper {room_number: 0, room_id: 0};
    let result = sqlx::query_as::<_, QueryIntegerTuple32Helper>("\
        select id as value1, room_number as value2  from rooms r2 \
        where r2.id not in (select b.room_id from bookings b where (b.check_in_date, b.check_out_date) overlaps ($1::DATE, $2::DATE))
        and hotel_id = $3 and room_type_id = $4")
        .bind(from_date)
        .bind(to_date)
        .bind(hotel_id)
        .bind(room_type_id)
        .fetch_one(&**conn)
        .await;
    match result {
        Ok(val) => {
            println!("Id of next free room: {}", val.value1);
            res_struct.room_id = val.value1 as i32;
            res_struct.room_number = val.value2 as i32;
            Ok(res_struct)},
        Err(error) => {
            error!("Something went wrong: {}", error);
            Err(Status::InternalServerError)
        }
    }
}

pub async fn calc_total_price(room_type_id: i64, from_date: &str, to_date: &str, conn: &Db) -> Result<i32, Status>{
    println!("in calc_total");
    let result = sqlx::query_as::<_, QueryInteger32Helper>("\
        select price_per_night as value from room_types  \
        where id = $1")
        .bind(room_type_id)
        .fetch_one(&**conn)
        .await;
    match result {
        Ok(val) => {
            let begin_date = NaiveDate::parse_from_str(from_date, "%Y-%m-%d").unwrap();
            let end_date  = NaiveDate::parse_from_str(to_date, "%Y-%m-%d").unwrap();
            let day_diff = end_date.day() as i32 - begin_date.day() as i32;
            Ok(val.value * day_diff)
        }
        Err(error) => {
            error!("Something went wrong: {}", error);
            Err(Status::InternalServerError)
        }
    }
}


