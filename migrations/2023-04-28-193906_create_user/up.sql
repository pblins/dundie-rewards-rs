-- Your SQL goes here
CREATE TABLE user (
    id INTEGER PRIMARY KEY NOT NULL,
    password VARCHAR NOT NULL,
    person_id INTEGER REFERENCES person(id) NOT NULL
);