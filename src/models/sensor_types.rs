//CREATE TABLE sensor_types (
    //id serial PRIMARY KEY NOT NULL,
    //name VARCHAR NOT NULL,
    //units VARCHAR NOT NULL
//);
use crate::{
    config::db::Connection,
    schema::sensor_types,
    schema::sensor_types::{ dsl::* },
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;


//WIP

#[derive(Identifiable, Queryable)]
#[table_name = "sensor_types"]
pub struct SensorTypes {
    pub id: i32,
    pub date_created: NaiveDateTime,
    pub box_key: String,
    pub group_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Queryable)]
#[table_name = "sensor_types"]
pub struct SensorTypesDTO {
    pub date_created: NaiveDateTime,
    pub box_key: String,
    pub group_id: i32,
}

impl Boxes {
    pub fn create_box(associate_with_group_id: i32, conn: &Connection) -> Result<i32, String> {
            let new_box = BoxesDTO {
                date_created: Utc::now().naive_utc(),
                box_key: String::from("Placeholder, secret goes here"),
                group_id: associate_with_group_id,
            };
            let new_box_id: i32 = diesel::insert_into(boxes).values(&new_box).returning(id).get_result(conn).unwrap();
            Ok(new_box_id)
        }
}
