use colored::*;
use jsonschema::{Draft, JSONSchema};
use serde_json::Value;

#[derive(Debug, Clone)]
pub enum ValidationErrorType {
    SchemaCompile(String),
    Validation { path: String, message: String },
}

impl std::fmt::Display for ValidationErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationErrorType::SchemaCompile(e) => write!(f, "Schema compile error: {}", e),
            ValidationErrorType::Validation { path, message } => {
                write!(f, "{}: {}", path.yellow(), message)
            }
        }
    }
}

impl std::error::Error for ValidationErrorType {}

pub fn validate_template(schema: &Value, instance: &Value) -> Result<(), Vec<ValidationErrorType>> {
    // Compile schema
    let compiled = match JSONSchema::options()
        .with_draft(Draft::Draft7)
        .compile(schema)
    {
        Ok(c) => c,
        Err(e) => return Err(vec![ValidationErrorType::SchemaCompile(e.to_string())]),
    };

    // Validate — the iterator returned borrows from `compiled`, so we must
    // consume it before `compiled` is dropped. Scope it.
    let result = compiled.validate(instance);
    let mut errors = Vec::new();
    match result {
        Ok(()) => return Ok(()),
        Err(err_iter) => {
            for err in err_iter {
                let path = err.instance_path.to_string();
                let message = format!("{}", err);
                errors.push(ValidationErrorType::Validation { path, message });
            }
        }
    }
    // `compiled` is dropped here, after the iterator was fully consumed
    Err(errors)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn get_test_schema() -> Value {
        json!({
            "type": "object",
            "required": ["workflow", "output_template"],
            "properties": {
                "workflow": {
                    "type": "object",
                    "required": ["title", "steps"],
                    "properties": {
                        "title": { "type": "string" },
                        "steps": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "required": ["id", "critical", "content"],
                                "properties": {
                                    "id": { "type": "string" },
                                    "critical": { "type": "boolean" },
                                    "content": { "type": "string" }
                                }
                            }
                        }
                    }
                },
                "output_template": {
                    "type": "object",
                    "required": ["format", "content"],
                    "properties": {
                        "format": { "type": "string", "enum": ["markdown", "yaml", "json"] },
                        "content": { "type": "string" }
                    }
                }
            }
        })
    }

    fn get_valid_template() -> Value {
        json!({
            "workflow": {
                "title": "Test Workflow",
                "steps": [
                    { "id": "0", "critical": true, "content": "First step" },
                    { "id": "1", "critical": false, "content": "Second step" }
                ]
            },
            "output_template": {
                "format": "markdown",
                "content": "# Test\nHello, {{name}}!"
            }
        })
    }

    fn get_invalid_template() -> Value {
        json!({
            "workflow": {
                "title": "Invalid",
                "steps": [
                    { "id": "0", "critical": true }
                ]
            },
            "output_template": {
                "format": "unknown",
                "content": "test"
            }
        })
    }

    #[test]
    fn test_valid_template_passes() {
        let schema = get_test_schema();
        let instance = get_valid_template();
        assert!(validate_template(&schema, &instance).is_ok());
    }

    #[test]
    fn test_invalid_template_fails() {
        let schema = get_test_schema();
        let instance = get_invalid_template();
        let result = validate_template(&schema, &instance);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_schema_compile_error_handling() {
        let bad_schema = json!("not an object");
        let instance = get_valid_template();
        let result = validate_template(&bad_schema, &instance);
        assert!(result.is_err());
    }
}
