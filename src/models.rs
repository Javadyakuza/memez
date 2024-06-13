use crate::api_models::{APIAccounts, APIEditAccounts, APIMemecoins, APIThreads, APITrades};
use crate::schema::{accounts, memecoins, threads, trades};
use crate::{accounts::dsl::*, memecoins::dsl::*, threads::dsl::*, trades::dsl::*};
use chrono::NaiveDateTime;
use diesel::{
    prelude::*,
    sql_types::{Bytea, Numeric},
};
use rocket::request::FromForm;
use serde::{self, Deserialize, Serialize};
use std::fmt::Debug;

pub trait ToDBModel<
    T: for<'a> FromForm<'a> + Debug + Serialize,
    D: Serialize + for<'a> Deserialize<'a> + Debug,
>
{
    fn to_db_model(model: T) -> D;
}

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

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
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

// fetch table response //
#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::threads)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ThreadsResp {
    pub id: i32,
    pub memecoin: String,
    pub timestamp: Option<chrono::NaiveDateTime>,
    pub author: i32,
    pub text: String,
    pub image: Option<String>,
}

#[derive(Queryable, Deserialize, Serialize, Selectable, Debug, Insertable, PartialEq)]
#[diesel(table_name = crate::schema::accounts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AccountsResp {
    pub id: i32,
    pub wallet_address: String,
    pub nickname: Option<String>,
    pub profile_picture: Option<String>,
}

impl ToDBModel<APIAccounts, Accounts> for APIAccounts {
    fn to_db_model(model: APIAccounts) -> Accounts {
        Accounts {
            wallet_address: model.wallet_address,
            nickname: model.nickname,
            profile_picture: model.profile_picture,
        }
    }
}
impl ToDBModel<APIMemecoins, Memecoins> for APIMemecoins {
    fn to_db_model(model: APIMemecoins) -> Memecoins {
        let parsed_links: serde_json::Value = serde_json::from_str(&model.links.unwrap()).unwrap();
        let parsed_crated_at =
            NaiveDateTime::parse_from_str(&model.created_at.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap();

        Memecoins {
            contract_address: model.contract_address,
            creator_id: model.creator_id,
            name: model.name,
            symbol: model.symbol,
            cap: model.cap,
            icon: model.icon,
            description: model.description,
            links: Some(parsed_links),
            market_cap: model.market_cap,
            created_at: Some(parsed_crated_at),
        }
    }
}

impl ToDBModel<APIThreads, Threads> for APIThreads {
    fn to_db_model(model: APIThreads) -> Threads {
        let parsed_timestamp =
            NaiveDateTime::parse_from_str(&model.timestamp.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap();

        Threads {
            memecoin: model.memecoin,
            timestamp: Some(parsed_timestamp),
            author: model.author,
            text: model.text,
            image: model.image,
        }
    }
}

impl ToDBModel<APITrades, Trades> for APITrades {
    fn to_db_model(model: APITrades) -> Trades {
        let parsed_timestamp =
            NaiveDateTime::parse_from_str(&model.timestamp, "%Y-%m-%d %H:%M:%S").unwrap();

        Trades {
            tx_hash: model.tx_hash,
            memecoin: model.memecoin,
            timestamp: parsed_timestamp,
            initiator: model.initiator,
            type_: model.type_,
            amount_eth: model.amount_eth,
            amount_token: model.amount_token,
        }
    }
}

impl ToDBModel<APIEditAccounts, AccountsResp> for APIEditAccounts {
    fn to_db_model(model: APIEditAccounts) -> AccountsResp {
        AccountsResp {
            id: model.id,
            wallet_address: model.wallet_address,
            nickname: model.nickname,
            profile_picture: model.profile_picture,
        }
    }
}
