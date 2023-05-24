pub const CMD_USERADD: &str = "/usr/sbin/adduser";
pub const CMD_USERDEL: &str = "/usr/sbin/deluser";
pub const CMD_PASSWD: &str = "/usr/bin/passwd";

pub const CMD_SSHKEYGEN: &str = "/usr/bin/ssh-keygen";

pub static CMD_WIREGUARD: &str = "/usr/bin/wireguard-go";
pub static CMD_MKNOD: &str = "/bin/mknod";
pub static CMD_IPTABLES: &str = "/sbin/iptables";

pub const WG_PRIVATE_KEY_PATH: &str = "/data/wg_private_key";

// Les clés wireguard (Curve25519) font 32bits, en base 64 cela fait 43 caractères plus le padding pour monter à 44 caractères.
// Le dernier caractères ne peux pas prendre toutes les valeurs de la base 64 mais uniquement certaines.
// https://lists.zx2c4.com/pipermail/wireguard/2020-December/006222.html
pub static REGEX_WG_KEYS: &str = r"^[A-Za-z0-9+/]{42}[A|E|I|M|Q|U|Y|c|g|k|o|s|w|4|8|0]=$";

// IPv4 : https://stackoverflow.com/questions/5284147/validating-ipv4-addresses-with-regexp
// CIDR : https://www.regextester.com/93987
// "ip/mask"
pub static REGEX_IPV4_CIDR: &str =
    r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}(/([0-9]|[1-2][0-9]|3[0-2]))$";

// "ip:port"
// le port peux être incorrect, il faut le vérifier
pub static REGEX_IPV4_PORT: &str = r"^((25[0-5]|(2[0-4]|1\d|[1-9]|)\d)\.?\b){4}:([\d]{1,5})$";