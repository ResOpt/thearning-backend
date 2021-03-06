CREATE TABLE classes(
    class_id VARCHAR NOT NULL PRIMARY KEY,
    class_name VARCHAR NOT NULL,
    class_creator VARCHAR,
    class_description VARCHAR,
    class_image VARCHAR,
    section VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,

    FOREIGN KEY(class_creator) REFERENCES users(user_id) ON DELETE SET NULL
)