-- Your SQL goes here
CREATE TABlE
    productscategories (
        id SERIAL PRIMARY KEY,
        product_id INT NOT NULL,
        category_id INT NOT NULL,
        FOREIGN KEY (product_id) REFERENCES products(id) ON DELETE CASCADE,
        FOREIGN KEY (category_id) REFERENCES categories(id) ON DELETE CASCADE
    )
