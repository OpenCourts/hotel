CREATE TABLE IF NOT EXISTS guests
(
    id                    SERIAL PRIMARY KEY,
    guest_name            VARCHAR(255) NOT NULL,
    guest_email           VARCHAR(255) NOT NULL,
    guest_phone_number    VARCHAR(20)  NOT NULL,
    guest_address         VARCHAR(255),
    guest_passport_number VARCHAR(20)
);
