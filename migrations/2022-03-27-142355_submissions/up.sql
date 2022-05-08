CREATE TABLE submissions(
    submission_id VARCHAR NOT NULL PRIMARY KEY,
    assignment_id VARCHAR NOT NULL,
    user_id VARCHAR NOT NULL,
    submitted_date DATE,
    submitted_time TIME,
    on_time BOOLEAN,
    marks_allotted INT,
    submitted BOOLEAN NOT NULL,
    created_at TIMESTAMP NOT NULL,

    FOREIGN KEY (assignment_id) REFERENCES assignments(assignment_id) ON DELETE CASCADE ,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE
)