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
    k8sSession (name) {
        name -> Text,
        ip_cluster -> Text,
    }
}

diesel::table! {
    ressource (id) {
        id -> Text,
        id_bastion -> Text,
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
        id_wireguard -> Text,
        id_ssh -> Text,
        id_k8s -> Text,
    }
}

diesel::table! {
    session (id) {
        id -> Text,
        id_ressource -> Text,
        id_user -> Text,
        temps_fin -> Int4,
    }
}

diesel::table! {
    sshSession (name) {
        name -> Text,
        ip_machine -> Text,
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
        user_id -> Int4,
        bastion_id -> Int4,
        wireguard -> Bool,
        net_id -> Int4,
    }
}

diesel::table! {
    wireguardSession (name) {
        name -> Text,
        target_cidr -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    bastion,
    k8sSession,
    ressource,
    session,
    sshSession,
    user,
    users,
    wireguardSession,
);
