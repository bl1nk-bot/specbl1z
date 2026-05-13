use anyhow::{Context, Result};
use pyo3::prelude::*;
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

pub struct SkillDistiller {
    script_code: String,
}

impl SkillDistiller {
    pub fn new(script_path: &Path) -> Result<Self> {
        let script_code = std::fs::read_to_string(script_path)
            .context(format!("Failed to read python distiller logic from {:?}", script_path))?;
        Ok(Self { script_code })
    }

    pub fn analyze_file(&self, file_path: &Path) -> Result<SkillMetadata> {
        let content = std::fs::read_to_string(file_path)?;
        
        let json_str: String = Python::with_gil(|py| -> PyResult<String> {
            use std::ffi::CString;
            let code = CString::new(self.script_code.clone()).unwrap();
            let file_name = CString::new("distiller_logic.py").unwrap();
            let module_name = CString::new("distiller_logic").unwrap();
            
            // Load the python module dynamically
            let module = PyModule::from_code(py, &code, &file_name, &module_name)?;
            
            // Call the analyze_skill function
            let result: String = module.getattr("analyze_skill")?
                                       .call1((content,))?
                                       .extract()?;
            Ok(result)
        }).context("Failed to execute Python logic via PyO3")?;

        let meta: SkillMetadata = serde_json::from_str(&json_str)
            .context("Failed to parse JSON returned from Python distiller")?;
            
        Ok(meta)
    }
}
