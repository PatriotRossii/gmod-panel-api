-- Your SQL goes here
CREATE TABLE servers (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    client_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    ip TEXT NOT NULL,
    password TEXT NOT NULL
);