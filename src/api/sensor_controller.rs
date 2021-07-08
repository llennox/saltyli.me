use crate::{
    config::db::Pool,
    constants,
    schema::hardware::{dsl::*},
    models::{
        hardware::{ Hardware, SafeHardwareOutput },
        user::{ User },
        sensor::{ Sensors, SensorsInput, SensorsOutput },
        response::ResponseBody,
        sensor_type::{ SensorTypes },
        sensor_log::{ SensorLogs, MultiSensorLogsDTO, SensorRangeInput },
    }
};
use bcrypt::{ verify };
use diesel::prelude::*;
use chrono::{NaiveDateTime, Duration};
use chrono::prelude::*;
use actix_web::{web, HttpResponse, HttpRequest, Result};

// POST api/admin/sensor
pub async fn insert(sensor_dto: web::Json<SensorsInput>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    if Sensors::create_sensor(sensor_dto.0.sensor_type_id, sensor_dto.0.hardware_id, &pool.get().unwrap()).is_ok() {
        Ok(HttpResponse::Ok().json(ResponseBody::new(constants::SENSOR_CREATE_SUCCESS, constants::EMPTY)))
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(constants::PROBLEM_CREATING_HARDWARE, constants::EMPTY)))
    }
}

// POST api/sensor-types/get
pub async fn get_sensor_types(pool: web::Data<Pool>) -> Result<HttpResponse> {
    if let Ok(all_sensor_types) = SensorTypes::get_sensor_types(&pool.get().unwrap()) {
        Ok(HttpResponse::Ok().json(ResponseBody::new(&String::from("Sensor types"), all_sensor_types)))
    } else {
        Ok(HttpResponse::BadRequest().json(ResponseBody::new(&String::from("Problem fetching sensor types"), constants::EMPTY)))
    }
}

fn intersect_requested_sensor_ids(requested: Option<&Vec<i32>>, available: Vec<i32>) -> Vec<i32> {
    match requested {
        Some(requested_sensor_ids) => {
            let mut filtered_sensor_ids: Vec<i32> = Vec::new();
            for a in available {
                if requested_sensor_ids.contains(&a) {
                    filtered_sensor_ids.push(a);
                }
            }
            return filtered_sensor_ids;
        },
        None => { 
            return available;
        }
    }
}

fn get_time_range(start: Option<NaiveDateTime>, end: Option<NaiveDateTime>) -> (NaiveDateTime, NaiveDateTime) {
    let mut end_date = Utc::now().naive_utc();
    let mut start_date = Utc::now().naive_utc();
    start_date - Duration::days(364);

    match start {
        Some(provided_start_date) => { start_date = provided_start_date; },
        None => { }
    }
    match end {
        Some(provided_end_date) => { end_date = provided_end_date; },
        None => { }
    }
    return (start_date, end_date);
}

fn get_sensor_info(selected_sensor_id: i32, hardware_sensors_for_group: &Vec<(SensorsOutput, SafeHardwareOutput)>, sensor_types: &Vec<SensorTypes>) -> SensorTypes {
    let selected_sensor = hardware_sensors_for_group.iter().find( |&s| s.0.sensor_id == selected_sensor_id ).expect("Selected sensor not found");
    let sensor_type_info = sensor_types.iter().find( |&st| st.id == selected_sensor.0.sensor_type_id ).expect("Sensor type info not found");
    let sensor_type_info = sensor_type_info.clone();
    let sensor_info = SensorTypes {
        id: selected_sensor_id,
        name: sensor_type_info.name,
        units: sensor_type_info.units 
    };
    return sensor_info;
}

//fn first<T>(v: &Vec<T>) -> Option<&T> {
    //v.first()
//}

#[derive(Serialize)]
pub struct SensorLogRecord {
    pub units: String,
    pub name: String,
    pub sensor_id: i32,
    pub x: Vec<NaiveDateTime>,
    pub y: Vec<f64>,
}

