// @generated automatically by Diesel CLI.

diesel::table! {
    bastion (bastion_id) {
        bastion_id -> Text,
        name -> Text,
        subnet_cidr -> Text,
        agent_endpoint -> Text,
        pubkey -> Text,
        port -> Int4,
        net_id -> Int4,
    }
}

diesel::table! {
    bastion_token (token) {
        token -> Text,
        bastion_id -> Text,
    }
}

diesel::table! {
    k8sressource (id) {
        id -> Int4,
        id_bastion -> Text,
        name -> Text,
        ip_cluster -> Text,
    }
}

diesel::table! {
    ressource (id) {
        id -> Text,
        id_bastion -> Text,
        name -> Text,
        rtype -> Text,
        id_wireguard -> Nullable<Int4>,
        id_ssh -> Nullable<Int4>,
        id_k8s -> Nullable<Int4>,
    }
}

diesel::table! {
    sshressource (id) {
        id -> Int4,
        id_bastion -> Text,
        name -> Text,
        ip_machine -> Text,
        port -> Int4,
    }
}

diesel::table! {
    user_config_ssh (id) {
        id -> Int4,
        uuid_user -> Text,
        uuid_ressource -> Text,
        pubkey -> Text,
        username -> Text,
    }
}

diesel::table! {
    user_config_wireguard (id) {
        id -> Int4,
        uuid_user -> Text,
        uuid_ressource -> Text,
        pubkey -> Text,
        user_net_id -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        user_id -> Text,
        ressource_id -> Text,
        net_id -> Int4,
    }
}

diesel::table! {
    wireguardressource (id) {
        id -> Int4,
        id_bastion -> Text,
        name -> Text,
        subnet_cidr -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bastion,
    bastion_token,
    k8sressource,
    ressource,
    sshressource,
    user_config_ssh,
    user_config_wireguard,
    users,
    wireguardressource,
);
