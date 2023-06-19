-- Add migration script here
CREATE TABLE IF NOT EXISTS groups
(
    group_name TEXT PRIMARY KEY,
    group_owner TEXT NOT NULL,
    FOREIGN KEY(group_owner)
        REFERENCES users (username)
);

CREATE TABLE IF NOT EXISTS memberships
(
    user TEXT NOT NULL,
    group_name TEXT NOT NULL,
    is_admin INTEGER,
    FOREIGN KEY(user)
        REFERENCES users (username),
    FOREIGN KEY(group_name)
        REFERENCES groups (group_name)
);