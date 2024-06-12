use crate::models::{Accounts, AccountsResp, Memecoins, Threads, ThreadsResp, Trades};
use crate::{accounts, memecoins, threads, trades};
use crate::{accounts::dsl::*, memecoins::dsl::*, threads::dsl::*, trades::dsl::*};
pub use diesel;
pub use diesel::pg::PgConnection;
pub use diesel::prelude::*;
pub use diesel::result::Error;
pub use dotenvy::dotenv;

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
    let accounts_row: Vec<Memecoins> = memecoins
        .filter(contract_address.eq(&_contract_address))
        .select(Memecoins::as_select())
        .load(_conn)
        .unwrap_or(vec![]);

    if accounts_row.len() == 1 {
        Ok(Memecoins {
            contract_address: accounts_row[0].contract_address.clone(),
            creator_id: accounts_row[0].creator_id.clone(),
            name: accounts_row[0].name.clone(),
            symbol: accounts_row[0].symbol.clone(),
            cap: accounts_row[0].cap.clone(),
            icon: Some(accounts_row[0].icon.clone().unwrap_or_default()),
            description: Some(accounts_row[0].description.clone().unwrap_or_default()),
            links: Some(accounts_row[0].links.clone().unwrap_or_default()),
            market_cap: Some(accounts_row[0].market_cap.unwrap_or_default()),
            created_at: Some(accounts_row[0].created_at.unwrap_or_default()),
        })
    } else {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "token not found !",
        )))
    }
}
