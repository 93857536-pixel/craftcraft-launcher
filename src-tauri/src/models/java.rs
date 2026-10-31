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

    /// Does this runtime satisfy a requested release label?
    ///
    /// Handles the two naming schemes seen in the wild:
    ///  • Java 8 reports `1.8.0_504` while the UI wants `"8"`
    ///  • Java 17/21 report `17.0.9` / `21.0.13` and want `"17"` / `"21"`
    ///
    /// `want` may also be given as `"1.8"` for callers that already carry the
    /// legacy prefix.
    pub fn matches_release(&self, want: &str) -> bool {
        let want = want.trim();
        let have = self.version.trim();
        if want.is_empty() || have.is_empty() {
            return false;
        }
        // Exact match or prefix match on a dotted boundary (21 vs 21.0.13).
        if have == want || have.starts_with(&format!("{want}.")) {
            return true;
        }
        // Legacy 1.x naming: "8" ≙ "1.8.0_504", "1.8" ≙ "1.8.0_504".
        let legacy = if want.starts_with("1.") {
            want.to_string()
        } else {
            format!("1.{want}")
        };
        have.starts_with(&legacy)
            || have.starts_with(&format!("{legacy}."))
            || have.starts_with(&format!("{legacy}_"))
            || have.starts_with(&legacy.replace('.', "u"))
    }
}

/// Does this Minecraft version ship arm64 macOS natives?
///
/// Mojang only started publishing arm64-native LWJGL builds with 1.19. Everything
/// up to 1.18.2 carries x86_64-only `.dylib`s, so on Apple Silicon those versions
/// must run on an x86_64 JVM (Rosetta 2) — an aarch64 JVM cannot dlopen them.
pub fn uses_arm64_natives(mc_version: &str) -> bool {
    let v = mc_version.trim();
    // Numeric scheme ("21w44a", "26.3", …) is always modern.
    if !v.starts_with("1.") {
        return true;
    }
    let mut parts = v.split('.');
    let _major = parts.next();
    let minor: u32 = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    minor >= 19
}

/// True when the host is Apple Silicon *and* the game version needs an x86_64 JVM.
/// 把 JDK 版本串解析成主版本号：`"17.0.21"` → 17，`"1.8.0_504"` → 8。
pub fn major_of(version: &str) -> Option<u32> {
    let v = version.trim();
    if v.is_empty() {
        return None;
    }
    // Legacy `1.x.y_z` naming: the real major is the second component.
    let mut it = v.split(['.', '_', '-', '+']);
    let first = it.next()?.trim().parse::<u32>().ok()?;
    if first == 1 {
        return it.next()?.trim().parse::<u32>().ok();
    }
    Some(first)
}

/// JDK 是否**够用**跑这个版本的 MC：主版本必须 >= 要求。
/// 高于要求可以用（21 跑 1.12.2 没问题），**低于要求绝不行**——
/// 否则就是 `UnsupportedClassVersionError: class file version 61.0 ... up to 52.0`
/// （用 Java 8 跑 1.18.2 的经典症状）。
pub fn satisfies_requirement(have_version: &str, want: &str) -> bool {
    match (major_of(have_version), major_of(want)) {
        (Some(have), Some(need)) => have >= need,
        _ => false,
    }
}

pub fn needs_x64_jvm(mc_version: &str) -> bool {
    cfg!(all(target_os = "macos", target_arch = "aarch64")) && !uses_arm64_natives(mc_version)
}

