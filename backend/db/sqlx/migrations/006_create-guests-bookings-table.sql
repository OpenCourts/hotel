CREATE TABLE IF NOT EXISTS guests_bookings
(
    guest_id   INTEGER REFERENCES guests (id),
    booking_id INTEGER REFERENCES bookings (id),
    PRIMARY KEY (guest_id, booking_id)
);