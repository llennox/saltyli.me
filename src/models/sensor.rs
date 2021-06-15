//CREATE TABLE sensors (
    //id serial PRIMARY KEY NOT NULL,
    //date_created TIMESTAMP WITH TIME ZONE NOT NULL,
    //hardware_id int NOT NULL REFERENCES boxes(id),
    //sensor_type_id int NOT NULL REFERENCES sensor_types(id)
//);

use crate::{
    config::db::Connection,
    schema::sensors,
    schema::sensors::{ dsl::* },
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;


//WIP

#[derive(Identifiable, Queryable, Serialize)]
#[table_name = "sensors"]
pub struct Sensors {
    pub id: i32,
    pub date_created: NaiveDateTime,
    pub hardware_id: i32,
    pub sensor_type_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Queryable)]
#[table_name = "sensors"]
pub struct SensorsDTO {
    pub date_created: NaiveDateTime,
    pub hardware_id: i32,
    pub sensor_type_id: i32,
}

#[derive(Deserialize)]
pub struct SensorsInput {
    pub hardware_id: i32,
    pub sensor_type_id: i32,
}

#[derive(Serialize, Queryable)]
pub struct SensorsOutput {
    pub sensor_id: i32,
    pub date_created: NaiveDateTime,
    pub hardware_id: i32,
    pub sensor_type_id: i32,
}

impl Sensors {
    pub fn create_sensor(associate_with_sensor_type: i32, associate_with_hardware_id: i32, conn: &Connection) -> Result<(), String> {
            let sensor = SensorsDTO {
                date_created: Utc::now().naive_utc(),
                hardware_id: associate_with_hardware_id,
                sensor_type_id: associate_with_sensor_type,
            };
            if diesel::insert_into(sensors).values(&sensor).execute(conn).is_ok() {
                Ok(())
            } else {
                Err(format!("sensor log issue sensor_log_id: {}", &sensor.date_created))
            }
        }
}
