CREATE TABLE Memecoins (
    contract_address VARCHAR(42) UNIQUE PRIMARY KEY,
    creator_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    symbol VARCHAR(20) NOT NULL,
    cap DECIMAL(30, 18) NOT NULL,
    icon VARCHAR(255) DEFAULT NULL,
    description TEXT DEFAULT NULL,
    links JSONB DEFAULT NULL,
    market_cap DECIMAL(30, 18) DEFAULT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (creator_id) REFERENCES Accounts(id) ON DELETE CASCADE
);