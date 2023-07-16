-- Your SQL goes here
CREATE TABLE functions (
  id BINARY(128) PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  hash VARCHAR NOT NULL,
  project_id BINARY(128) REFERENCES project(id) NOT NULL,
  UNIQUE(name, project_id)
);