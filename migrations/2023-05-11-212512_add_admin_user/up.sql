INSERT INTO person (email, name, "role", dept, currency) VALUES ('admin@admin.com', 'admin', 'admin', 'admin', 'USD');
INSERT INTO "user" (person_id, username, password, superuser) VALUES ((SELECT person.id from person where person.name = 'admin'), 'admin', 'admin', 1);