//// POST api/sensor/log/read-v2
pub async fn read_from_log_v2(req: HttpRequest, sensor_range_input: web::Json<SensorRangeInput>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let session_header = req.headers().get(constants::USER_ID).unwrap();
    let user_id = session_header.to_str().unwrap();
    let user = User::find_user_by_id(&user_id.parse::<i32>().unwrap(), &pool.get().unwrap()).expect("user not found");
    let sensor_types = SensorTypes::get_sensor_types(&pool.get().unwrap()).expect("Problem getting sensor types");
    let hardware_sensors_for_group = Hardware::get_hardware_sensors_for_group(
        user.group_id, &pool.get().unwrap()).expect("Problem getting hardware sensors for group");

    let mut available_sensor_ids: Vec<i32> = Vec::new();
    for hardware_sensors in &hardware_sensors_for_group {
        available_sensor_ids.push( hardware_sensors.0.sensor_id );
    }
    let filtered_sensor_ids = intersect_requested_sensor_ids(sensor_range_input.sensor_ids.as_ref(), available_sensor_ids);
    let (start_date, end_date) = get_time_range(sensor_range_input.start, sensor_range_input.end);

    let sensor_records = SensorLogs::get_sensor_log_entries(
        filtered_sensor_ids, start_date, end_date, &pool.get().unwrap()).expect("Problem getting sensor records");
    
    let mut plotly_formed_json_records: Vec<SensorLogRecord> = Vec::new();
    for records in sensor_records {
        let first_record = records.first().expect("Problem getting first record");
        let sensor_info = get_sensor_info(first_record.sensor_id, &hardware_sensors_for_group, &sensor_types);
        let mut x: Vec<NaiveDateTime> = Vec::new();
        let mut y: Vec<f64> = Vec::new();
        for record in records {
            x.push(record.timestamp);
            y.push(record.value);
        }
        let sensor_record_to_push = SensorLogRecord {
          x: x,
          y: y,
          units: sensor_info.units,
          sensor_id: sensor_info.id,
          name: sensor_info.name
        };
        plotly_formed_json_records.push( sensor_record_to_push );
    }
    Ok(HttpResponse::Ok().json(ResponseBody::new(constants::SENSOR_LOG_READ_SUCCESS, plotly_formed_json_records)))
} 

//// POST api/sensor/log/read
pub async fn read_from_log(req: HttpRequest, sensor_range_input: web::Json<SensorRangeInput>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    let session_header = req.headers().get(constants::USER_ID).unwrap();
    let user_id = session_header.to_str().unwrap();
    let user = User::find_user_by_id(&user_id.parse::<i32>().unwrap(), &pool.get().unwrap()).expect("User not found");
    if let Ok(hardware_sensors_for_group) = Hardware::get_hardware_sensors_for_group(user.group_id, &pool.get().unwrap()) {
        let mut available_sensor_ids: Vec<i32> = Vec::new();
        for hardware_sensors in hardware_sensors_for_group {
            available_sensor_ids.push( hardware_sensors.0.sensor_id );
        }
        let filtered_sensor_ids = intersect_requested_sensor_ids(sensor_range_input.sensor_ids.as_ref(), available_sensor_ids);
        let (start_date, end_date) = get_time_range(sensor_range_input.start, sensor_range_input.end);

        if let Ok(sensor_records) = SensorLogs::get_sensor_log_entries(filtered_sensor_ids, start_date,
            end_date, &pool.get().unwrap()) {
               Ok(HttpResponse::Ok().json(ResponseBody::new(constants::SENSOR_LOG_READ_SUCCESS, sensor_records)))
            } else {
               Ok(HttpResponse::Ok().json(ResponseBody::new(constants::SENSOR_LOG_READ_FAILURE, constants::EMPTY)))
            }
    } else {
        Ok(HttpResponse::Ok().json(ResponseBody::new(constants::HARDWARE_SENSOR_READ_FAILURE, constants::EMPTY)))

    }
} 

// POST api/sensor/log/write
//TODO infer hardware from hardware key, much like username - password auth. I may need the hardware ID to find before matching pass 
pub async fn write_to_log(sensors_output: web::Json<MultiSensorLogsDTO>, pool: web::Data<Pool>) -> Result<HttpResponse> {
    if let Ok(hardware_to_verify) = hardware 
        .filter(id.eq(&sensors_output.hardware_id))
        .get_result::<Hardware>(&pool.get().unwrap())
    {
        if !hardware_to_verify.hardware_key.is_empty()
            && verify(&sensors_output.hardware_key, &hardware_to_verify.hardware_key).unwrap()
        {
            for log_write in &sensors_output.data {
                SensorLogs::create_sensor_log_entry(log_write ,&pool.get().unwrap()).unwrap();
            }
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::SENSOR_LOG_ENTRY_SUCCESS, constants::EMPTY)))
    } else {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::SENSOR_LOG_WRITE_FAILURE_NOT_FOUND, constants::EMPTY)))
    }  
    } else {
            Ok(HttpResponse::Ok().json(ResponseBody::new(constants::SENSOR_LOG_WRITE_FAILURE_BAD_PASS, constants::EMPTY)))
    }
} 
