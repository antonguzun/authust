CREATE TABLE IF NOT EXISTS entities (
    entity_id int GENERATED BY DEFAULT AS IDENTITY PRIMARY KEY,
    name text
);

INSERT INTO entities (name) VALUES ('Ivan'), ('Anton'), ('Godzilla');

