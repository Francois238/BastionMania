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
