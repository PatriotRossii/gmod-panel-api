-- Your SQL goes here
CREATE TABLE connected_modules (
    server_id INTEGER PRIMARY KEY NOT NULL,
    module_id INTEGER NOT NULL,
    status BOOLEAN NOT NULL
);