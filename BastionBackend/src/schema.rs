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
    k8sressource (id) {
        id -> Int4,
        id_bastion -> Int4,
        name -> Text,
        ip_cluster -> Text,
    }
}

diesel::table! {
    ressource (id) {
        id -> Int4,
        id_bastion -> Int4,
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
        id_bastion -> Int4,
        name -> Text,
        ip_machine -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        user_id -> Int4,
        bastion_id -> Int4,
        wireguard -> Bool,
        net_id -> Int4,
    }
}

diesel::table! {
    wireguardressource (id) {
        id -> Int4,
        id_bastion -> Int4,
        name -> Text,
        subnet_cidr -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bastion,
    k8sressource,
    ressource,
    sshressource,
    users,
    wireguardressource,
);
