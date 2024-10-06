-- Create Users Table
CREATE TABLE Users (
    user_id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL
);

-- Create Cars Table
CREATE TABLE Cars (
    car_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES Users(user_id) ON DELETE CASCADE,
    make VARCHAR(50) NOT NULL,
    model VARCHAR(50) NOT NULL,
    year INT NOT NULL
);

-- Create Car Check-Ins Table
CREATE TABLE CarCheckIns (
    checkin_id SERIAL PRIMARY KEY,
    car_id INT NOT NULL REFERENCES Cars(car_id) ON DELETE CASCADE,
    checkin_date DATE NOT NULL,
    miles INT NOT NULL,
    tank_left INT NOT NULL
);

-- Create Fill-Ups Table
CREATE TABLE FillUps (
    fillup_id SERIAL PRIMARY KEY,
    car_id INT NOT NULL REFERENCES Cars(car_id) ON DELETE CASCADE,
    fillup_date VARCHAR(10) NOT NULL,
    gallons_filled DECIMAL(5,2) NOT NULL,
    price_per_gallon DECIMAL(5,2) NOT NULL,
    new_range INT NOT NULL
);
