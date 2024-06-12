CREATE TABLE Accounts (
    id SERIAL UNIQUE NOT NULL,
    wallet_address VARCHAR(50) PRIMARY KEY NOT NULL,
    nickname VARCHAR(255),
    profile_picture VARCHAR(255) DEFAULT 'https://i.postimg.cc/yYjk7nKf/def-pfp.jpg'
);
CREATE UNIQUE INDEX idx_accounts_id ON Accounts (id);