#![feature(decl_macro)] // helps us with the routing of our application
extern crate rocket;

use memez::{
    add_account, add_memecoin, add_thread, add_trades,
    api_models::{APIAccounts, APIEditAccounts, APIMemecoins, APIThreads, APITrades},
    edit_account, establish_connection, get_account_with_wallet_address, get_memecoin_threads,
    get_memecoin_with_address, get_memecoin_with_name, get_thread_with_id, get_trade_with_tx_hash,
    models::{AccountsResp, Memecoins, ThreadsResp, ToDBModel, Trades},
};

use rocket::request::{Form, Request};
use rocket::*;
use rocket_contrib::json::Json;
#[get("/acc-via-wallet/<wallet_address>")]
fn acc_via_wallet(wallet_address: String) -> Json<Result<AccountsResp, String>> {
    let mut conn = establish_connection();
    match get_account_with_wallet_address(&mut conn, wallet_address.as_str()) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[get("/token-via-addr/<token_address>")]
fn token_with_addr(token_address: String) -> Json<Result<Memecoins, String>> {
    let mut conn = establish_connection();
    match get_memecoin_with_address(&mut conn, &token_address) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[get("/token-via-name/<token_name>")]
fn token_with_name(token_name: String) -> Json<Result<Memecoins, String>> {
    let mut conn = establish_connection();
    match get_memecoin_with_name(&mut conn, &token_name) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[get("/thread-via-id/<thread_id>")]
fn thread_via_id(thread_id: i32) -> Json<Result<ThreadsResp, String>> {
    let mut conn = establish_connection();
    match get_thread_with_id(&mut conn, thread_id) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[get("/tokens-threads/<token_address>")]
fn tokens_threads(token_address: String) -> Json<Result<Vec<ThreadsResp>, String>> {
    let mut conn = establish_connection();
    match get_memecoin_threads(&mut conn, token_address) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[get("/trade-via-tx-hash/<tx_hash>")]
fn trade_via_tx_hash(tx_hash: String) -> Json<Result<Trades, String>> {
    let mut conn = establish_connection();
    match get_trade_with_tx_hash(&mut conn, tx_hash) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[get("/tokens-trades/<token_address>")]
fn tokens_trades(token_address: String) -> Json<Result<Vec<ThreadsResp>, String>> {
    let mut conn = establish_connection();
    match get_memecoin_threads(&mut conn, token_address) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/add-account", data = "<new_acc_info>")]
fn add_acc(new_acc_info: Form<APIAccounts>) -> Json<Result<AccountsResp, String>> {
    let mut conn = establish_connection();
    match add_account(&mut conn, APIAccounts::to_db_model(new_acc_info.0)) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/edit-account", data = "<new_acc_info>")]
fn edit_acc(new_acc_info: Form<APIEditAccounts>) -> Json<Result<AccountsResp, String>> {
    let mut conn = establish_connection();
    match edit_account(&mut conn, APIEditAccounts::to_db_model(new_acc_info.0)) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/add-memecoin", data = "<new_memecoin_info>")]
fn add_memcoin(new_memecoin_info: Form<APIMemecoins>) -> Json<Result<Memecoins, String>> {
    let mut conn = establish_connection();
    match add_memecoin(&mut conn, APIMemecoins::to_db_model(new_memecoin_info.0)) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/add-thread", data = "<new_thread_info>")]
fn create_thread(new_thread_info: Form<APIThreads>) -> Json<Result<ThreadsResp, String>> {
    let mut conn = establish_connection();
    match add_thread(&mut conn, APIThreads::to_db_model(new_thread_info.0)) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}
#[post("/add-trade", data = "<new_trade_info>")]
fn store_trade(new_trade_info: Form<APITrades>) -> Json<Result<Trades, String>> {
    let mut conn = establish_connection();
    match add_trades(&mut conn, APITrades::to_db_model(new_trade_info.0)) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Oh no the {} path doesn't exists !!", req.uri())
}
fn main() {
    rocket::ignite()
        .register(catchers![not_found])
        .mount(
            "/api",
            routes![
                acc_via_wallet,
                token_with_addr,
                token_with_name,
                thread_via_id,
                tokens_threads,
                trade_via_tx_hash,
                tokens_trades,
                add_acc,
                edit_acc,
                add_memcoin,
                create_thread,
                store_trade
            ],
        )
        .launch();
    // needs the "cargo build and then cargo run to be ran oin the fucking serve"
}
