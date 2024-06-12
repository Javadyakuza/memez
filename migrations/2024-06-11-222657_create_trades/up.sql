CREATE TABLE Trades (
    tx_hash VARCHAR(70) UNIQUE PRIMARY KEY,
    memecoin VARCHAR(50) NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    initiator VARCHAR(42) NOT NULL,
    type VARCHAR(4) NOT NULL,
    amount_ETH VARCHAR(46) NOT NULL,
    amount_token VARCHAR(46) NOT NULL,
    FOREIGN KEY (memecoin) REFERENCES Memecoins(contract_address) ON DELETE CASCADE
);