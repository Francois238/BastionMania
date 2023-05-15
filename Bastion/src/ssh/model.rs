use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicKey {
    pub algo: String,
    pub key: String,
}

impl PublicKey {
    pub fn from_str(s: &str) -> Result<PublicKey, String> {
        let mut parts = s.split(' ');
        let algo = parts.next().ok_or("No algo")?;
        let key = parts.next().ok_or("No key")?;
        Ok(PublicKey {
            algo: algo.to_string(),
            key: key.to_string(),
        })
    }

    pub fn to_string(&self) -> String {
        format!("{} {}", self.algo, self.key)
    }
}
