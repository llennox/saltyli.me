-- Your SQL goes here
CREATE TABLE groups (
    id serial PRIMARY KEY NOT NULL,
    date_created TIMESTAMP NOT NULL
);
ALTER TABLE users
ADD column group_id int NOT NULL REFERENCES groups(id);