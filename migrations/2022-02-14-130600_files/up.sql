-- Your SQL goes here

CREATE TABLE files(
    file_id VARCHAR NOT NULL PRIMARY KEY,
    filename VARCHAR NOT NULL,
    file_path VARCHAR NOT NULL,
    file_urls VARCHAR NOT NULL,
    filetype VARCHAR NOT NULL
)