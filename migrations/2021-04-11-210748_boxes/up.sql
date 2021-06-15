-- Protect these api routes with user_role == "admin"
CREATE TABLE hardware (
    id serial PRIMARY KEY NOT NULL,
    date_created TIMESTAMP NOT NULL,
    hardware_key VARCHAR NOT NULL,
    group_id int NOT NULL REFERENCES groups(id)
);

CREATE TABLE sensor_types (
    id serial PRIMARY KEY NOT NULL,
    name VARCHAR NOT NULL,
    units VARCHAR NOT NULL
);


CREATE TABLE sensors (
    id serial PRIMARY KEY NOT NULL,
    date_created TIMESTAMP NOT NULL,
    hardware_id int NOT NULL REFERENCES hardware(id),
    sensor_type_id int NOT NULL REFERENCES sensor_types(id)
);

CREATE TABLE sensor_logs (
    id serial PRIMARY KEY NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    sensor_id int NOT NULL REFERENCES sensors(id),
    value float not null
);
