// @generated automatically by Diesel CLI.

diesel::table! {
    agent (id) {
        id -> Int4,
        privatekey -> Text,
        publickey -> Text,
        address -> Text,
    }
}

diesel::table! {
    bastion (id) {
        id -> Int4,
        name -> Text,
        protocols -> Text,
        subnet_cidr -> Text,
        endpoint_ip -> Text,
        endpoint_port -> Text,
        serveur_config_id -> Text,
        client_config_id -> Text,
    }
}

diesel::table! {
    to_agent_config (id) {
        id -> Int4,
        privatekey -> Text,
        publickey -> Text,
        address -> Text,
        peerallowedips -> Text,
        peerendpoint -> Text,
    }
}

diesel::table! {
    to_user_config (id) {
        id -> Int4,
        publickey -> Text,
        privatekey -> Text,
        ip -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    agent,
    bastion,
    to_agent_config,
    to_user_config,
);
