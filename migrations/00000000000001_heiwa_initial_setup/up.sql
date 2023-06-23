CREATE TABLE authors (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    display_name VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    biography TEXT,
    role VARCHAR DEFAULT 'author'
);

CREATE TABLE links (
    id SERIAL PRIMARY KEY,
    url VARCHAR NOT NULL,
    title VARCHAR NOT NULL,
    author_id SERIAL REFERENCES authors(id)
);

INSERT INTO authors (name, email, display_name, password, role) VALUES (
    'admin',
    'admin@getheiwa.fr',
    'Admin',
    'heiwa',
    'administrator'
);