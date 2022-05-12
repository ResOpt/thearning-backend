CREATE TABLE private_comments(
     id VARCHAR NOT NULL PRIMARY KEY,
     user_id VARCHAR NOT NULL,
     submission_id VARCHAR,
     body TEXT NOT NULL,
     created_at TIMESTAMP NOT NULL,

     FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
     FOREIGN KEY (submission_id) REFERENCES submissions(submission_id) ON DELETE CASCADE
)