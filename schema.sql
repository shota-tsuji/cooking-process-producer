USE cooking;

CREATE TABLE IF NOT EXISTS recipes (
    id INT NOT NULL AUTO_INCREMENT,
    title VARCHAR(140) NOT NULL,
    description VARCHAR(1000),
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS resources (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(140) NOT NULL,
    amount INT NOT NULL,
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS steps (
    id VARCHAR(36) NOT NULL,
    recipe_id INT NOT NULL,
    description VARCHAR(140) NOT NULL,
    resource_id BIGINT UNSIGNED NOT NULL,
    order_number INT UNSIGNED NOT NULL,
    duration INT NOT NULL,
    FOREIGN KEY (recipe_id)
        REFERENCES recipes(id),
    FOREIGN KEY (resource_id)
        REFERENCES resources(id),
    PRIMARY KEY (id)
);

CREATE TABLE IF NOT EXISTS processes (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    name VARCHAR(140) NOT NULL,
    PRIMARY KEY (id)
);

/*
 Join table
 https://react.dev/reference/react/useState#ive-updated-the-state-but-logging-gives-me-the-old-value
 */
CREATE TABLE IF NOT EXISTS process_regsitrations (
    id BIGINT UNSIGNED NOT NULL AUTO_INCREMENT,
    process_id BIGINT UNSIGNED NOT NULL,
    recipe_id INT NOT NULL,
    FOREIGN KEY (process_id)
        REFERENCES processes(id),
    FOREIGN KEY (recipe_id)
        REFERENCES recipes(id),
    PRIMARY KEY (id)
);