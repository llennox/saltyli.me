use crate::{
    config::db::Pool,
    constants,
    models::{
        hardware::{Hardware},
        response::ResponseBody,
        user::{ User, UserEmail },
    },
};

use actix_web::{web, HttpRequest, HttpResponse, Result};

// POST api/admin/hardware
pub async fn insert(user_email: web::Json<UserEmail>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let user = User::find_user_by_email(&user_email.email, &pool.get().unwrap()).unwrap();
    if let Ok(hardware_secrets) = Hardware::create_hardware(user.group_id, &pool.get().unwrap()) {
        Ok(HttpResponse::Ok().json(ResponseBody::new(&String::from("New hardware secret"), hardware_secrets)))
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::PROBLEM_CREATING_HARDWARE, constants::EMPTY)))
    }
} 

// POST api/hardware/get
pub async fn get_hardware_for_user(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let session_header = req.headers().get(constants::USER_ID).unwrap();
    let user_id = session_header.to_str().unwrap();
    let user = User::find_user_by_id(&user_id.parse::<i32>().unwrap(), &pool.get().unwrap()).unwrap();
    if let Ok(hardware_for_group) = Hardware::get_all_hardware_for_group(user.group_id, &pool.get().unwrap()) {
        Ok(HttpResponse::Ok().json(ResponseBody::new(&String::from("All hardware for group"), hardware_for_group)))
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::HARDWARE_SENSOR_READ_FAILURE, constants::EMPTY)))
    }
} 

// POST api/hardware/sensors/get
pub async fn get_hardware_sensors_for_user(req: HttpRequest, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let session_header = req.headers().get(constants::USER_ID).unwrap();
    let user_id = session_header.to_str().unwrap();
    let user = User::find_user_by_id(&user_id.parse::<i32>().unwrap(), &pool.get().unwrap()).unwrap();
    if let Ok(hardware_sensors_for_group) = Hardware::get_hardware_sensors_for_group(user.group_id, &pool.get().unwrap()) {
        Ok(HttpResponse::Ok().json(ResponseBody::new(&String::from("All hardware and sensors for group"), hardware_sensors_for_group)))
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::HARDWARE_SENSOR_READ_FAILURE, constants::EMPTY)))
    }
} 
