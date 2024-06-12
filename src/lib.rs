#![recursion_limit = "256"]
pub mod helpers;
pub mod models;
pub mod schema;

use crate::models::{Accounts, Memecoins, Threads, Trades};
use crate::schema::{accounts, memecoins, threads, trades};
use chrono::Local;
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;
use helpers::{get_account_with_wallet_address, get_memecoin_with_address};
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
    account_info: Accounts,
) -> Result<AccountsResp, Box<dyn std::error::Error>> {
    // adding the trade
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
