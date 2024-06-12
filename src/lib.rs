#![recursion_limit = "256"]
pub mod api_models;
pub mod models;
pub mod schema;
use crate::models::{Accounts, Memecoins, Threads, Trades};
use crate::schema::{accounts, memecoins, threads, trades};
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
use models::{AccountsResp, ThreadsResp};
use schema::{accounts::dsl::*, memecoins::dsl::*, threads::dsl::*, trades::dsl::*};

pub use std::env;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn establish_connection() -> PgConnection {
    // loading the env vars into the current scope
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

// setter functions //
pub fn add_account(
    conn: &mut PgConnection,
    account_info: Accounts,
) -> Result<AccountsResp, Box<dyn std::error::Error>> {
    // adding the account
    match diesel::insert_into(accounts::table)
        .values(&account_info)
        .returning(AccountsResp::as_returning())
        .get_result(conn)
    {
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )));
        }
        Ok(res) => return Ok(res),
    }
}

pub fn add_memecoin(
    conn: &mut PgConnection,
    memecoin_info: Memecoins,
) -> Result<Memecoins, Box<dyn std::error::Error>> {
    // adding the memecoin
    match diesel::insert_into(memecoins::table)
        .values(&memecoin_info)
        .returning(Memecoins::as_returning())
        .get_result(conn)
    {
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )));
        }
        Ok(res) => return Ok(res),
    }
}

pub fn add_thread(
    conn: &mut PgConnection,
    thread_info: Threads,
) -> Result<ThreadsResp, Box<dyn std::error::Error>> {
    // adding the thread
    match diesel::insert_into(threads::table)
        .values(&thread_info)
        .returning(ThreadsResp::as_returning())
        .get_result(conn)
    {
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )));
        }
        Ok(res) => return Ok(res),
    }
}

pub fn add_trades(
    conn: &mut PgConnection,
    _trade_info: Trades,
) -> Result<Trades, Box<dyn std::error::Error>> {
    // adding the trade
    match diesel::insert_into(trades::table)
        .values(&_trade_info)
        .returning(Trades::as_returning())
        .get_result(conn)
    {
        Err(e) => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("{:?}", e),
            )));
        }
        Ok(res) => return Ok(res),
    }
}

pub fn get_account_with_wallet_address(
    _conn: &mut PgConnection,
    _wallet_address: &str,
) -> Result<AccountsResp, Box<dyn std::error::Error>> {
    let accounts_row: Vec<AccountsResp> = accounts
        .filter(wallet_address.eq(&_wallet_address))
        .select(AccountsResp::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if accounts_row.len() == 1 {
        Ok(AccountsResp {
            id: accounts_row[0].id,
            wallet_address: _wallet_address.to_string(),
            nickname: accounts_row[0].nickname.clone(),
            profile_picture: accounts_row[0].profile_picture.clone(),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "wallet address not found !",
        )))
    }
}

pub fn get_memecoin_with_address(
    _conn: &mut PgConnection,
    _contract_address: &str,
) -> Result<Memecoins, Box<dyn std::error::Error>> {
    let memecoin_row: Vec<Memecoins> = memecoins
        .filter(contract_address.eq(&_contract_address))
        .select(Memecoins::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if memecoin_row.len() == 1 {
        Ok(Memecoins {
            contract_address: memecoin_row[0].contract_address.clone(),
            creator_id: memecoin_row[0].creator_id.clone(),
            name: memecoin_row[0].name.clone(),
            symbol: memecoin_row[0].symbol.clone(),
            cap: memecoin_row[0].cap.clone(),
            icon: Some(memecoin_row[0].icon.clone().unwrap_or_default()),
            description: Some(memecoin_row[0].description.clone().unwrap_or_default()),
            links: Some(memecoin_row[0].links.clone().unwrap_or_default()),
            market_cap: Some(memecoin_row[0].market_cap.unwrap_or_default()),
            created_at: Some(memecoin_row[0].created_at.unwrap_or_default()),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "token not found !",
        )))
    }
}

