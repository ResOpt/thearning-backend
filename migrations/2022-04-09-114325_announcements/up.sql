CREATE TABLE announcements(
    announcement_id VARCHAR NOT NULL PRIMARY KEY,
    announcement_name VARCHAR,
    class_id VARCHAR,
    posted_date DATE NOT NULL,
    body VARCHAR,
    draft BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL,

    FOREIGN KEY (class_id) REFERENCES classes(class_id) ON DELETE CASCADE
)