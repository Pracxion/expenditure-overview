use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub enum SchemaKey {
    MBS,
    DKB,
}

impl SchemaKey {
    pub fn as_str(&self) -> &'static str {
        match self {
            SchemaKey::MBS => "mbs",
            SchemaKey::DKB => "dkb",
        }
    }
    
    pub fn all_keys() -> Vec<SchemaKey> {
        vec![
            SchemaKey::MBS,
            SchemaKey::DKB,
        ]
    }
}

impl fmt::Display for SchemaKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl From<&str> for SchemaKey {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "mbs" => SchemaKey::MBS,
            "dkb" => SchemaKey::DKB,
            _ => panic!("Invalid schema key: {}", s),
        }
    }
} 