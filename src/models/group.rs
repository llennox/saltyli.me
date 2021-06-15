use crate::{
    config::db::Connection,
    schema::groups,
    schema::groups::{ dsl::*},
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;

#[derive(Identifiable, Queryable)]
#[table_name = "groups"]
pub struct Groups {
    pub id: i32,
    pub date_created: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Queryable)]
#[table_name = "groups"]
pub struct GroupsDTO {
    pub date_created: NaiveDateTime,
}

impl Groups {
    pub fn create_group(conn: &Connection) -> Result<i32, String> {
            let group = GroupsDTO {
                date_created: Utc::now().naive_utc(),
            };
            let new_group_id: i32 = diesel::insert_into(groups).values(&group).returning(id).get_result(conn).unwrap();
            Ok(new_group_id)
        }
}