/// Report the CPU architecture of a Mach-O binary via `lipo -archs`.
/// Returns "x86_64", "arm64", "universal", or "" when it can't be determined.
pub fn binary_arch(path: &str) -> String {
    let p = std::path::Path::new(path);
    if !p.exists() {
        return String::new();
    }
    match std::process::Command::new("lipo").arg("-archs").arg(p).output() {
        Ok(o) if o.status.success() => {
            let s = String::from_utf8_lossy(&o.stdout).trim().to_string();
            if s.contains("arm64") && s.contains("x86_64") {
                "universal".to_string()
            } else {
                s
            }
        }
        _ => String::new(),
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
    /// Build a recommendation from Mojang's own `javaVersion.majorVersion`
    /// field, falling back to the version-string heuristics when the JSON has
    /// no such field (very old versions).
    pub fn from_version_json(mc_version: &str, required_major: Option<&str>) -> Self {
        match required_major {
            Some(major) if !major.is_empty() => {
                // Java 8 is the floor for anything pre-1.17; Mojang never asks for
                // less, but clamp defensively.
                let rec = major.to_string();
                Self {
                    mc_version: mc_version.to_string(),
                    recommended_java: rec.clone(),
                    minimum_java: rec,
                    explanation: format!(
                        "Mojang declares Java {} for {} (javaVersion.majorVersion)",
                        major, mc_version
                    ),
                }
            }
            _ => Self::get_for_version(mc_version),
        }
    }

    /// Recommend Java for a given Mojang version string.
    /// Returns (recommended_id, minimum_id, explanation).
    ///
    /// Version parsing notes:
    ///  • Old versions use dot-separated semantic-ish strings: "1.7.10", "1.16.5".
    ///  • Post-1.20.5 Mojang adopted a pure-integer scheme ("21w44a", "23w31a", "26.3").
    ///  • SNAPSHOT identifiers ("20w14infinite") must not match release branches.
    pub fn get_for_version(mc_version: &str) -> Self {
        // Strip pre-release suffixes that would confuse naive dot-split parsing
        // (e.g. "1.8.9-pre4" → "1.8.9").
        let clean = mc_version
            .split('-')
            .next()
            .unwrap_or(mc_version)
            .trim();
        let recommended: String;
        let minimum: String;
        let explanation: String;
        if clean.starts_with("26.") || clean.starts_with("25.") || clean.starts_with("24.")
        {
            recommended = "21".to_string();
            minimum = "17".to_string();
            explanation = "Minecraft 24+ requires Java 17; Java 21 recommended".to_string();
        } else if clean.starts_with("23.") || clean.starts_with("22.") || clean.starts_with("21.") || clean.starts_with("1.21") {
            recommended = "21".to_string();
            minimum = "17".to_string();
            explanation = "Minecraft 1.21 / 21.x+ works best on Java 21 (Java 17 minimum)".to_string();
        } else if clean.starts_with("1.20") || clean.starts_with("1.19") {
            recommended = "17".to_string();
            minimum = "17".to_string();
            explanation = "Minecraft 1.19+ requires Java 17".to_string();
        } else if clean.starts_with("1.18.2") || clean.starts_with("1.18.1") || clean.starts_with("1.18") {
            recommended = "17".to_string();
            minimum = "17".to_string();
            explanation = "Minecraft 1.18.2+ requires Java 17".to_string();
        } else if clean.starts_with("1.17") {
            recommended = "17".to_string();
            minimum = "16".to_string();
            explanation = "Minecraft 1.17 requires Java 16; Java 17 recommended".to_string();
        } else if clean.starts_with("1.16")
            || clean.starts_with("1.15")
            || clean.starts_with("1.14")
            || clean.starts_with("1.13")
        {
            recommended = "8".to_string();
            minimum = "8".to_string();
            explanation = "Minecraft 1.13 - 1.16.5 runs on Java 8".to_string();
        } else if clean.starts_with("1.12")
            || clean.starts_with("1.11")
            || clean.starts_with("1.10")
            || clean.starts_with("1.9")
            || clean.starts_with("1.8")
            || clean.starts_with("1.7")
        {
            recommended = "8".to_string();
            minimum = "8".to_string();
            explanation = "Legacy Minecraft before 1.17 needs Java 8".to_string();
        } else {
            // Fallback for anything unrecognized: assume modern (Java 17 min, 21 rec).
            recommended = "21".to_string();
            minimum = "17".to_string();
            explanation = "Unrecognized version string — defaulting to Java 21".to_string();
        }
        Self {
            mc_version: mc_version.to_string(),
            recommended_java: recommended.clone(),
            minimum_java: minimum,
            explanation,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn jv(version: &str) -> JavaVersion {
        JavaVersion::new(
            String::new(),
            String::new(),
            version.to_string(),
            String::new(),
            String::new(),
            true,
        )
    }

    #[test]
    fn matches_release_handles_legacy_1x_naming() {
        // Java 8 reports "1.8.0_504" but the UI asks for "8".
        assert!(jv("1.8.0_504").matches_release("8"));
        assert!(jv("1.8.0_504").matches_release("1.8"));
        assert!(jv("17.0.21").matches_release("17"));
        assert!(jv("21.0.13").matches_release("21"));
        assert!(jv("25").matches_release("25"));
    }

    #[test]
    fn matches_release_rejects_wrong_generation() {
        // The bug that shipped: Java 8 got used for 1.18.2 / 1.20.1.
        assert!(!jv("1.8.0_504").matches_release("17"));
        assert!(!jv("1.8.0_504").matches_release("21"));
        assert!(!jv("21.0.13").matches_release("25"));
        assert!(!jv("17.0.21").matches_release("1.7"));
        assert!(!jv("").matches_release("8"));
    }

    #[test]
    fn arm64_natives_start_at_1_19() {
        assert!(!uses_arm64_natives("1.5.2"));
        assert!(!uses_arm64_natives("1.7.10"));
        assert!(!uses_arm64_natives("1.12.2"));
        assert!(!uses_arm64_natives("1.18.2"));
        assert!(uses_arm64_natives("1.19"));
        assert!(uses_arm64_natives("1.20.1"));
        assert!(uses_arm64_natives("26.3"));
    }

    #[test]
    fn json_java_version_beats_heuristics() {
        // 26.3 declares Java 25 — heuristics alone would say 21 and crash.
        let rec = JavaRecommendation::from_version_json("26.3", Some("25"));
        assert_eq!(rec.recommended_java, "25");
        // 1.16.5 declares Java 8 (heuristics used to claim 16).
        let rec = JavaRecommendation::from_version_json("1.16.5", Some("8"));
        assert_eq!(rec.recommended_java, "8");
        // No declaration (very old JSON) → fall back to string rules.
        let rec = JavaRecommendation::from_version_json("1.7.10", None);
        assert_eq!(rec.recommended_java, "8");
    }

    #[test]
    fn major_of_parses_both_naming_schemes() {
        assert_eq!(major_of("17.0.21"), Some(17));
        assert_eq!(major_of("21"), Some(21));
        assert_eq!(major_of("1.8.0_504"), Some(8));
        assert_eq!(major_of("1.7.0"), Some(7));
        assert_eq!(major_of("26.0.1"), Some(26));
        assert_eq!(major_of(""), None);
        assert_eq!(major_of("weird"), None);
    }

    #[test]
    fn satisfies_requirement_never_accepts_older_jdk() {
        // 这条就是 UnsupportedClassVersionError 的守门人
        assert!(!satisfies_requirement("1.8.0_504", "17"), "Java 8 must not satisfy Java 17");
        assert!(satisfies_requirement("17.0.21", "17"));
        assert!(satisfies_requirement("21.0.13", "17"), "newer JDK is fine");
        assert!(satisfies_requirement("1.8.0_504", "8"));
        assert!(!satisfies_requirement("8.0.432", "17"));
        assert!(satisfies_requirement("25.0.1", "25"));
        assert!(!satisfies_requirement("21.0.13", "25"), "26.x needs Java 25");
    }

}
