CREATE TABLE attachments(
    attachment_id VARCHAR NOT NULL PRIMARY KEY,
    file_id VARCHAR,
    link_id VARCHAR,
    assignment_id VARCHAR,
    announcement_id VARCHAR,
    uploader VARCHAR NOT NULL,
    created_at TIMESTAMP NOT NULL,

    FOREIGN KEY(file_id) REFERENCES files(file_id) ON DELETE CASCADE ,
    FOREIGN KEY(assignment_id) REFERENCES assignments(assignment_id) ON DELETE CASCADE,
    FOREIGN KEY(announcement_id) REFERENCES announcements(announcement_id) ON DELETE CASCADE,
    FOREIGN KEY(uploader) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY(link_id) REFERENCES links(id) ON DELETE CASCADE
)