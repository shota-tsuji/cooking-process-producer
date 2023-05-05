USE cooking;

CREATE TABLE IF NOT EXISTS recipes (
    id INT NOT NULL,
    title VARCHAR(140) NOT NULL,
    description VARCHAR(1000),
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS resources (
    id INT NOT NULL,
    name VARCHAR(140) NOT NULL,
    amount INT NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS steps (
    id INT NOT NULL,
    recipe_id INT NOT NULL,
    description VARCHAR(140) NOT NULL,
    resource_id INT NOT NULL,
    order_number INT UNSIGNED NOT NULL,
    duration INT NOT NULL,
    FOREIGN KEY (recipe_id)
        REFERENCES recipes(id),
    FOREIGN KEY (resource_id)
        REFERENCES resources(id),
    PRIMARY KEY (id)
);
