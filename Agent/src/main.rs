use std::env;
use bastion_mania_agent::wireguard::WireguardConf;

fn main() {
    let config = WireguardConf{
        priv_key: env::var("PRIVATE_KEY").expect("need PRIVATE_KEY"),
        listen_port: env::var("LISTEN_PORT").expect("need LISTEN_PORT"),
        address: env::var("ADDRESS").expect("need ADDRESS"),
        peer_public: env::var("PEER_PUBLIC").expect("need PEER_PUBLIC"),
        peer_ip: env::var("PEER_IP").expect("need PEER_IP"),
    };

    println!("config: {:?}" ,config);
    match config.validate() {
        Ok(_) => println!("Valid config"),
        Err(e) => {
            println!("Invalid config: {}", e);
            panic!();
        },
    }

    println!("{}", config.to_string());
}
