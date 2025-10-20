// @generated automatically by Diesel CLI.

diesel::table! {
    divisions (id) {
        id -> Integer,
        #[max_length = 25]
        name -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        pass -> Varchar,
        #[max_length = 5]
        role -> Varchar,
        division_id -> Integer,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(users -> divisions (division_id));

diesel::allow_tables_to_appear_in_same_query!(divisions, users,);
