use std::{env, fs, thread, time};
use std::process::Command;
use bastion_mania_agent::wireguard::WireguardConf;

fn main() {
    let config = WireguardConf {
        priv_key: env::var("PRIVATE_KEY").expect("need PRIVATE_KEY"),
        peer_public: env::var("PEER_PUBLIC").expect("need PEER_PUBLIC"),
    };

    match config.validate() {
        Ok(_) => println!("Valid config"),
        Err(e) => {
            println!("Invalid config: {}", e);
            panic!();
        }
    }

    fs::write("/etc/wireguard/wg0.conf", config.to_string()).expect("Unable to write file");

    let out = Command::new("/usr/bin/wg-quick")
        .arg("up")
        .arg("wg0")
        .output()
        .expect("failed to execute process");

    if !out.status.success() {
        println!("stdout: {}", String::from_utf8_lossy(&out.stdout));
        println!("Error: {}", String::from_utf8_lossy(&out.stderr));
        panic!();
    }

    println!("Wireguard server started");

    // infinite sleep
    loop {
        thread::sleep(time::Duration::from_secs(1000));
    }
}
