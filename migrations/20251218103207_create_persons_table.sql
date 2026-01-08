-- Add migration script here
CREATE TABLE IF NOT EXISTS persons (
    name TEXT PRIMARY KEY,
    age INTEGER NOT NULL
);