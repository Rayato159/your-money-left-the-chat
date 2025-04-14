-- Your SQL goes here
CREATE TABLE IF NOT EXISTS monthly_spending (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    amount REAL NOT NULL,
    due_date TEXT NOT NULL
);