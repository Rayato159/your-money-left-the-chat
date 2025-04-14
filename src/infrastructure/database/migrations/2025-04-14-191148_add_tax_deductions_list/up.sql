-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tax_deductions_list (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    amount REAL NOT NULL
);