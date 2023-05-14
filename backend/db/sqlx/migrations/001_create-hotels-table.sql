CREATE TABLE IF NOT EXISTS hotels
(
    id SERIAL PRIMARY KEY,
    name VARCHAR(255),
    street VARCHAR(255),
    house_number VARCHAR(10),
    city VARCHAR(255),
    state VARCHAR(255),
    postal_code VARCHAR(20),
    country VARCHAR(255),
    phone_number VARCHAR(20),
    total_rooms INTEGER
    );