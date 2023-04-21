// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Uuid,
        name -> Text,
        last_name -> Text,
        mail -> Text,
        password -> Nullable<Bytea>,
        change -> Nullable<Bool>,
        otp -> Nullable<Text>,
        otpactive -> Nullable<Bool>,
    }
}

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        last_name -> Text,
        mail -> Text,
        password -> Nullable<Bytea>,
        change -> Nullable<Bool>,
        otp -> Nullable<Text>,
        otpactive -> Nullable<Bool>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    admins,
    users,
);
