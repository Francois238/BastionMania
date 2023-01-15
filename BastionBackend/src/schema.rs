// @generated automatically by Diesel CLI.

diesel::table! {
    bastion (id) {
        id -> Int4,
        name -> Text,
        subnet_cidr -> Text,
        agent_endpoint -> Text,
        pubkey -> Text,
        port -> Int4,
        net_id -> Int4,
    }
}

diesel::table! {
    user (id) {
        id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        bastion_id -> Int4,
        wireguard -> Bool,
        net_id -> Int4,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bastion,
    user,
    users,
);
