-- Your SQL goes here
CREATE TABLE handlers (
  id CHAR(21) PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  language VARCHAR NOT NULL,
  hash VARCHAR NOT NULL,
  project_id CHAR(21) REFERENCES project(id) NOT NULL,
  UNIQUE(name, project_id),
  foreign key (project_id) references projects(id)
);