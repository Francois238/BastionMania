// @generated automatically by Diesel CLI.

diesel::table! {
    bastion (id) {
        id -> Int4,
        name -> Text,
        protocols -> Text,
        wireguard_id -> Nullable<Int4>,
    }
}

diesel::table! {
    client_config (privatekey) {
        privatekey -> Text,
        address -> Text,
        peerpublickey -> Text,
        peerallowedips -> Text,
        peerendpoint -> Text,
    }
}

diesel::table! {
    serveur_config (id) {
        id -> Int4,
        publikey -> Text,
        presharedkey -> Text,
        ip -> Text,
    }
}

diesel::joinable!(bastion -> serveur_config (wireguard_id));

diesel::allow_tables_to_appear_in_same_query!(
    bastion,
    client_config,
    serveur_config,
);
