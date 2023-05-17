use std::fs;
use std::io::Result;
use std::path::Path;
use std::process::Command;

use log::{error, info, debug};

static COMMAND_WIREGUARD: &str = "/usr/bin/wireguard-go";
static COMMAND_MKNOD: &str = "/bin/mknod";
static COMMAND_IPTABLES: &str = "/sbin/iptables";

fn get_ipv4_forward_status() -> Result<bool> {
    let str_ip4f = fs::read_to_string("/proc/sys/net/ipv4/ip_forward")?;
    Ok(str_ip4f == "1\n")
}

fn init_routing() {
    // Check the status of ipv4 forwarding
    let active = get_ipv4_forward_status().expect("Can't read ipv4 forward status");
    if !active{
        error!("ipv4 forwarding is not active");
        panic!("ipv4 forwarding is not active");
    }
    info!("ipv4 forwarding is active");

}

fn create_tun_device() {
    // create /dev/net if it doesn't exist
    let exist = Path::new("/dev/net").exists();
    if !exist {
        debug!("Creating /dev/net");
        fs::create_dir("/dev/net").expect("Can't create /dev/net");
    }
    // create tun interface
    let output = Command::new(COMMAND_MKNOD)
        .arg("/dev/net/tun")
        .arg("c")
        .arg("10")
        .arg("200")
        .output()
        .expect("Failed to create /dev/net/tun");
    if !output.status.success() {
        error!("{}", String::from_utf8_lossy(&output.stderr));
        panic!("Can't create /dev/net/tun");
    }
}

fn add_wg_interface(name: &str) {
    let output = Command::new(COMMAND_WIREGUARD)
        .arg(name)
        .output()
        .expect("Failed to execute");
    if !output.status.success() {
        error!("{}", String::from_utf8_lossy(&output.stdout));
        panic!("Can't create wireguard interface");
    }
}

/// Create ip masquerading for wireguard
/// 
///  `iptables -t nat -I POSTROUTING -o wg-agent -j MASQUERADE`
fn iptables_masquerade() {
    let output = Command::new(COMMAND_IPTABLES)
        .arg("-t")
        .arg("nat")
        .arg("-I")
        .arg("POSTROUTING")
        .arg("-o")
        .arg("wg-agent")
        .arg("-j")
        .arg("MASQUERADE")
        .output()
        .expect("Failed to create ip MASQUERADE");
    if !output.status.success() {
        error!("{}", String::from_utf8_lossy(&output.stdout));
        panic!("Can't create ip masquerading");
    }
}

pub fn init() {
    init_routing();
    create_tun_device();
    add_wg_interface("wg-agent");
    add_wg_interface("wg-client");
    iptables_masquerade();
}
