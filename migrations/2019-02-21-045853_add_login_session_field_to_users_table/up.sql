-- Your SQL goes here
ALTER TABLE users
ADD COLUMN login_session VARCHAR NOT NULL DEFAULT '';
ALTER TABLE users
ADD column user_role VARCHAR NOT NULL DEFAULT 'user';