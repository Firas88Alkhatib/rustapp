-- Your SQL goes here

CREATE TABLE
    users_roles (
        id SERIAL PRIMARY KEY,
        user_id INT NOT NULL,
        role_id INT NOT NULL,
        FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
        FOREIGN KEY (role_id) REFERENCES roles(id)
    );

-- seeding admin user relation

INSERT INTO
    users_roles (id, user_id, role_id)
VALUES (1, 1, 1), (2, 1, 2)