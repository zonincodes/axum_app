CREATE TABLE users (
   id SERIAL PRIMARY KEY,
   name VARCHAR(200) NOT NULL,
   email VARCHAR(200) NOT NULL
);

INSERT INTO users (id, name, email)
VALUES (1, 'Alice Smith', 'alice.smith@example.com'),
    (3, 'Charlie Lee', 'charlie.lee@example.com'),
    (4, 'Dana White', 'dana.white@example.com'),
    (5, 'Evan Brown', 'evan.brown@example.com');