pub fn get_memecoin_with_name(
    _conn: &mut PgConnection,
    _name: &str,
) -> Result<Memecoins, Box<dyn std::error::Error>> {
    let memecoin_row: Vec<Memecoins> = memecoins
        .filter(contract_address.eq(&_name))
        .select(Memecoins::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if memecoin_row.len() == 1 {
        Ok(Memecoins {
            contract_address: memecoin_row[0].contract_address.clone(),
            creator_id: memecoin_row[0].creator_id.clone(),
            name: memecoin_row[0].name.clone(),
            symbol: memecoin_row[0].symbol.clone(),
            cap: memecoin_row[0].cap.clone(),
            icon: Some(memecoin_row[0].icon.clone().unwrap_or_default()),
            description: Some(memecoin_row[0].description.clone().unwrap_or_default()),
            links: Some(memecoin_row[0].links.clone().unwrap_or_default()),
            market_cap: Some(memecoin_row[0].market_cap.unwrap_or_default()),
            created_at: Some(memecoin_row[0].created_at.unwrap_or_default()),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "token not found !",
        )))
    }
}

pub fn get_thread_with_id(
    _conn: &mut PgConnection,
    _id: i32,
) -> Result<ThreadsResp, Box<dyn std::error::Error>> {
    let threads_row: Vec<ThreadsResp> = threads
        .filter(threads::id.eq(&_id))
        .select(ThreadsResp::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if threads_row.len() == 1 {
        Ok(ThreadsResp {
            id: threads_row[0].id,
            memecoin: threads_row[0].memecoin.clone(),
            timestamp: threads_row[0].timestamp.clone(),
            author: threads_row[0].author.clone(),
            text: threads_row[0].text.clone(),
            image: threads_row[0].image.clone(),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "token not found !",
        )))
    }
}

pub fn get_trade_with_tx_hash(
    _conn: &mut PgConnection,
    _hash: String,
) -> Result<Trades, Box<dyn std::error::Error>> {
    let trades_row: Vec<Trades> = trades
        .filter(tx_hash.eq(&_hash))
        .select(Trades::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if trades_row.len() == 1 {
        Ok(Trades {
            tx_hash: trades_row[0].tx_hash.clone(),
            memecoin: trades_row[0].memecoin.clone(),
            timestamp: trades_row[0].timestamp.clone(),
            initiator: trades_row[0].initiator.clone(),
            type_: trades_row[0].type_.clone(),
            amount_eth: trades_row[0].amount_eth.clone(),
            amount_token: trades_row[0].amount_token.clone(),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "token not found !",
        )))
    }
}

// get all memecoin threads
pub fn get_memecoin_threads(
    _conn: &mut PgConnection,
    _memecoin_address: String,
) -> Result<Vec<ThreadsResp>, Box<dyn std::error::Error>> {
    Ok(threads
        .filter(threads::memecoin.eq(&_memecoin_address))
        .select(ThreadsResp::as_select())
        .load(_conn)
        .unwrap_or(vec![]))
}

// get all memecoin trades
pub fn get_memecoin_trades(
    _conn: &mut PgConnection,
    _hash: String,
) -> Result<Trades, Box<dyn std::error::Error>> {
    let trades_row: Vec<Trades> = trades
        .filter(tx_hash.eq(&_hash))
        .select(Trades::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if trades_row.len() == 1 {
        Ok(Trades {
            tx_hash: trades_row[0].tx_hash.clone(),
            memecoin: trades_row[0].memecoin.clone(),
            timestamp: trades_row[0].timestamp.clone(),
            initiator: trades_row[0].initiator.clone(),
            type_: trades_row[0].type_.clone(),
            amount_eth: trades_row[0].amount_eth.clone(),
            amount_token: trades_row[0].amount_token.clone(),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "token not found !",
        )))
    }
}
