CREATE TABLE prediction_events (
    id INTEGER PRIMARY KEY,
    by_user INTEGER NOT NULL,
    for_outcome INTEGER NOT NULL,
    prediction BOOLEAN NOT NULL,
    creation_date DATETIME NOT NULL,
    FOREIGN KEY(by_user) REFERENCES user(id),
    FOREIGN KEY(for_outcome) REFERENCES outcome(id)
);
CREATE TABLE outcomes (
    id INTEGER PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    creation_date DATETIME NOT NULL,
    resolution_date DATETIME NOT NULL
);
CREATE TABLE users (
    id INTEGER PRIMARY KEY,
    username TEXT NOT NULL,
    display_name TEXT NOT NULL
);
CREATE TABLE transactions (
    id INTEGER PRIMARY KEY,
    date DATETIME NOT NULL,
    amount INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    CONSTRAINT fk_user_id FOREIGN KEY(user_id) REFERENCES user(id) ON DELETE CASCADE
);