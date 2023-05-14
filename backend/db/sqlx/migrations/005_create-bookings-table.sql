CREATE TABLE IF NOT EXISTS bookings
(
    id             SERIAL PRIMARY KEY,
    room_id        INTEGER REFERENCES rooms (id),
    guest_id       INTEGER REFERENCES guests (id),
    check_in_date  TIMESTAMP,
    check_out_date TIMESTAMP,
    total_price    DECIMAL(10, 2)
);