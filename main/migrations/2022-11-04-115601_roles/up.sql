-- Your SQL goes here

CREATE TABLE
    roles (
        id SERIAL PRIMARY KEY,
        name VARCHAR NOT NULL UNIQUE
    );

-- seeding default roles

INSERT INTO roles (id, name)
VALUES (1, 'ROLE_ADMIN'), (2, 'ROLE_USER');