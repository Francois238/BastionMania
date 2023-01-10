-- Your SQL goes here

CREATE TABLE "admins" (
    id SERIAL PRIMARY KEY,
    name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    mail TEXT UNIQUE NOT NULL,
    password BYTEA NOT NULL
);