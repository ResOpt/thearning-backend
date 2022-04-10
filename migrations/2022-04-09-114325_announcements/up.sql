CREATE TABLE announcements(
    announcement_id VARCHAR NOT NULL PRIMARY KEY,
    announcement_name VARCHAR,
    class_id VARCHAR NOT NULL,
    posted_date DATE NOT NULL,
    body TEXT,
    created_at TIMESTAMP NOT NULL,

    FOREIGN KEY (class_id) REFERENCES classes(class_id) ON DELETE CASCADE
)