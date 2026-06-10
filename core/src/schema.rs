use serde_json::Value;

/// Load the built-in schema at compile time via include_str.
/// Path is relative to this file's directory (core/src/).
pub fn load_schema() -> Value {
    let data = include_str!("../../schema/template_schema.json");
    serde_json::from_str(data).expect("Built-in schema is invalid JSON")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_schema_loads() {
        let s = load_schema();
        assert_eq!(s["title"], "WorkflowTemplate");
    }
}
