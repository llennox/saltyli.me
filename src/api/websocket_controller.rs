//! Simple echo websocket server.
//! Open `http://localhost:8080/ws/index.html` in browser
//! or [python console client](https://github.com/actix/examples/blob/master/websocket/websocket-client.py)
//! could be used for testing.

use std::time::{Duration, Instant};
use crate::{
    config::db::Pool,
    models::hardware::{Hardware},
    models::websocket::{WebsocketMessageType},
    models::sensor_log::{ SensorLogs },
    schema::hardware::{dsl::*},
};
use actix::prelude::*;
use actix_web::{web, Error, HttpRequest, HttpResponse };
use actix_web_actors::ws;
use bcrypt::{ verify };
use diesel::prelude::*;
use serde_json::{Value};


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub async fn ws_index(r: HttpRequest, stream: web::Payload, pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    info!("HI");
    let res = ws::start(MyWebSocket::new(pool), &r, stream);
    res
}

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    authed: bool,
    pool: web::Data<Pool>
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                let v: Result<Value, serde_json::Error> = serde_json::from_str(&text);
                    let json_msg: Value = v.unwrap();
                    match serde_json::from_value(json_msg).expect("bad json") {
                        WebsocketMessageType::Auth { key, hardware_id } => {
                            if let Ok(hardware_to_verify) = hardware 
                                .filter(id.eq(hardware_id))
                                .get_result::<Hardware>(&self.pool.get().unwrap())
                            {
                                if !hardware_to_verify.hardware_key.is_empty()
                                    && verify(&String::from(key), &hardware_to_verify.hardware_key).unwrap()
                                {
                                    self.authed = true;
                                    ctx.text("Authed")
                                } else {
                                    ctx.text("Bad pass");
                                    ctx.stop();
                                }  
                            } else {
                                ctx.text("Hardware not found");
                                ctx.stop();
                            }
                        },
                        WebsocketMessageType::WriteSensorLog { sensor_log_to_write } => {
                            if self.authed {
                                SensorLogs::create_sensor_log_entry(&sensor_log_to_write ,&self.pool.get().unwrap()).unwrap();
                                ctx.text("success");
                            } else {
                                ctx.text("Auth again");
                                ctx.stop();
                            }
                        },
                        _ => {
                            ctx.text("type not found");
                            ctx.stop();
                        }
                    }
            },
            Ok(ws::Message::Binary(_bin)) => ctx.stop(),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl MyWebSocket {
    fn new(pool: web::Data<Pool>) -> Self {
        Self { 
            hb: Instant::now(),
            authed: false,
            pool: pool
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}
