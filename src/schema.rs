// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (wallet_address) {
        id -> Int4,
        #[max_length = 50]
        wallet_address -> Varchar,
        #[max_length = 255]
        nickname -> Nullable<Varchar>,
        #[max_length = 255]
        profile_picture -> Nullable<Varchar>,
    }
}

diesel::table! {
    memecoins (contract_address) {
        #[max_length = 42]
        contract_address -> Varchar,
        creator_id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 20]
        symbol -> Varchar,
        cap -> Int4,
        #[max_length = 255]
        icon -> Nullable<Varchar>,
        description -> Nullable<Text>,
        links -> Nullable<Jsonb>,
        market_cap -> Nullable<Int4>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    threads (id) {
        id -> Int4,
        #[max_length = 50]
        memecoin -> Varchar,
        timestamp -> Nullable<Timestamptz>,
        author -> Int4,
        text -> Text,
        #[max_length = 255]
        image -> Nullable<Varchar>,
    }
}

diesel::table! {
    trades (tx_hash) {
        #[max_length = 70]
        tx_hash -> Varchar,
        #[max_length = 50]
        memecoin -> Varchar,
        timestamp -> Timestamptz,
        #[max_length = 42]
        initiator -> Varchar,
        #[sql_name = "type"]
        #[max_length = 4]
        type_ -> Varchar,
        #[max_length = 46]
        amount_eth -> Varchar,
        #[max_length = 46]
        amount_token -> Varchar,
    }
}

diesel::joinable!(threads -> memecoins (memecoin));
diesel::joinable!(trades -> memecoins (memecoin));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    memecoins,
    threads,
    trades,
);
