use rocket::*;
use serde::Serialize;

#[derive(FromForm, Debug, Serialize)]
pub struct APIAccounts {
    pub wallet_address: String,
    pub nickname: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(FromForm, Debug, Serialize)]
pub struct APIEditAccounts {
    pub id: i32,
    pub wallet_address: String,
    pub nickname: Option<String>,
    pub profile_picture: Option<String>,
}

#[derive(FromForm, Debug, Serialize)]
pub struct APIMemecoins {
    pub contract_address: String,
    pub creator_id: i32,
    pub name: String,
    pub symbol: String,
    pub cap: i32,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub links: Option<String>,
    pub market_cap: Option<i32>,
    pub created_at: Option<String>,
}
#[derive(FromForm, Debug, Serialize)]

pub struct APIThreads {
    pub memecoin: String,
    pub timestamp: Option<String>,
    pub author: i32,
    pub text: String,
    pub image: Option<String>,
}
#[derive(FromForm, Debug, Serialize)]

pub struct APITrades {
    pub tx_hash: String,
    pub memecoin: String,
    pub timestamp: String,
    pub initiator: String,
    pub type_: String,
    pub amount_eth: String,
    pub amount_token: String,
}
