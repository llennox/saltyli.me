use crate::{
    config::db::Connection,
    schema::hardware,
    schema::sensors,
    models::sensor::{ SensorsOutput },
    schema::hardware::{ dsl::* },
};
use chrono::{NaiveDateTime, Utc};
use diesel::prelude::*;
use passwords::PasswordGenerator;
use bcrypt::{hash, DEFAULT_COST};


//CREATE TABLE hardwares (
    //id serial PRIMARY KEY NOT NULL,
    //date_created TIMESTAMP WITH TIME ZONE NOT NULL,
    //box_key VARCHAR NOT NULL,
    //group_id int NOT NULL REFERENCES groups(id)
//);

#[derive(Identifiable, Queryable)]
#[table_name = "hardware"]
pub struct Hardware {
    pub id: i32,
    pub date_created: NaiveDateTime,
    pub hardware_key: String,
    pub group_id: i32,
}

#[derive(Queryable, Serialize)]
pub struct SafeHardwareOutput {
    pub hardware_id: i32,
    pub date_created: NaiveDateTime,
    pub group_id: i32,
}

#[derive(Queryable, Serialize)]
pub struct UnSafeHardwareOutput {
    pub hardware_id: i32,
    pub password: String,
}

#[derive(Insertable, Serialize, Deserialize, Queryable)]
#[table_name = "hardware"]
pub struct HardwareDTO {
    pub date_created: NaiveDateTime,
    pub hardware_key: String,
    pub group_id: i32,
}


impl Hardware {
    pub fn create_hardware(associate_with_group_id: i32, conn: &Connection) -> Result<UnSafeHardwareOutput, String> {

            let pgen = PasswordGenerator {
                length: 12,
                numbers: true,
                lowercase_letters: true,
                uppercase_letters: true,
                symbols: false,
                spaces: false,
                exclude_similar_characters: false,
                strict: true,
            };
            let hardware_password: String = pgen.generate_one().unwrap();

            let hashed_pass: String = hash(&hardware_password, DEFAULT_COST).unwrap();
            let new_hardware = HardwareDTO {
                date_created: Utc::now().naive_utc(),
                hardware_key: hashed_pass,
                group_id: associate_with_group_id,
            };
            let new_hardware_id: i32 = diesel::insert_into(hardware).values(&new_hardware).returning(id).get_result(conn).unwrap();
            // Make this into some type of response object
            let hardware_secrets = UnSafeHardwareOutput {
                hardware_id: new_hardware_id,
                password: hardware_password
            };
            Ok(hardware_secrets)
        }

        
    pub fn get_all_hardware_for_group(group_id_associated_with_hardware: i32, conn: &Connection) -> Result<Vec<SafeHardwareOutput>, String> {
        let hardware_for_group: Vec<SafeHardwareOutput> = 
            hardware.filter(hardware::group_id.eq(group_id_associated_with_hardware))
                .select((hardware::id, hardware::date_created, hardware::group_id)).load::<SafeHardwareOutput>(conn).unwrap();
        Ok(hardware_for_group)
    }
    pub fn get_hardware_sensors_for_group(group_id_associated_with_hardware: i32, conn: &Connection) -> Result<Vec<(SensorsOutput, SafeHardwareOutput)>, String> {
        let sensors_for_group: Vec<(SensorsOutput, SafeHardwareOutput)> = sensors::table.inner_join(hardware::table)
            .filter(hardware::group_id.eq(group_id_associated_with_hardware))
                .select(
                    ((sensors::id, sensors::date_created, sensors::hardware_id, sensors::sensor_type_id),
                    (hardware::id, hardware::date_created, hardware::group_id)))
                .load::<(SensorsOutput, SafeHardwareOutput)>(conn).unwrap();
        Ok(sensors_for_group)
    }
}