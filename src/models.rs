use diesel::{
    prelude::*,
    sql_types::{Bytea, Numeric},
};
use pg_bigdecimal::PgNumeric;
// use merge_derivable;
use crate::schema::{accounts, memecoins, threads, trades};
use serde::{self, Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Accounts {
    // pub id: i32, // autofill serial type
    pub wallet_address: String,
    pub nickname: Option<String>,
    pub profile_picture: Option<String>,
}
#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::memecoins)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Memecoins {
    // pub id: i32, // autofill serial type
    pub contract_address: String,
    pub creator_id: i32,
    pub name: String,
    pub symbol: String,
    pub cap: i32,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub links: Option<serde_json::Value>,
    pub market_cap: Option<i32>,
    pub created_at: Option<chrono::NaiveDateTime>,
}
#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::threads)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Threads {
    // pub id: i32, // autofill serial type
    pub memecoin: String,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub author: i32,
    pub text: String,
    pub image: Option<String>,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, PartialEq)]
#[diesel(table_name = crate::schema::trades)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Trades {
    pub tx_hash: String,
    pub memecoin: String,
    pub timestamp: chrono::NaiveDateTime,
    pub initiator: String,
    pub type_: String,
    pub amount_eth: String,
    pub amount_token: String,
}
