use regex::Regex;
use std::env;

// Les clés wireguard (Curve25519) font 32bits, en base 64 cela fait 43 caractères plus le padding pour monter à 44 caractères.
// Le dernier caractères ne peux pas prendre toutes les valeurs de la base 64 mais uniquement certaines.
// https://lists.zx2c4.com/pipermail/wireguard/2020-December/006222.html
static REGEX_WG_KEYS: &str = r"^[A-Za-z0-9+/]{42}[A|E|I|M|Q|U|Y|c|g|k|o|s|w|4|8|0]=$";

// IPv4 : https://stackoverflow.com/questions/5284147/validating-ipv4-addresses-with-regexp
// CIDR : https://www.regextester.com/93987
// "ip/mask"
static REGEX_IPV4_CIDR: &str =
    r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}(/([0-9]|[1-2][0-9]|3[0-2]))$";

// "ip:port"
// le port peux être incorrect, il faut le vérifier
static REGEX_IPV4_PORT: &str = r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}:([\d]{1,5})$";

#[derive(Debug)]
pub struct BastionConfig {
    pub bastion_private_key: String,
    pub agent_endpoint: String,
    pub agent_public_key: String,
    pub net_cidr: String,
    pub net_id: u8,
}

impl BastionConfig {
    pub fn new() -> BastionConfig {
        let bastion_private_key =
            env::var("BASTION_PRIVATE_KEY").expect("BASTION_PRIVATE_KEY must be set");
        let agent_endpoint = env::var("AGENT_ENDPOINT").expect("AGENT_ENDPOINT must be set");
        let agent_public_key = env::var("AGENT_PUBLIC_KEY").expect("AGENT_PUBLIC_KEY must be set");
        let net_cidr = env::var("NET_CIDR").expect("NET_CIDR must be set");
        let net_id = env::var("NET_ID").expect("NET_ID must be set");

        let re_keys = Regex::new(REGEX_WG_KEYS).unwrap();
        let re_ipv4_cidr = Regex::new(REGEX_IPV4_CIDR).unwrap();
        let re_ipv4_port = Regex::new(REGEX_IPV4_PORT).unwrap();

        if !re_keys.is_match(&bastion_private_key) {
            panic!("Invalid private key : {}", bastion_private_key);
        }
        if !re_keys.is_match(&agent_public_key) {
            panic!("Invalid agent public key : {}", agent_public_key);
        }

        if !re_ipv4_cidr.is_match(&net_cidr) {
            panic!("Invalid CIDR : {}", net_cidr);
        }

        let endpoint_match = re_ipv4_port
            .captures(&agent_endpoint)
            .unwrap_or_else(|| panic!("Invalid endpoint : {}", agent_endpoint));
        let port = endpoint_match.get(4).unwrap().as_str();
        let port: u16 = port
            .parse()
            .unwrap_or_else(|_| panic!("Invalid endpoint port : {}", port));
        if port < 1024 {
            panic!("Invalid endpoint port : {}", port);
        }

        let net_id: u8 = net_id
            .parse()
            .unwrap_or_else(|_| panic!("Invalid net id : {}", net_id));

        BastionConfig {
            bastion_private_key,
            agent_endpoint,
            agent_public_key,
            net_cidr,
            net_id,
        }
    }
}
