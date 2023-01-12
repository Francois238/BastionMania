use std::env;

static DEFAULT_PORT: i32= 60000;
static DEFAULT_NET_ID: i32= 2;

pub fn generate_freeport(ports: &Vec<i32>) -> i32 {
    let mut port = env::var("FIRST_PORT").unwrap_or(DEFAULT_PORT.to_string()).parse::<i32>().unwrap_or(DEFAULT_PORT);
    while ports.contains(&port){
        port += 1;
    }
    port
}

pub fn generate_freenetid(net_ids: &Vec<i32>) -> i32 {
    let mut net_id = env::var("FIRST_NET_ID").unwrap_or(DEFAULT_NET_ID.to_string()).parse::<i32>().unwrap_or(DEFAULT_NET_ID);
    while net_ids.contains(&net_id){
        net_id += 1;
    }
    net_id
}