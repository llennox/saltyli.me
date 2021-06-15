write test suite for user creation, 

admin::
    POST admin/hardware for hardware creation from admin ✅
    POST admin/sensor to create sensor and associate with hardware ✅ 
user:: 
    update me to return user email etc.  ✅

    POST /hardware/sensors to get all the available sensor ids ✅
    POST /sensor-types/get ✅

    I need a seperate auth for hardware
    POST /sensor/log/write to create value for sensor ✅
    POST /sensor/log/read to get values from sensor ✅

    Websocket write auth 
    POST /api/ws to create value for sensor instead of the post request, auth once, stay alive
    I need to write the client for :this: too, probably in C 
     
Deploy after you have auth for hardware
then start on the FE, register, login, graphs
after you have graphs solder

