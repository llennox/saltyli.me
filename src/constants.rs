// Messages
pub const MESSAGE_OK: &str = "ok";
pub const MESSAGE_CAN_NOT_FETCH_DATA: &str = "Can not fetch data";
pub const MESSAGE_CAN_NOT_INSERT_DATA: &str = "Can not insert data";
pub const MESSAGE_CAN_NOT_UPDATE_DATA: &str = "Can not update data";
pub const MESSAGE_CAN_NOT_DELETE_DATA: &str = "Can not delete data";
pub const MESSAGE_SIGNUP_SUCCESS: &str = "Signup successfully";
pub const SENSOR_CREATE_SUCCESS: &str = "sensor created successfully";
// pub const MESSAGE_SIGNUP_FAILED: &str = "Error while signing up, please try again";
pub const MESSAGE_LOGIN_SUCCESS: &str = "Login successfully";
pub const MESSAGE_LOGIN_FAILED: &str = "Wrong username or password, please try again";
pub const MESSAGE_USER_NOT_FOUND: &str = "User not found, please signup";
pub const MESSAGE_LOGOUT_SUCCESS: &str = "Logout successfully";
pub const MESSAGE_PROCESS_TOKEN_ERROR: &str = "Error while processing token";
pub const MESSAGE_INVALID_TOKEN: &str = "Invalid token, please login again";
pub const MESSAGE_INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

// Bad request messages
pub const MESSAGE_TOKEN_MISSING: &str = "Token is missing";

// Headers
pub const AUTHORIZATION: &str = "Authorization";

//login session
pub const USER_ID: &str = "user_id";
//pub const ADMIN_ERROR: &str = "Must be Admin";

// Misc
pub const EMPTY: &str = "";

// ignore routes
pub const IGNORE_ROUTES: [&str; 7] = [ "/", "/api/ws" ,"/api/sensor/log/read", "/api/sensor/log/write","/api/ping", "/api/auth/signup", "/api/auth/login"];
// admin routes
pub const ADMIN_ROUTES: &str = "/api/admin";


// sensor log messages
pub const SENSOR_LOG_ENTRY_SUCCESS: &str = "Succesfully saved value to sensor log";
pub const SENSOR_LOG_READ_SUCCESS: &str = "Sensor log entries";
pub const SENSOR_LOG_READ_FAILURE: &str = "Sensor log entries";
pub const SENSOR_LOG_WRITE_FAILURE_NOT_FOUND: &str = "Hardware with that id not found";
pub const SENSOR_LOG_WRITE_FAILURE_BAD_PASS: &str = "Bad pass for hardware";

pub const HARDWARE_SENSOR_READ_FAILURE: &str = "Hardware sensors read failure";

//hardware
pub const PROBLEM_CREATING_HARDWARE: &str = "There was a problem creating that hardware";
