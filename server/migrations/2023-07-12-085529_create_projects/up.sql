-- Your SQL goes here
CREATE TABLE projects (
  id CHAR(21) PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL, 
  user_id CHAR(21) REFERENCES user(id) NOT NULL,
  UNIQUE(name, user_id)
);