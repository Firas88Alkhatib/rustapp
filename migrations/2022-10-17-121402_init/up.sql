-- Your SQL goes here

CREATE TABLE
    users (
        id SERIAL PRIMARY KEY,
        first_name VARCHAR NOT NULL,
        last_name VARCHAR NOT NULL
    )