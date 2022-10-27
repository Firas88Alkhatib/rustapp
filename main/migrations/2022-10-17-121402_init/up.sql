-- Your SQL goes here

CREATE TABLE
    users (
        id SERIAL PRIMARY KEY,
        username VARCHAR NOT NULL UNIQUE,
        password VARCHAR NOT NULL,
        roles TEXT ARRAY NOT NULL,
        first_name VARCHAR NOT NULL,
        last_name VARCHAR NOT NULL
    );

INSERT INTO
    users (
        username,
        password,
        roles,
        first_name,
        last_name
    )
VALUES (
        'admin',
        -- password: admin
        '$argon2i$v=19$m=4096,t=3,p=1$PjWFvYaPH/58v0gi/LdVvVedu0TRg5JcZUa3VLwxLHc$1r35nzi5G/rIcvte1NprIwao5pQ5EP82ZLhQY3SjtJ0',
        ARRAY ['ROLE_ADMIN',
        'ROLE_USER'],
        'Firas',
        'Alkhatib'
    );