-- Your SQL goes here
CREATE TABLE users (
  id CHAR(21) NOT NULL PRIMARY KEY,
  email VARCHAR NOT NULL,
  github_id INTEGER NOT NULL,
  github_access_token VARCHAR NOT NULL
)