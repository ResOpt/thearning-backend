CREATE TABLE submissions(
    submission_id VARCHAR NOT NULL PRIMARY KEY,
    assignment_id VARCHAR NOT NULL,
    user_id VARCHAR NOT NULL,
    submitted_date DATE NOT NULL,
    submitted_time TIME NOT NULL,
    on_time BOOLEAN NOT NULL,
    marks_allotted INT,
    created_at TIMESTAMP NOT NULL,

    FOREIGN KEY (assignment_id) REFERENCES assignments(assignment_id) ON DELETE CASCADE ,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
)