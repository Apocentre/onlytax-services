// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (address) {
        #[max_length = 44]
        address -> Varchar,
        soft_delete -> Bool,
        created_at -> Nullable<Timestamptz>,
    }
}
