CREATE TABLE authors (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL UNIQUE,
    email VARCHAR NOT NULL UNIQUE,
    display_name VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    biography TEXT,
    role VARCHAR DEFAULT 'AUTHOR'
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
    '$argon2id$v=19$m=19456,t=2,p=1$p4XJkjVIrvkUkDHsw3u3GA$eh3zH5sxy1qZNUeEaAdZZU0dmr4gnaA8HP92w3A35no',
    'ADMIN'
);

CREATE TABLE articles (
    id SERIAL PRIMARY KEY,
    permalink VARCHAR NOT NULL UNIQUE,
    title VARCHAR NOT NULL,
    creation_date TIMESTAMP NOT NULL DEFAULT CURRENT_DATE,
    publication_date TIMESTAMP,
    update_date TIMESTAMP,
    content TEXT,
    published BOOLEAN NOT NULL DEFAULT FALSE,
    meta_description VARCHAR,
    author_id INT NOT NULL
);

CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    permalink VARCHAR NOT NULL UNIQUE,
    label VARCHAR NOT NULL
);

CREATE TABLE articles_tags (
    article_id SERIAL REFERENCES articles(id),
    tag_id SERIAL REFERENCES tags(id),
    PRIMARY KEY(article_id, tag_id)
);