-- Your SQL goes here
CREATE TABLE users (
  id BINARY(128) NOT NULL PRIMARY KEY,
  email VARCHAR NOT NULL,
  github_id INTEGER NOT NULL,
  github_access_token VARCHAR NOT NULL
)