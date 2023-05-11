-- Your SQL goes here
CREATE TABLE balance (
  id INTEGER PRIMARY KEY NOT NULL,
  person_id INTEGER REFERENCES person(id) NOT NULL,
  value FLOAT NOT NULL
);