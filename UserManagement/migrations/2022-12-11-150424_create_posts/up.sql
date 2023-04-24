-- Your SQL goes here
CREATE TABLE "users" (
    id uuid PRIMARY KEY,
    name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    mail TEXT UNIQUE NOT NULL
);