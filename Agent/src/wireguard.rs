use regex::Regex;

// Les clés wireguard (Curve25519) font 32bits, en base 64 cela fait 43 caractères plus le padding pour monter à 44 caractères.
// Le dernier caractères ne peux pas prendre toutes les valeurs de la base 64 mais uniquement certaines.
// https://lists.zx2c4.com/pipermail/wireguard/2020-December/006222.html
static REGEX_WG_KEYS: &str = r"^[A-Za-z0-9+/]{42}[A|E|I|M|Q|U|Y|c|g|k|o|s|w|4|8|0]=$";

static WG_LISTEN_PORT: u16 = 60469;
static WG_ADDRESS: &str = "10.10.1.1";
static WG_PEER_ADDR: &str = "10.10.1.2";

#[derive(Debug)]
pub struct WireguardConf {
    pub priv_key: String,
    pub peer_public: String,
}

impl WireguardConf {
    pub fn validate(&self) -> Result<(), String> {
        let re_keys = Regex::new(REGEX_WG_KEYS).unwrap();

        if !re_keys.is_match(&self.priv_key) {
            return Err("Invalid private key : ".to_string() + &self.priv_key);
        }
        if !re_keys.is_match(&self.peer_public) {
            return Err("Invalid peer public key : ".to_string() + &self.peer_public);
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
        ", self.priv_key, WG_LISTEN_PORT, WG_ADDRESS, self.peer_public, WG_PEER_ADDR)
    }
}