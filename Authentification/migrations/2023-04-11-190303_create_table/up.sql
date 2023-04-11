-- Your SQL goes here
CREATE TABLE "admins" (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    mail TEXT UNIQUE NOT NULL,
    password BYTEA,
    change BOOLEAN,
    otp TEXT,
    otpactive BOOLEAN
);


CREATE TABLE "users" (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    mail TEXT UNIQUE NOT NULL,
    password BYTEA,
    change BOOLEAN,
    otp TEXT,
    otpactive BOOLEAN
);