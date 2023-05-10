use rocket::http::Status;
use rocket::serde::json::Json;
use crate::api::response::GenericResponse;

#[get("/hotels")]
pub fn get_hotels() -> Result<Json<GenericResponse>, Status> {
    let response_json = GenericResponse {
        status: "success".to_string(),
        message: "Hello, World!".to_string(),
    };

    Ok(Json(response_json))
}