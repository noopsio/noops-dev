-- Your SQL goes here
CREATE TABLE projects (
  id BINARY(128) PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL, 
  user_id BINARY(128) REFERENCES user(id) NOT NULL,
  UNIQUE(name, user_id)
);