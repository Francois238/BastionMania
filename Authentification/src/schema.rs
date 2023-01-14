// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Int4,
        name -> Text,
        last_name -> Text,
        mail -> Text,
        password -> Bytea,
        change -> Bool,
        otp -> Nullable<Text>,
        otpactive -> Bool,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        name -> Text,
        last_name -> Text,
        mail -> Text,
        password -> Bytea,
        change -> Bool,
        otp -> Nullable<Text>,
        otpactive -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    users,
);
