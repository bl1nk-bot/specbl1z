pub mod parser;
pub mod renderer;
pub mod schema;
pub mod sense;
pub mod validator;

pub mod bl1nk {
    include!(concat!(env!("OUT_DIR"), "/bl1nk.rs"));
}

pub mod db;
pub mod distiller;
pub mod memory;
pub mod models;
pub mod rules_engine;
pub mod sync;
pub mod task_delegator;
use serde_json::{Map, Value};

// Re-export key types for consumers
pub use parser::markdown::serialize_markdown_template;
pub use parser::{parse_template, parse_template_str, TemplateFormat};
pub use renderer::{render_markdown, RenderContext};
pub use schema::load_schema;
pub use validator::validate_template;

/// Recursively removes null values from a serde_json::Value.
fn filter_null_from_json_value(value: Value) -> Value {
    match value {
        Value::Object(map) => {
            let filtered_map: Map<String, Value> = map
                .into_iter()
                .filter(|(_, v)| !v.is_null()) // Filter out nulls
                .map(|(k, v)| (k, filter_null_from_json_value(v))) // Recurse for nested values
                .collect();
            Value::Object(filtered_map)
        }
        Value::Array(array) => {
            let filtered_array: Vec<Value> = array
                .into_iter()
                .map(filter_null_from_json_value) // Recurse for nested values
                .collect();
            Value::Array(filtered_array)
        }
        _ => value, // For all other types (String, Number, Bool), return as is
    }
}

/// Load a template JSON from a file path (JSON format only, legacy)
/// For format auto-detection, use `parse_template` instead.
pub fn load_template(path: &str) -> Result<Value, String> {
    let data =
        std::fs::read_to_string(path).map_err(|e| format!("Failed to read {}: {}", path, e))?;
    serde_json::from_str(&data).map_err(|e| format!("Invalid JSON in {}: {}", path, e))
}

/// Serialize a template Value into the specified format
pub fn serialize_template(value: &Value, format: TemplateFormat) -> Result<String, String> {
    match format {
        TemplateFormat::Json => {
            serde_json::to_string_pretty(value).map_err(|e| format!("JSON error: {}", e))
        }
        TemplateFormat::Toml => {
            // Filter out null values before serializing to TOML
            let filtered_value = filter_null_from_json_value(value.clone());
            toml::to_string_pretty(&filtered_value).map_err(|e| format!("TOML error: {}", e))
        }
        TemplateFormat::Markdown => serialize_markdown_template(value),
    }
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
