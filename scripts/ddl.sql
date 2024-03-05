CREATE TABLE clients (
    id SERIAL PRIMARY KEY,
    balance_limit INTEGER,
    balance INTEGER
);

CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    client_id INTEGER REFERENCES clients(id),
    value INTEGER,
    transaction_type VARCHAR(255),
    description TEXT,
    created_at TIMESTAMP DEFAULT NOW()
);
