table! {
    account (id) {
        id -> Varchar,
        owner_name -> Varchar,
        number -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    pixkey (id) {
        id -> Varchar,
        kind -> Varchar,
        key -> Varchar,
        status -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        account_id -> Varchar,
    }
}

joinable!(pixkey -> account (account_id));

allow_tables_to_appear_in_same_query!(account, pixkey,);
