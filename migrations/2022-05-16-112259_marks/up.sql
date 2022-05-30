CREATE TABLE marks (
    id VARCHAR PRIMARY KEY NOT NULL,
    submission_id VARCHAR,
    marker_id VARCHAR,
    student_id VARCHAR,
    value INT NOT NULL,

    FOREIGN KEY (submission_id) REFERENCES submissions(submission_id),
    FOREIGN KEY (marker_id) REFERENCES users(user_id),
    FOREIGN KEY (student_id) REFERENCES users(user_id)
)