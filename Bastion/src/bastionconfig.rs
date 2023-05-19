use std::env;

#[derive(Debug)]
pub struct BastionConfig {
    pub net_id: u8,
}

impl BastionConfig {
    pub fn new() -> BastionConfig {
        let net_id = env::var("NET_ID").expect("NET_ID must be set");

        let net_id: u8 = net_id
            .parse()
            .unwrap_or_else(|_| panic!("Invalid net id : {}", net_id));

        BastionConfig {
            net_id,
        }
    }
}
