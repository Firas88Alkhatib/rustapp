-- Your SQL goes here

CREATE TABLE
    users (
        id SERIAL PRIMARY KEY,
        username VARCHAR NOT NULL UNIQUE,
        password VARCHAR NOT NULL,
        first_name VARCHAR NOT NULL,
        last_name VARCHAR NOT NULL
    );

INSERT INTO
    users (
        id,
        username,
        password,
        first_name,
        last_name
    )
VALUES (
        1,
        'admin',
        -- password: admin
        '$argon2i$v=19$m=4096,t=3,p=1$PjWFvYaPH/58v0gi/LdVvVedu0TRg5JcZUa3VLwxLHc$1r35nzi5G/rIcvte1NprIwao5pQ5EP82ZLhQY3SjtJ0',
        'Firas',
        'Alkhatib'
    );