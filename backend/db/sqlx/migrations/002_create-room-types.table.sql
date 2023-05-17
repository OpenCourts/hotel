CREATE TABLE IF NOT EXISTS room_types
(
    id              SERIAL PRIMARY KEY,
    name            VARCHAR(255),
    description     TEXT,
    size            INTEGER,
    capacity        INTEGER,
    amenities       VARCHAR(255),
    price_per_night INTEGER
);
