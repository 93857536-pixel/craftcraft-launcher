use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerBookmark {
    pub id: String,
    pub address: String,
    pub name: String,
    pub description: Option<String>,
    pub is_favorite: bool,
    pub last_accessed_at: Option<u64>,
    pub access_count: u64,
}

impl ServerBookmark {
    pub fn new(address: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            address: address.into(),
            name: name.into(),
            description: None,
            is_favorite: false,
            last_accessed_at: None,
            access_count: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub reachable: bool,
    pub motd: Option<String>,
    pub online: Option<u32>,
    pub max: Option<u32>,
    pub version: Option<String>,
    pub error: Option<String>,
}
