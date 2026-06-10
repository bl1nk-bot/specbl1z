pub mod markdown;
pub mod toml;

use serde_json::Value;
use std::path::Path;

/// Supported template formats
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateFormat {
    Json,
    Markdown,
    Toml,
}

impl TemplateFormat {
    /// Detect format from file extension
    pub fn from_path(path: &Path) -> Option<Self> {
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| match ext.to_lowercase().as_str() {
                "json" => TemplateFormat::Json,
                "md" | "markdown" => TemplateFormat::Markdown,
                "toml" => TemplateFormat::Toml,
                _ => TemplateFormat::Json, // default to JSON
            })
    }
}

/// Parse a template file into a JSON Value structure
/// Auto-detects format from file extension if format is None
pub fn parse_template(path: &str, format: Option<TemplateFormat>) -> Result<Value, String> {
    let data =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path, e))?;

    let fmt = match format {
        Some(f) => f,
        None => TemplateFormat::from_path(Path::new(path))
            .ok_or_else(|| format!("Cannot detect format from path: {}", path))?,
    };

    match fmt {
        TemplateFormat::Json => {
            serde_json::from_str(&data).map_err(|e| format!("Invalid JSON: {}", e))
        }
        TemplateFormat::Markdown => markdown::parse_markdown_template(&data),
        TemplateFormat::Toml => toml::parse_toml_template(&data),
    }
}

/// Parse a template string with explicit format
pub fn parse_template_str(data: &str, format: TemplateFormat) -> Result<Value, String> {
    match format {
        TemplateFormat::Json => {
            serde_json::from_str(data).map_err(|e| format!("Invalid JSON: {}", e))
        }
        TemplateFormat::Markdown => markdown::parse_markdown_template(data),
        TemplateFormat::Toml => toml::parse_toml_template(data),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_detection() {
        assert_eq!(
            TemplateFormat::from_path(Path::new("test.json")),
            Some(TemplateFormat::Json)
        );
        assert_eq!(
            TemplateFormat::from_path(Path::new("test.md")),
            Some(TemplateFormat::Markdown)
        );
        assert_eq!(
            TemplateFormat::from_path(Path::new("test.toml")),
            Some(TemplateFormat::Toml)
        );
        assert_eq!(
            TemplateFormat::from_path(Path::new("test.txt")),
            Some(TemplateFormat::Json)
        );
    }
}
