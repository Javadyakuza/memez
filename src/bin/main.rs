#![feature(decl_macro)] // helps us with the routing of our application
#[macro_use]
extern crate rocket;

use memez::establish_connection;
use memez::get_account_with_wallet_address;
use memez::get_memecoin_threads;
use memez::get_memecoin_with_address;
use memez::get_memecoin_with_name;
use memez::get_thread_with_id;
use memez::get_trade_with_tx_hash;
use memez::models::AccountsResp;
use memez::models::Memecoins;
use memez::models::ThreadsResp;
use memez::models::Trades;
use rocket::request::Form;
use rocket::request::Request;
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

#[post("/create-user", data = "<new_user>")]
fn new_user(new_user: Form<NewUserIN>) -> Json<Result<QUsersResponse, String>> {
    let mut conn = establish_connection();
    match add_new_user(
        &mut conn,
        &Users {
            username: new_user.username_in.clone(),
            email: new_user.email_in.clone(),
            password: new_user.password_in.clone(),
            phone_number: new_user.phone_number_in.clone(),
        },
        &mut UserProfiles {
            user_id: 0,
            bio: new_user.bio_in.clone(),
            profile_picture: new_user.profile_picture_in.clone(),
        },
    ) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/update-user-credits", data = "<new_credits>")]
fn update_user_conditionals(
    new_credits: Form<UpdatedUserCreditsIN>,
) -> Json<Result<QUsers, String>> {
    let mut conn = establish_connection();
    match update_user_credits(
        &mut conn,
        &new_credits.username_out.clone(),
        &Users {
            username: new_credits.username_in.clone(),
            email: new_credits.email_in.clone(),
            password: new_credits.password_in.clone(),
            phone_number: new_credits.phone_number_in.clone(),
        },
    ) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/update-user-profile", data = "<new_profile>")]
fn update_user_profile_api(
    new_profile: Form<UpdatedUserProfileIN>,
) -> Json<Result<UserProfiles, String>> {
    let mut conn = establish_connection();
    match update_user_profile(
        &mut conn,
        &new_profile.username_in.clone(),
        &mut UserProfiles {
            user_id: 0,
            bio: Some(new_profile.bio_in.clone()),
            profile_picture: Some(new_profile.profile_picture_in.clone()),
        },
    ) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/delete-user", data = "<username>")]
fn delete_user_via_username(username: Form<SinglePostUsername>) -> Json<Result<bool, String>> {
    let mut conn = establish_connection();
    match delete_user(&mut conn, &username.username_in.clone()) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/create-p2p", data = "<new_p2p_info>")]
fn new_p2p(new_p2p_info: Form<NewP2PChatRoomIN>) -> Json<Result<QChatRooms, String>> {
    let mut conn = establish_connection();
    let req_user: QUsers;
    match get_user_with_username(
        &mut conn,
        new_p2p_info.requestor_username_in.clone().as_str(),
    ) {
        Ok(res) => req_user = res,
        Err(e) => return Json(Err(format!("{:?}", e))),
    }

    let acc_user: QUsers;
    match get_user_with_username(
        &mut conn,
        new_p2p_info.acceptor_username_in.clone().as_str(),
    ) {
        Ok(res) => acc_user = res,
        Err(e) => return Json(Err(format!("{:?}", e))),
    }

    match add_new_p2p_chat_room(
        &mut conn,
        req_user.user_id,
        acc_user.user_id,
        new_p2p_info.chat_room_pubkey_in.clone(),
    ) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/delete-p2p", data = "<new_gp_info>")]
fn delete_p2p(new_gp_info: Form<DeleteP2PChatRoomIN>) -> Json<Result<bool, String>> {
    let mut conn = establish_connection();
    match delete_p2p_chat_room(
        &mut conn,
        &new_gp_info.remover_username_in.clone(),
        &new_gp_info.contact_username_in.clone(),
    ) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/create-gp", data = "<new_gp_info>")]
fn new_gp(new_gp_info: Json<NewGroupChatRoomIN>) -> Json<Result<QChatRooms, String>> {
    let mut conn = establish_connection();
    match add_new_group_chat_room(
        &mut conn,
        &ChatRooms {
            room_name: new_gp_info.room_name_in.clone(),
            room_description: new_gp_info.room_description_in.clone(),
            chat_room_pubkey: new_gp_info.chat_room_pubkey.as_bytes().to_vec(),
        },
        &(new_gp_info.group_owner_username_in.clone()),
        new_gp_info.group_members_in.to_owned(),
    ) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/update-gp-info", data = "<new_gp_info>")]
fn update_gp(new_gp_info: Form<UpdatedGroupChatRoomInfoIN>) -> Json<Result<QChatRooms, String>> {
    let mut conn = establish_connection();
    match update_group_chat_room_info(
        &mut conn,
        &new_gp_info.old_chat_room_name_in.clone(),
        &UpdatableChatRooms {
            room_name: new_gp_info.room_name_in.clone(),
            room_description: new_gp_info.room_description_in.clone(),
        },
        &new_gp_info.editor_username_in.clone(),
    ) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/delete-gp", data = "<new_gp_info>")]
fn delete_gp(new_gp_info: Form<DeleteGroupChatRoomIN>) -> Json<Result<bool, String>> {
    let mut conn = establish_connection();
    match delete_group_chat_room(
        &mut conn,
        &new_gp_info.chat_room_name_in.clone(),
        &new_gp_info.remover_username_in.clone(),
    ) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}
#[post("/add-user-to-gp", data = "<new_participant>")]
fn add_user_to_gp(
    new_participant: Form<NewGroupChatParticipantIN>,
) -> Json<Result<ChatRoomParticipants, String>> {
    let mut conn = establish_connection();
    let _chat_room_id;

    match get_group_chat_by_name(&mut conn, &new_participant.chat_room_name_in.clone()) {
        Ok(res) => _chat_room_id = res.chat_room_id,
        Err(e) => return Json(Err(format!("{}", e))),
    }

    let _user_id: i32;
    match get_user_with_username(&mut conn, new_participant.username_in.clone().as_str()) {
        Ok(res) => _user_id = res.user_id,
        Err(e) => return Json(Err(format!("{:?}", e))),
    }

    match add_participant_to_group_chat_room(
        &mut conn,
        &ChatRoomParticipants {
            chat_room_id: _chat_room_id,
            user_id: _user_id,
            is_admin: false,
        },
        &new_participant.adder_username_in,
    ) {
        Ok(res) => return Json(Ok(res)),
        Err(e) => return Json(Err(format!("{:?}", e))),
    }
}

#[post("/delete-user-from-gp", data = "<removing_participant>")]
fn delete_user_from_gp(
    removing_participant: Form<GroupChatParticipantToRemoveIN>,
) -> Json<Result<bool, String>> {
    let mut conn = establish_connection();

    let _chat_room_id;
    match get_group_chat_by_name(&mut conn, &removing_participant.chat_room_name_in.clone()) {
        Ok(res) => _chat_room_id = res.chat_room_id,
        Err(e) => return Json(Err(format!("{}", e))),
    }

    let _removing_user_id;
    match get_user_with_username(&mut conn, removing_participant.username_in.clone().as_str()) {
        Ok(res) => _removing_user_id = res.user_id,
        Err(e) => return Json(Err(format!("{}", e))),
    }

    let _remover_user_id;
    match get_user_with_username(
        &mut conn,
        removing_participant.remover_username_in.clone().as_str(),
    ) {
        Ok(res) => _remover_user_id = res.user_id,
        Err(e) => return Json(Err(format!("{}", e))),
    }

    let _admin;
    match get_group_owner_by_id(&mut conn, _chat_room_id) {
        Ok(res) => _admin = res,
        Err(e) => return Json(Err(format!("{}", e))),
    }

    match del_participant_from_group_chat_room(
        &mut conn,
        &ChatRoomParticipants {
            chat_room_id: _chat_room_id,
            user_id: _removing_user_id,
            is_admin: _remover_user_id == _admin,
        },
        _remover_user_id,
    ) {
        Ok(res) => Json(Ok(res)),
        Err(e) => return Json(Err(format!("{}", e))),
    }
}

#[post("/add-solana-wallet", data = "<new_wallet_info>")]
fn add_solana_wallet(new_wallet_info: Form<NewWalletIn>) -> Json<Result<QSolanaWallet, String>> {
    let mut conn = establish_connection();

    let _user_id;
    match get_user_with_username(&mut conn, &new_wallet_info.username_in) {
        Ok(res) => _user_id = res.user_id,
        Err(e) => return Json(Err(format!("{}", e))),
    }
    match initialize_new_solana_wallet(
        &mut conn,
        &SolanaWallet {
            user_id: _user_id,
            wallet_addr: new_wallet_info.wallet_addr_in.as_bytes().to_vec(),
            wallet_backup: new_wallet_info.wallet_backup_in.as_bytes().to_vec(),
        },
    ) {
        Ok(res) => Json(Ok(res)),
        Err(e) => return Json(Err(format!("{}", e))),
    }
}

#[post("/create-token-account", data = "<new_wallet_info>")]
fn create_token_account_api(
    new_wallet_info: Form<CreateTokenAccount>,
) -> Json<Result<CreateTokenAccountResponse, String>> {
    match create_token_account(&CreateTokenAccount {
        wallet_address: new_wallet_info.wallet_address.clone(),
        token_mint_address: new_wallet_info.token_mint_address.clone(),
        token_program_id: new_wallet_info.token_program_id.clone(),
        lbh: new_wallet_info.lbh.clone(),
    }) {
        Ok(res) => Json(Ok(res)),
        Err(e) => return Json(Err(format!("{}", e))),
    }
}
#[post("/fund-wallet", data = "<wallet_address>")]
fn fund_wallet(wallet_address: Form<FundWalletIn>) -> Json<Result<String, String>> {
    match activate_wallet_account_for_transfer(wallet_address.wallet_address.clone()) {
        Ok(res) => Json(Ok(res)),
        Err(e) => return Json(Err(format!("{}", e))),
    }
}

#[get("/get-solana-addr-by-username/<username>")]
fn get_solana_addr(username: String) -> Json<Result<String, String>> {
    let mut conn = establish_connection();
    let _user_id;
    match get_user_with_username(&mut conn, &username) {
        Ok(res) => _user_id = res.user_id,
        Err(e) => return Json(Err(format!("{}", e))),
    }

    match get_user_solana_wallet(&mut conn, _user_id) {
        Ok(res) => Json(Ok(String::from_utf8_lossy(res.wallet_addr.as_slice())
            .as_ref()
            .to_string())),
        Err(e) => return Json(Err(format!("{}", e))),
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
                get_user_via_email,
                get_user_via_username,
                get_user_via_user_id,
                get_user_profile_via_username,
                get_chatroom_by_id,
                get_chatroom_by_name,
                get_group_owner_via_id,
                get_chatroom_id_via_name,
                validate_gp,
                validate_cr,
                validate_user,
                validate_chatroom_user,
                get_all_user_groups,
                get_all_user_p2p,
                // setters
                new_user,
                update_user_conditionals,
                update_user_profile_api,
                delete_user_via_username,
                new_p2p,
                delete_p2p,
                new_gp,
                update_gp,
                delete_gp,
                add_user_to_gp,
                delete_user_from_gp,
                get_solana_addr,
                add_solana_wallet,
                create_token_account_api,
                fund_wallet
            ],
        )
        .launch();
    // needs the "cargo build and then cargo run to be ran oin the fucking serve"
}
