// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Uuid,
        name -> Text,
        last_name -> Text,
        mail -> Text,
    }
}
