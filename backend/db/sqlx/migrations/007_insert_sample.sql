-- Hotels
INSERT INTO hotels (name, street, house_number, city, state, postal_code, country, phone_number, total_rooms)
VALUES ('Six Star Hotels Germany Berlin', 'Karl-Liebknecht-Strasse', '5', 'Berlin', '', '10178', 'Germany',
        '+49-30-1234-5678', 200),
       ('Six Star Hotels France Paris', 'Rue de Rivoli', '5', 'Paris', '', '75001', 'France', '+33-1-2345-6789',
        150),
       ('Six Star Hotels USA New York', '5th Ave', '456', 'New York', 'NY', '10016', 'USA', '+1-555-234-5678', 100),
       ('Six Star Hotels Japan Tokyo', 'Nishishinjuku', '6-6-2', 'Shinjuku City', 'Tokyo', '160-0023', 'Japan',
        '+81-3-3344-5111', 250);

-- Room Types
INSERT INTO room_types (name, description, size, capacity, amenities, price_per_night, image_url)
VALUES ('Standard', 'A standard room with a queen size bed and a bathroom.', 300, 2, 'TV, Wi-Fi, Coffee maker',
        100, '/images/rooms/room_standard.jpg'),
       ('Deluxe', 'A deluxe room with a king size bed, a bathroom and a balcony with a view.', 400, 2,
        'TV, Wi-Fi, Coffee maker, Balcony', 150, '/images/rooms/room_deluxe.jpg'),
       ('Suite', 'A luxurious suite with a king size bed, a living room, a kitchenette and a bathroom.', 600, 4,
        'TV, Wi-Fi, Coffee maker, Kitchenette, Living room', 300, '/images/rooms/room_suite.jpg');

-- Rooms
INSERT INTO rooms (hotel_id, room_number, room_type_id)
VALUES (1, 101, 1),
       (1, 102, 1),
       (1, 201, 2),
       (1, 202, 2),
       (2, 301, 1),
       (2, 302, 1),
       (2, 401, 2),
       (2, 402, 2),
       (2, 501, 3),
       (2, 502, 3);

-- Guests
INSERT INTO guests (guest_name, guest_email, guest_phone_number, guest_address, guest_passport_number)
VALUES ('John Smith', 'john.smith@gmail.com', '555-111-2222', '123 Main Street, New York, NY', 'ABC123456'),
       ('Jane Doe', 'jane.doe@yahoo.com', '555-222-3333', '456 Ocean Drive, Miami, FL', 'DEF789012'),
       ('Robert Johnson', 'robert.johnson@hotmail.com', '555-333-4444', '789 Alpine Road, Aspen, CO', 'GHI345678');

-- Bookings
INSERT INTO bookings(room_id, guest_id, check_in_date, check_out_date, total_price)
VALUES (1, 1, '2023-06-01', '2023-06-05', 400.00),
       (3, 2, '2023-07-10', '2023-07-15', 750.00),
       (5, 3, '2023-08-20', '2023-08-25', 1200.00),
       (6, 1, '2023-09-05', '2023-09-10', 500.00),
       (8, 2, '2023-10-01', '2023-10-05', 600.00),
       (9, 3, '2023-11-15', '2023-11-20', 1500.00),
       (10, 1, '2023-12-01', '2023-12-05', 1000.00),
       (2, 2, '2024-01-15', '2024-01-20', 600.00),
       (4, 3, '2024-02-10', '2024-02-15', 750.00),
       (7, 1, '2024-03-20', '2024-03-25', 800.00);

-- Guest Bookings
INSERT INTO guests_bookings (guest_id, booking_id)
VALUES (1, 1),
       (2, 2),
       (3, 3),
       (1, 4),
       (2, 5),
       (3, 6),
       (1, 7),
       (2, 8),
       (3, 9),
       (1, 10);