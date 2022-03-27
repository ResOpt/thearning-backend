CREATE TABLE attachments(
    attachment_id VARCHAR NOT NULL PRIMARY KEY,
    file_id VARCHAR NOT NULL,
    assignment_id VARCHAR NOT NULL,
    uploader VARCHAR NOT NULL,

    FOREIGN KEY(file_id) REFERENCES files(file_id) ON DELETE NO ACTION,
    FOREIGN KEY(assignment_id) REFERENCES assignments(assignment_id) ON DELETE NO ACTION,
    FOREIGN KEY(uploader) REFERENCES users(user_id) ON DELETE NO ACTION
)