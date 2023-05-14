CREATE TABLE IF NOT EXISTS rooms
(
    id           SERIAL PRIMARY KEY,
    hotel_id     INTEGER REFERENCES hotels (id),
    room_number  INTEGER,
    room_type_id INTEGER REFERENCES room_types (id)
);

