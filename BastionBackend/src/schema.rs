// @generated automatically by Diesel CLI.

diesel::table! {
    bastion (id) {
        id -> Int4,
        name -> Text,
        subnet_cidr -> Text,
        pubkey -> Text,
        port -> Int4,
        net_id -> Int4,
    }
}
