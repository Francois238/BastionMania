use std::fs;

use crate::ssh::ressource::SSHRessource;
use crate::ssh::user::SSHUser;

pub struct AuthorizedKeys {
    keys: Vec<AuthorizedKey>,
}

pub struct AuthorizedKey {
    pub command: String,
    pub key: String,
    pub comment: String,
}

impl AuthorizedKey {
    pub fn new(ressource: &SSHRessource, user: &SSHUser) -> Result<AuthorizedKey, String> {
        Ok(AuthorizedKey {
            command: format!(
                "command=\"ssh -p {} {}@{}\"",
                ressource.port, user.name, ressource.ip
            ),
            key: user.public_key.to_string(),
            comment: user.id.to_string(),
        })
    }

    pub fn to_string(&self) -> String {
        format!("{} {} {}", self.command, self.key, self.comment)
    }

    /// Parse une ligne de fichier authorized_keys
    pub fn from_line(line: &str) -> Result<AuthorizedKey, String> {
        static START_COMMAND_KEY: &str = "command=\"";
        static START_COMMAND_KEY_SIZE: usize = 9;

        let start_command = line.find(START_COMMAND_KEY).ok_or("No command")?;
        let end_command = START_COMMAND_KEY_SIZE
            + line[start_command + START_COMMAND_KEY_SIZE..]
                .find("\"")
                .ok_or("No end command")?;
        let command = line[start_command..start_command + 1].to_string();

        let after_command = line[end_command + 1..].trim();
        let mut parts = after_command.split(' ');
        let algo = parts.next().ok_or("No algo")?;
        let key = parts.next().ok_or("No key")?;
        let comment = parts.next().ok_or("No comment")?;

        Ok(AuthorizedKey {
            command,
            key: format!("{} {}", algo, key),
            comment: comment.to_string(),
        })
    }
}

impl AuthorizedKeys {
    pub fn new() -> AuthorizedKeys {
        AuthorizedKeys { keys: Vec::new() }
    }

    pub fn add_key(&mut self, key: AuthorizedKey) {
        self.keys.push(key);
    }

    /// Retourne la liste des clés autorisées pour une ressource
    pub fn from_path(path: &str) -> Result<AuthorizedKeys, String> {
        let mut authorized_keys = AuthorizedKeys::new();
        if let Ok(lines) = fs::read_to_string(path) {
            for line in lines.lines() {
                authorized_keys.add_key(AuthorizedKey::from_line(line)?);
            }
        } else {
            return Err(format!("Error reading authorized_key file : {}", path));
        }
        Ok(authorized_keys)
    }

    pub fn save(&self, path: &str) -> Result<(), String> {
        let mut content = String::new();
        for key in &self.keys {
            content.push_str(format!("{}\n", key.to_string()).as_str());
        }
        fs::write(path, content)
            .map_err(|e| format!("Error saving authorized_keys file: {}", e))?;
        Ok(())
    }
}
