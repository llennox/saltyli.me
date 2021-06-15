-- Your SQL goes here
CREATE TABLE login_history
(
    id Serial PRIMARY KEY NOT NULL,
    user_id int NOT NULL REFERENCES users(id),
    login_timestamp TIMESTAMP NOT NULL
);
