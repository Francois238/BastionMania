// @generated automatically by Diesel CLI.

diesel::table! {
    admins (id) {
        id -> Uuid,
        name -> Text,
        last_name -> Text,
        mail -> Text,
    }
}
