CREATE TABLE assignments(
    assignment_id VARCHAR NOT NULL PRIMARY KEY,
    assignment_name VARCHAR NOT NULL,
    class_id VARCHAR NOT NULL,
    due_date DATE,
    due_time TIME,
    posted_date DATE NOT NULL,
    instructions TEXT,
    total_marks INT,

    FOREIGN KEY (class_id) REFERENCES classes(class_id) ON DELETE NO ACTION
)