use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaVersion {
    pub id: String,
    pub path: String,
    pub version: String,
    pub vendor: String,
    pub architecture: String,
    pub is_managed: bool,
}

impl JavaVersion {
    pub fn new(id: String, path: String, version: String, vendor: String, architecture: String, is_managed: bool) -> Self {
        Self {
            id,
            path,
            version,
            vendor,
            architecture,
            is_managed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaRecommendation {
    pub mc_version: String,
    pub recommended_java: String,
    pub minimum_java: String,
    pub explanation: String,
}

impl JavaRecommendation {
    pub fn get_for_version(mc_version: &str) -> Self {
        let (recommended, minimum, explanation) = if mc_version < "1.17" {
            ("8", "8", "Minecraft before 1.17 needs Java 8")
        } else if mc_version < "1.18.2" {
            ("17", "17", "Minecraft 1.17 - 1.18.1 needs Java 17")
        } else {
            ("21", "17", "Minecraft 1.18.2+ works best on Java 21 (Java 17 minimum)")
        };
        Self {
            mc_version: mc_version.to_string(),
            recommended_java: recommended.to_string(),
            minimum_java: minimum.to_string(),
            explanation: explanation.to_string(),
        }
    }
}
