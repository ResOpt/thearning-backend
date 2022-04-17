CREATE TABLE assignments(
    assignment_id VARCHAR NOT NULL PRIMARY KEY,
    assignment_name VARCHAR,
    class_id VARCHAR,
    topic_id VARCHAR,
    due_date DATE,
    due_time TIME,
    posted_date DATE NOT NULL,
    instructions TEXT,
    total_marks INT,
    created_at TIMESTAMP NOT NULL,
    draft BOOLEAN NOT NULL,

    FOREIGN KEY (class_id) REFERENCES classes(class_id) ON DELETE CASCADE ,
    FOREIGN KEY (topic_id) REFERENCES topics(id) ON DELETE CASCADE
)