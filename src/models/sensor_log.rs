//CREATE TABLE sensor_logs (
    //id serial PRIMARY KEY NOT NULL,
    //timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    //sensor_id int NOT NULL REFERENCES sensors(id),
    //value int NOT NULL REFERENCES sensor_types(id)
//);

use crate::{
    config::db::Connection,
    constants,
    schema::sensor_logs,
    schema::sensor_logs::{ dsl::* },
};
use chrono::{NaiveDateTime};
use diesel::prelude::*;


#[derive(Identifiable, Queryable, Serialize)]
#[table_name = "sensor_logs"]
pub struct SensorLogs {
    pub id: i32,
    pub timestamp: NaiveDateTime,
    pub sensor_id: i32,
    pub value: f64,
}

#[derive(Insertable, Serialize, Deserialize, Queryable)]
#[table_name = "sensor_logs"]
pub struct SensorLogsDTO {
    pub timestamp: NaiveDateTime,
    pub sensor_id: i32,
    pub value: f64,
}

#[derive(Deserialize)]
pub struct MultiSensorLogsDTO {
    pub hardware_id: i32,
    pub hardware_key: String,
    pub data: Vec<SensorLogsDTO>,
}

#[derive(Deserialize)]
pub struct SensorRangeInput {
    pub sensor_ids: Option<Vec<i32>>,
    pub start: Option<NaiveDateTime>,
    pub end: Option<NaiveDateTime>,
}

impl SensorLogs {
    pub fn create_sensor_log_entry(sensor_log: &SensorLogsDTO, conn: &Connection) -> Result<String, String> {
            let _sensor_log_result = diesel::insert_into(sensor_logs).values(sensor_log).execute(conn).unwrap();
            eprint!("{:?}", _sensor_log_result);
            Ok(constants::EMPTY.to_string())
        }

    pub fn get_sensor_log_entries(sensor_ids: Vec<i32>, start: NaiveDateTime, end: NaiveDateTime, conn: &Connection)
     -> Result<Vec<Vec<SensorLogs>>, String> {
            let mut multi_sensor_logs: Vec<Vec<SensorLogs>> = Vec::new();
            for requested_sensor_id in sensor_ids {
                let sensor_log_for_sensor_id: Vec<SensorLogs> = sensor_logs.filter(sensor_logs::sensor_id.eq(requested_sensor_id))
                .filter(sensor_logs::timestamp.between(start, end))
                    .select((sensor_logs::id, sensor_logs::timestamp, sensor_logs::sensor_id, sensor_logs::value))
                        .order(sensor_logs::timestamp.desc()).load::<SensorLogs>(conn).unwrap();
                multi_sensor_logs.push(sensor_log_for_sensor_id);
            }
            Ok(multi_sensor_logs)
        }
}
