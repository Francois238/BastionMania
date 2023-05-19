use std::env;

static DEFAULT_BASTION_PORT: i32 = 60000;
static DEFAULT_BASTION_NET_ID: i32 = 2;
static DEFAULT_USER_NET_ID: i32 = 2;
static DEFAULT_RESSOURCE_WIREGUARD_ID: i32 = 0;
static DEFAULT_RESSOURCE_SSH_ID: i32 = 0;
static DEFAULT_RESSOURCE_K8S_ID: i32 = 0;

pub fn generate_bastion_freeport(ports: &Vec<i32>) -> i32 {
    let mut port = env::var("FIRST_PORT")
        .unwrap_or(DEFAULT_BASTION_PORT.to_string())
        .parse::<i32>()
        .unwrap_or(DEFAULT_BASTION_PORT);
    while ports.contains(&port) {
        port += 1;
    }
    port
}

pub fn generate_bastion_freenetid(net_ids: &Vec<i32>) -> i32 {
    let mut net_id = env::var("FIRST_NET_ID")
        .unwrap_or(DEFAULT_BASTION_NET_ID.to_string())
        .parse::<i32>()
        .unwrap_or(DEFAULT_BASTION_NET_ID);
    while net_ids.contains(&net_id) {
        net_id += 1;
    }
    net_id
}

pub fn generate_user_freenetid(net_ids: &Vec<i32>) -> i32 {
    let mut net_id = env::var("FIRST_USER_NET_ID")
        .unwrap_or(DEFAULT_USER_NET_ID.to_string())
        .parse::<i32>()
        .unwrap_or(DEFAULT_USER_NET_ID);
    while net_ids.contains(&net_id) {
        net_id += 1;
    }
    net_id
}


pub fn generate_ressource_wireguard_freenetid(net_ids: &Vec<i32>) -> i32 {
    let mut net_id = env::var("FIRST_WIREGUARD_NET_ID")
        .unwrap_or(DEFAULT_RESSOURCE_WIREGUARD_ID.to_string())
        .parse::<i32>()
        .unwrap_or(DEFAULT_RESSOURCE_WIREGUARD_ID);
    while net_ids.contains(&net_id) {
        net_id += 1;
    }
    net_id
}

pub fn generate_ressource_ssh_freenetid(net_ids: &Vec<i32>) -> i32 {
    let mut net_id = env::var("FIRST_SSH_NET_ID")
        .unwrap_or(DEFAULT_RESSOURCE_SSH_ID.to_string())
        .parse::<i32>()
        .unwrap_or(DEFAULT_RESSOURCE_SSH_ID);
    while net_ids.contains(&net_id) {
        net_id += 1;
    }
    net_id
}

pub fn generate_ressource_k8s_freenetid(net_ids: &Vec<i32>) -> i32 {
    let mut net_id = env::var("FIRST_K8S_NET_ID")
        .unwrap_or(DEFAULT_RESSOURCE_K8S_ID.to_string())
        .parse::<i32>()
        .unwrap_or(DEFAULT_RESSOURCE_K8S_ID);
    while net_ids.contains(&net_id) {
        net_id += 1;
    }
    net_id
}
