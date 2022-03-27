CREATE TABLE users(
                      user_id VARCHAR UNIQUE NOT NULL PRIMARY KEY,
                      fullname VARCHAR NOT NULL,
                      profile_photo VARCHAR NOT NULL,
                      email VARCHAR UNIQUE NOT NULL,
                      password VARCHAR NOT NULL,
                      bio TEXT NOT NULL,
                      status VARCHAR NOT NULL
)