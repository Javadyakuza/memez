DB tables:

Accounts (user should add info manually, otherwise some default values will be used)
wallet_address (primary key / unique constraint)
nickname
profile_picture

Memecoins (data should be fetched from blockchain on MemeCoinDeployed event)
contract_address (primary key / unique constraint)
creator (foreign key to Accounts table)
name
symbol
cap
icon
description
links (twitter, telegram, website?)
(fields: contract_address, name, symbol, cap, creator should be fetched from blockchain, other fields can be filled only by the creator of this memecoin; may be we also need to cache such aggregate fields as market cap, etc.)

Thread_messages
id
memecoin (foreign key to Memecoins table)
timestamp
author (foreign key to Accounts table)
text
image (?)

Trades (data should be fetched from blockchain on Mint and Retire events)
tx_hash (primary key / unique constraint)
memecoin (foreign key to Memecoins table)
timestamp
initiator (wallet address, foreign key to Accounts table if user created account)
response : we must force the user to connect his wallet to see the trade history for two reason:
type (buy/sell - if you want we can remove this field if we use signed data in the fields below)
amount_ETH
amount_token

May be I missed something, but we can start with these tables and then add something else! Right now I'm modifying our smart contracts, so events can be changed!
