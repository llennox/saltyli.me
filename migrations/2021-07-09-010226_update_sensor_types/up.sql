-- Your SQL goes here


ALTER TABLE sensor_types
ADD COLUMN label VARCHAR;

UPDATE sensor_types SET units = 'relative humidity', label = 'humidity' WHERE id = 1;
UPDATE sensor_types SET label = 'temperature' WHERE id = 2;

ALTER TABLE sensor_types
ALTER COLUMN label SET NOT NULL;