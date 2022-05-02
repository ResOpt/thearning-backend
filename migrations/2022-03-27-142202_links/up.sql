CREATE TABLE links(
    id VARCHAR NOT NULL PRIMARY KEY,
    title VARCHAR,
    description VARCHAR,
    thumbnail VARCHAR,
    url VARCHAR,
    created_at TIMESTAMP NOT NULL
)