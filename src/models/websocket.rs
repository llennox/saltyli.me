use crate::models::sensor_log::{ SensorLogsDTO };


#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum WebsocketMessageType {
    Auth { key: String, hardware_id: i32 },
    WriteSensorLog { sensor_log_to_write: SensorLogsDTO },
}

#[derive(Queryable, Deserialize)]
pub struct WebSocketAuthInput {
    pub hardware_id: i32,
    pub hardware_key: String,
}