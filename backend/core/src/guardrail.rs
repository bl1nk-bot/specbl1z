use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use std::path::Path;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuardrailRules {
    pub response_language: String,     // e.g., "th", "en"
    pub doc_language: String,          // e.g., "th", "en"
    pub required_markers: Vec<String>, // e.g., ["[FIX]", "[TODO]"]
    pub memory_topics: Vec<String>,    // Strict: ["LEARN", "WORK", "TOOL", "INTEREST", "PROJECT", "IDENTIFY"]
    pub min_test_coverage: f32,        // e.g., 0.8 (80%)
    pub allow_auto_approval: bool,
    pub forbidden_patterns: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GuardrailPack {
    pub name: String,
    pub description: String,
    pub rules: GuardrailRules,
}

impl Default for GuardrailRules {
    fn default() -> Self {
        Self {
            response_language: "th".to_string(),
            doc_language: "th".to_string(),
            required_markers: vec![
                "[FIX]".to_string(),
                "[TODO]".to_string(),
                "[REF]".to_string(),
                "[NOTE]".to_string(),
            ],
            memory_topics: vec![
                "LEARN".to_string(),
                "WORK".to_string(),
                "TOOL".to_string(),
                "INTEREST".to_string(),
                "PROJECT".to_string(),
                "IDENTIFY".to_string(),
            ],
            min_test_coverage: 0.8,
            allow_auto_approval: false,
            forbidden_patterns: vec!["any".to_string(), "TODO: fixme".to_string()],
        }
    }
}

pub fn get_builtin_packs() -> Vec<GuardrailPack> {
    vec![
        GuardrailPack {
            name: "default".to_string(),
            description: "Standard engineering guardrails with Thai language enforcement".to_string(),
            rules: GuardrailRules::default(),
        },
        GuardrailPack {
            name: "strict".to_string(),
            description: "No auto-approvals, full markers required, 100% test coverage".to_string(),
            rules: GuardrailRules {
                min_test_coverage: 1.0,
                ..GuardrailRules::default()
            },
        },
        GuardrailPack {
            name: "fast".to_string(),
            description: "Minimal gates, allows auto-approval for non-critical changes".to_string(),
            rules: GuardrailRules {
                allow_auto_approval: true,
                min_test_coverage: 0.0,
                ..GuardrailRules::default()
            },
        },
        GuardrailPack {
            name: "security".to_string(),
            description: "Extra focus on security review and pattern rejection".to_string(),
            rules: GuardrailRules {
                forbidden_patterns: vec![
                    "api_key".to_string(),
                    "password".to_string(),
                    "secret".to_string(),
                ],
                ..GuardrailRules::default()
            },
        },
        GuardrailPack {
            name: "migration".to_string(),
            description: "Rules for safe schema migrations and rollback plans".to_string(),
            rules: GuardrailRules {
                required_markers: vec!["[MIGRATION]".to_string(), "[ROLLBACK]".to_string()],
                ..GuardrailRules::default()
            },
        },
    ]
}

pub fn load_active_rules() -> Result<GuardrailRules> {
    let path = Path::new(".omp/state/rules.json");
    if path.exists() {
        let content = fs::read_to_string(path)?;
        let pack: GuardrailPack = serde_json::from_str(&content)?;
        Ok(pack.rules)
    } else {
        // Fallback to default pack
        Ok(GuardrailRules::default())
    }
}

pub fn apply_pack(name: &str) -> Result<GuardrailPack> {
    let packs = get_builtin_packs();
    let pack = packs.into_iter()
        .find(|p| p.name == name)
        .context(format!("Pack '{}' not found", name))?;

    let dir = Path::new(".omp/state");
    if !dir.exists() {
        fs::create_dir_all(dir)?;
    }

    let content = serde_json::to_string_pretty(&pack)?;
    fs::write(dir.join("rules.json"), content)?;

    Ok(pack)
}

pub fn reset_to_default() -> Result<GuardrailPack> {
    apply_pack("default")
}
