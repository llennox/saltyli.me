use crate::{
    config::db::Connection,
    schema::boxes,
    schema::boxes::{ dsl::* },
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

//CREATE TABLE boxes (
    //id serial PRIMARY KEY NOT NULL,
    //date_created TIMESTAMP WITH TIME ZONE NOT NULL,
    //box_key VARCHAR NOT NULL,
    //group_id int NOT NULL REFERENCES groups(id)
//);

#[derive(Identifiable, Queryable)]
#[table_name = "boxes"]
pub struct Boxes {
    pub id: i32,
    pub date_created: NaiveDateTime,
    pub box_key: String,
    pub group_id: i32,
}

#[derive(Insertable, Serialize, Deserialize, Queryable)]
#[table_name = "boxes"]
pub struct BoxesDTO {
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