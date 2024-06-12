use crate::api_models::{APIAccounts, APIMemecoins, APIThreads, APITrades};
use crate::schema::{accounts, memecoins, threads, trades};
use crate::{accounts::dsl::*, memecoins::dsl::*, threads::dsl::*, trades::dsl::*};
use chrono::NaiveDateTime;
use diesel::{
    prelude::*,
    sql_types::{Bytea, Numeric},
};
use rocket::request::FromForm;
use serde::{self, Deserialize, Serialize};
use std::fmt::Debug; // use merge_derivable;

pub trait FromApiModel<T: for<'a> FromForm<'a> + Debug + Serialize> {
    fn from_api_model(model: T) -> Self;
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

impl FromApiModel<APIMemecoins> for Memecoins {
    fn from_api_model(model: APIMemecoins) -> Self {
        let parsed_links: serde_json::Value = serde_json::from_str(&model.links.unwrap()).unwrap();
        let parsed_crated_at =
            NaiveDateTime::parse_from_str(&model.created_at.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap();

        Self {
            contract_address: model.contract_address.clone(),
            creator_id: model.creator_id.clone(),
            name: model.name.clone(),
            symbol: model.symbol.clone(),
            cap: model.cap.clone(),
            icon: model.icon.clone(),
            description: model.description.clone(),
            links: Some(parsed_links),
            market_cap: model.market_cap.clone(),
            created_at: Some(parsed_crated_at),
        }
    }
}

impl FromApiModel<APIThreads> for Threads {
    fn from_api_model(model: APIThreads) -> Self {
        let parsed_timestamp =
            NaiveDateTime::parse_from_str(&model.timestamp.unwrap(), "%Y-%m-%d %H:%M:%S").unwrap();

        Self {
            memecoin: model.memecoin.clone(),
            timestamp: Some(parsed_timestamp),
            author: model.author.clone(),
            text: model.text.clone(),
            image: model.image.clone(),
        }
    }
}

impl FromApiModel<APITrades> for Trades {
    fn from_api_model(model: APITrades) -> Self {
        let parsed_timestamp =
            NaiveDateTime::parse_from_str(&model.timestamp, "%Y-%m-%d %H:%M:%S").unwrap();

        Self {
            tx_hash: model.tx_hash.clone(),
            memecoin: model.memecoin.clone(),
            timestamp: parsed_timestamp,
            initiator: model.initiator.clone(),
            type_: model.type_.clone(),
            amount_eth: model.amount_eth.clone(),
            amount_token: model.amount_token.clone(),
        }
    }
}
