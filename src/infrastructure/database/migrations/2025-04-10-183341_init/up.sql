-- Your SQL goes here
CREATE TABLE IF NOT EXISTS bitcoin_buy_ledger (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    amount REAL NOT NULL,
    price REAL NOT NULL,
    cost REAL NOT NULL,
    date TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS bitcoin_sell_ledger (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    amount REAL NOT NULL,
    price REAL NOT NULL,
    cost REAL NOT NULL,
    date TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS debt_ledger (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    amount REAL NOT NULL,
    category TEXT NOT NULL,
    description TEXT,
    who TEXT NOT NULL,
    date TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS my_ledger (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    amount REAL NOT NULL,
    category TEXT NOT NULL,
    description TEXT,
    date TEXT NOT NULL
);