CREATE TABLE IF NOT EXISTS guests (
                        id SERIAL PRIMARY KEY,
                        room_id INTEGER REFERENCES rooms(id) ON DELETE CASCADE,
                        guest_name VARCHAR NOT NULL,
                        guest_email VARCHAR NOT NULL,
                        guest_address VARCHAR NOT NULL,
                        guest_passport_number VARCHAR NOT NULL,
                        check_in_date TIMESTAMP NOT NULL,
                        check_out_date TIMESTAMP NOT NULL
);