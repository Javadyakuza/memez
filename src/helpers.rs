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
