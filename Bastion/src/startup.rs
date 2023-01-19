use std::fs;
use std::io::Result;
use std::process::Command;
use std::path::Path;

static COMMAND_WIREGUARD: &str = "/usr/bin/wireguard-go";
static COMMAND_MKNOD: &str = "/bin/mknod";
static COMMAND_IPTABLES: &str = "/sbin/iptables";

fn get_ipv4_forward_status() -> Result<bool> {
    let str_ip4f = fs::read_to_string("/proc/sys/net/ipv4/ip_forward")?;
    Ok(str_ip4f == "1\n")
}

fn init_routing() {
    print!("IPv4 routing : ");
    // Check the status of ipv4 forwarding
    let active = get_ipv4_forward_status().expect("Can't read ipv4 forward status");
    let status = if active {
        "OK"
    } else {
        "FAIL"
    };
    println!("{}", status);
    if !active {
        panic!("IPv4 routing must be enable");
    }
}

fn create_tun_device() {
    // create /dev/net if not exist
    let exist = Path::new("/dev/net").exists();
    if !exist {
        println!("Creating /dev/net");
        fs::create_dir("/dev/net").expect("Can't create /dev/net");
    }
    // create tun interface
    let output = Command::new(COMMAND_MKNOD)
        .arg("/dev/net/tun").arg("c").arg("10").arg("200")
        .output()
        .expect("Failed to create /dev/net/tun");
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stderr));
    }
}

fn add_wg_interface(name: &str) {
    let output = Command::new(COMMAND_WIREGUARD)
        .arg(name)
        .output()
        .expect("Failed to execute");
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        panic!("Can't create wireguard interface");
    }
}

// iptables -t nat -I POSTROUTING -o eth0 -j MASQUERADE
fn iptables_masquerade() {
    let output = Command::new(COMMAND_IPTABLES)
        .arg("-t").arg("nat")
        .arg("-I").arg("POSTROUTING")
        .arg("-o").arg("wg-agent")
        .arg("-j").arg("MASQUERADE")
        .output()
        .expect("Failed to execute MASQUERADE");
    if !output.status.success() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
        panic!("Can't create ip masquerading");
    }
}

pub fn startup() {
    init_routing();
    create_tun_device();
    add_wg_interface("wg-agent");
    add_wg_interface("wg-client");
    iptables_masquerade();
}