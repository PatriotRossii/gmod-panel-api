-- Your SQL goes here
CREATE TABLE servers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id INTEGER,
    name TEXT,
    ip TEXT,
    password TEXT
);