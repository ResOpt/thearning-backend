CREATE TABLE private_comments(
     id VARCHAR NOT NULL PRIMARY KEY,
     user_id VARCHAR NOT NULL,
     assignment_id VARCHAR,
     announcement_id VARCHAR,
     body TEXT NOT NULL,
     created_at TIMESTAMP NOT NULL,

     FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
     FOREIGN KEY (assignment_id) REFERENCES assignments(assignment_id) ON DELETE CASCADE,
     FOREIGN KEY (announcement_id) REFERENCES announcements(announcement_id) ON DELETE CASCADE
)