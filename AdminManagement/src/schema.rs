// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Int4,
        name -> Text,
        last_name -> Text,
        mail -> Text,
        password -> Bytea,
    }
}
