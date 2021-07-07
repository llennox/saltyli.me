first make some users
then set a users role to "admin" in the db i.e. `update users set user_role = 'admin' where id = 2;`

Make some hardware provide an email to assign it to a user's group
```curl --request POST \
  --url https://api.saltyli.me/api/admin/hardware \
  --header 'Authorization: bearer jwt' \
  --header 'Content-Type: application/json' \
  --data '{
	"email": "testme"
}'```

make a sensor you may need to create sensor types first you have to do this in the db
i.e `INSERT INTO table_name(name, units) VALUES ('BME', 'celsius');`

create a sensor with the right sensor type id and hardware id.
```curl --request POST \
  --url https://api.saltyli.me/api/admin/sensor \
  --header 'Authorization: bearer jwt' \
  --header 'Content-Type: application/json' \
  --data '{"sensor_type_id":2,  "hardware_id":3}'
```
