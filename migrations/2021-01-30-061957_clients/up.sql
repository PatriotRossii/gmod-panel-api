-- Your SQL goes here
CREATE TABLE clients (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    nickname TEXT NOT NULL,
    steam_id TEXT,
    vkid TEXT
);