-- Your SQL goes here

CREATE TABLE
    categories (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL
    );

INSERT INTO categories (id,name) VALUES (1,'test') 