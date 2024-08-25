// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (address) {
        #[max_length = 44]
        address -> Varchar,
        soft_delete -> Bool,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    collect_transactions (id) {
        id -> Int4,
        #[max_length = 44]
        withdraw_withheld_authority -> Varchar,
        #[max_length = 44]
        token -> Varchar,
        batch_size -> Int4,
        tx_signature -> Nullable<Varchar>,
        created_at -> Nullable<Timestamptz>,
    }
}

diesel::table! {
    tokens (address) {
        #[max_length = 44]
        address -> Varchar,
    }
}

diesel::joinable!(collect_transactions -> accounts (withdraw_withheld_authority));
diesel::joinable!(collect_transactions -> tokens (token));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    collect_transactions,
    tokens,
);
