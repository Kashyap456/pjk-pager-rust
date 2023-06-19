-- Add migration script here
CREATE TABLE IF NOT EXISTS users
(
    username TEXT PRIMARY KEY,
    salt TEXT NOT NULL,
    userhash TEXT NOT NULL
);