-- Add migration script here
CREATE TABLE IF NOT EXISTS keyval
(
    ukey TEXT PRIMARY KEY,
    hashpass TEXT
);