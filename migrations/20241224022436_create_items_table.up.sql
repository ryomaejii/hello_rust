-- Add up migration script here
CREATE TABLE items (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    price INTEGER NOT NULL,
    checked INTEGER NOT NULL DEFAULT 0
);