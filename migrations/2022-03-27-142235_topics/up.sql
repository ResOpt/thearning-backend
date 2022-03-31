CREATE TABLE topics(
    id VARCHAR NOT NULL PRIMARY KEY,
    topic_name VARCHAR NOT NULL,
    classroom_id VARCHAR NOT NULL,

    FOREIGN KEY (classroom_id) REFERENCES classes(class_id) ON DELETE CASCADE
)