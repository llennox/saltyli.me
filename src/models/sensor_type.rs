//CREATE TABLE sensor_types (
    //id serial PRIMARY KEY NOT NULL,
    //name VARCHAR NOT NULL,
    //units VARCHAR NOT NULL
//);
use crate::{
    config::db::Connection,
    constants,
    schema::sensor_types,
    schema::sensor_types::{ dsl::* },
};
use diesel::prelude::*;

//WIP
#[derive(Identifiable, Queryable, Serialize, Clone)]
#[table_name = "sensor_types"]
pub struct SensorTypes {
    pub id: i32,
    pub name: String,
    pub units: String,
    pub label: String,
}

#[derive(Insertable, Serialize, Deserialize, Queryable)]
#[table_name = "sensor_types"]
pub struct SensorTypesDTO {
    pub name: String,
    pub units: String,
}

impl SensorTypes {
    pub fn create_sensor_log(sensor_type: SensorTypesDTO, conn: &Connection) -> Result<String, String> {
            if diesel::insert_into(sensor_types).values(&sensor_type).execute(conn).is_ok() {
                Ok(constants::MESSAGE_SIGNUP_SUCCESS.to_string())
            } else {
                Err(format!("sensor log issue sensor_log_id: {}", &sensor_type.name))
            }
        }

    pub fn get_sensor_types(conn: &Connection) -> Result<Vec<SensorTypes>, String> {
        let all_sensor_types: Vec<SensorTypes> = 
            sensor_types::table.select(sensor_types::all_columns).load::<SensorTypes>(conn).unwrap();
        Ok(all_sensor_types)

    }
}
