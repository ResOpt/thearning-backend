CREATE TABLE admins(
     id SERIAL PRIMARY KEY NOT NULL,
     user_id VARCHAR NOT NULL,
     class_id VARCHAR NOT NULL,

     FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE NO ACTION,
     FOREIGN KEY (class_id) REFERENCES classes(class_id) ON DELETE NO ACTION
)