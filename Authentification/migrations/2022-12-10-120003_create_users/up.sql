-- Your SQL goes here
CREATE TABLE "admins" (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    mail TEXT UNIQUE NOT NULL,
    password BYTEA NOT NULL,
    change BOOLEAN NOT NULL,
    otp TEXT,
    optactive BOOLEAN NOT NULL
);


CREATE TABLE "users" (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    mail TEXT UNIQUE NOT NULL,
    password BYTEA NOT NULL,
    change BOOLEAN NOT NULL,
    otp TEXT,
    optactive BOOLEAN NOT NULL
);