CREATE TABLE IF NOT EXISTS  rooms (
                       id SERIAL PRIMARY KEY,
                       hotel_id INTEGER REFERENCES hotels(id) ON DELETE CASCADE,
                       room_number INTEGER NOT NULL,
                       description VARCHAR NOT NULL,
                       size INTEGER NOT NULL,
                       capacity INTEGER NOT NULL,
                       has_tv BOOLEAN NOT NULL,
                       has_wifi BOOLEAN NOT NULL,
                       has_ac BOOLEAN NOT NULL,
                       has_jacuzzi BOOLEAN NOT NULL,
                       has_sauna BOOLEAN NOT NULL,
                       has_butler_service BOOLEAN NOT NULL,
                       has_premium_bar BOOLEAN NOT NULL,
                       has_ocean_view BOOLEAN NOT NULL,
                       has_private_pool BOOLEAN NOT NULL,
                       has_private_beach BOOLEAN NOT NULL,
                       price_per_night DECIMAL(10,2) NOT NULL
);
