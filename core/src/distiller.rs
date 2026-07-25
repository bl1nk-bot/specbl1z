use anyhow::Result;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub quality_score: u32,
    pub is_slop: bool,
    pub word_count: usize,
}

pub struct SkillDistiller;

impl SkillDistiller {
    pub fn new(_script_path: &Path) -> Result<Self> {
        Ok(Self)
    }

    pub fn analyze_file(&self, _file_path: &Path) -> Result<SkillMetadata> {
        Ok(SkillMetadata {
            name: "Disabled".into(),
            description: "Python distiller is disabled".into(),
            tags: vec![],
            quality_score: 0,
            is_slop: true,
            word_count: 0,
        })
    }
}
