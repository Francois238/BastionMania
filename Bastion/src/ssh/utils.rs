use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::Command,
};

use hex;

use base64::{engine, Engine};
use regex::Regex;
use sha256::digest;

pub fn kill_all_sessions(ressource_name: &str, public_key: &str) -> Result<(), String> {
    let session_ids = get_lasts_session_id(public_key, ressource_name)?;
    for sid in session_ids {
        kill_session(&sid)?;
    }
    Ok(())
}

fn public_key_fingerprint(key: &str) -> Result<String, String> {
    let engine = engine::general_purpose::STANDARD_NO_PAD;
    // base64 decode
    let bin_key = engine.decode(key).map_err(|_| "Error decoding key")?;

    // sha256 hash
    let hash = digest(bin_key.as_slice());
    let hash = hex::decode(hash).map_err(|_| "Error decoding hash")?;
    // base64 encode
    let fingerprint = engine.encode(hash);
    Ok(format!("SHA256:{}", fingerprint))
}

/// Get all the sessions ids for a user on a ressouce
fn get_lasts_session_id(key: &str, res_name: &str) -> Result<Vec<String>, String> {
    let fingerprint = public_key_fingerprint(key).map_err(|_| "Error getting fingerprint")?;

    let file = File::open("/var/log/messages").map_err(|_| "Error opening messages")?;
    let reader = BufReader::new(file);

    let mut session_ids = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|_| "Error reading line")?;
        if let Some(sid) = get_sid_line(&line, res_name, &fingerprint) {
            session_ids.push(sid);
        }
    }

    Ok(session_ids)
}
/// Extract the session id from a log line
///
/// Return None if the line does not match
///
/// Example of line:
/// `2023-05-16T11:19:07.803797+00:00 1fbfc50b3291 sshd[2189]: Accepted publickey for sulfin from 172.17.0.1 port 52146 ssh2: ED25519 SHA256:mh0cV0c0hYvdUQ709wx6v9CmynetBr1QJo7rCWzgpv4`
fn get_sid_line(line: &str, res_name: &str, key: &str) -> Option<String> {
    static REG_SID: &str =
        r"\[([0-9]+)\]: Accepted publickey for (.+) from.+(SHA256:[A-Za-z0-9+/]+)";
    let re = Regex::new(REG_SID).ok()?;
    let caps = re.captures(line)?;
    let sid = caps.get(1)?.as_str();
    let ressource_name = caps.get(2)?.as_str();
    let fingerprint = caps.get(3)?.as_str();

    if ressource_name == res_name && fingerprint == key {
        Some(sid.to_string())
    } else {
        None
    }
}

fn kill_session(sid: &str) -> Result<(), String> {
    Command::new("/usr/bin/pkill")
        .arg("-s")
        .arg(sid)
        .output()
        .map_err(|_| "Error killing session")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_public_key_fingerprint() {
        let public_key = "AAAAC3NzaC1lZDI1NTE5AAAAIIWpkQszgFqwxdolm7gqK5D3fbfdDESZCaEw74She+eH";
        let fingerprint = public_key_fingerprint(public_key).unwrap();
        assert_eq!(
            fingerprint,
            "SHA256:mh0cV0c0hYvdUQ709wx6v9CmynetBr1QJo7rCWzgpv4"
        );
    }

    #[test]
    fn test_get_sid_line_real() {
        let line = "2023-05-16T11:19:07.803797+00:00 1fbfc50b3291 sshd[2189]: Accepted publickey for sulfin from 172.17.0.1 port 52146 ssh2: ED25519 SHA256:mh0cV0c0hYvdUQ709wx6v9CmynetBr1QJo7rCWzgpv4";
        let sid = get_sid_line(
            line,
            "sulfin",
            "SHA256:mh0cV0c0hYvdUQ709wx6v9CmynetBr1QJo7rCWzgpv4",
        );
        assert_eq!(sid, Some("2189".to_string()));
    }

    #[test]
    fn test_get_sid_line_wrong_ressource() {
        let line = "2023-05-16T11:19:07.803797+00:00 1fbfc50b3291 sshd[2189]: Accepted publickey for dede from 172.17.0.1 port 52146 ssh2: ED25519 SHA256:mh0cV0c0hYvdUQ709wx6v9CmynetBr1QJo7rCWzgpv4";
        let sid = get_sid_line(
            line,
            "sulfin",
            "SHA256:mh0cV0c0hYvdUQ709wx6v9CmynetBr1QJo7rCWzgpv4",
        );
        assert_eq!(sid, None);
    }
}
