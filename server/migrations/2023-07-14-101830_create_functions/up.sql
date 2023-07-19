-- Your SQL goes here
CREATE TABLE functions (
  id CHAR(21) PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  hash VARCHAR NOT NULL,
  project_id CHAR(21) REFERENCES project(id) NOT NULL,
  UNIQUE(name, project_id)
);