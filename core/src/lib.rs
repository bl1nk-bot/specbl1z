pub mod parser;
pub mod renderer;
pub mod schema;
pub mod validator;

use serde_json::Value;

// Re-export key types for consumers
pub use parser::{parse_template, parse_template_str, TemplateFormat};
pub use renderer::{render_markdown, RenderContext};
pub use schema::load_schema;
pub use validator::validate_template;

/// Load a template JSON from a file path (JSON format only, legacy)
/// For format auto-detection, use `parse_template` instead.
pub fn load_template(path: &str) -> Result<Value, String> {
    let data =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path, e))?;
    serde_json::from_str(&data).map_err(|e| format!("Invalid JSON in {}: {}", path, e))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_schema() {
        let s = load_schema();
        assert_eq!(s["title"], "WorkflowTemplate");
    }
}
