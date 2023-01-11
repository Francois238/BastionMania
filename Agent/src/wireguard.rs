use std::num::ParseIntError;
use regex::Regex;

// https://stackoverflow.com/questions/5284147/validating-ipv4-addresses-with-regexp
static REGEX_IPV4: &str = r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}$";

// Les clés wireguard (Curve25519) font 32bits, en base 64 cela fait 43 caractères plus le padding pour monter à 44 caractères.
// Le dernier caractères ne peux pas prendre toutes les valeurs de la base 64 mais uniquement certaines.
// https://lists.zx2c4.com/pipermail/wireguard/2020-December/006222.html
static REGEX_WG_KEYS: &str = r"^[A-Za-z0-9+/]{42}[A|E|I|M|Q|U|Y|c|g|k|o|s|w|4|8|0]=$";

#[derive(Debug)]
pub struct WireguardConf {
    pub priv_key: String,
    pub listen_port: String,
    pub address: String,
    pub peer_public: String,
    pub peer_ip: String,
}

impl WireguardConf {
    pub fn validate(&self) -> Result<(), String> {
        let re_ipv4 = Regex::new(REGEX_IPV4).unwrap();
        let re_keys = Regex::new(REGEX_WG_KEYS).unwrap();

        if !re_keys.is_match(&self.priv_key) {
            return Err("Invalid private key : ".to_string() + &self.priv_key);
        }
        if !re_keys.is_match(&self.peer_public) {
            return Err("Invalid peer public key : ".to_string() + &self.peer_public);
        }
        if !re_ipv4.is_match(&self.address) {
            return Err("Invalid address : ".to_string() + &self.address);
        }
        if !re_ipv4.is_match(&self.peer_ip) {
            return Err("Invalid peer ip".to_string() + &self.peer_ip);
        }
        let port: u16 = self.listen_port.parse().map_err(|e: ParseIntError| e.to_string())?;
        if port < 1024 {
            return Err("Invalid listen port : ".to_string() + &self.listen_port);
        }
        Ok(())
    }

    pub fn to_string(&self) -> String {
        format!("\
        [Interface]\n\
        PrivateKey = {}\n\
        ListenPort = {}\n\
        Address = {}/24\n\
        PostUp = iptables -t nat -I POSTROUTING -o eth0 -j MASQUERADE\n\
        PreDown = iptables -t nat -D POSTROUTING -o eth0 -j MASQUERADE\n\
        \n\
        [Peer]\n\
        PublicKey = {}\n\
        AllowedIPs = {}/32\n\
        ", self.priv_key, self.listen_port, self.address, self.peer_public, self.peer_ip)
    }
}