use serde_json::{json, to_value, Value};
use toml::Value as TomlValue;

/// Parse a TOML template into the standard JSON structure
/// Expected TOML structure based on the spec:
/// [workflow]
/// title = "..."
/// restart = "..."
/// [workflow.rules]
/// text = """..."""
/// [[workflow.steps]]
/// id = "..."
/// critical = true/false
/// content = """..."""
/// [workflow.loop_restart]
/// text = """..."""
/// [output_template]
/// format = "markdown" | "yaml" | "json"
/// content = """..."""
pub fn parse_toml_template(input: &str) -> Result<Value, String> {
    let toml_value: TomlValue = input
        .parse()
        .map_err(|e| format!("TOML parse error: {}", e))?;

    // Extract workflow table
    let workflow_table = toml_value
        .get("workflow")
        .ok_or_else(|| "Missing [workflow] table".to_string())?;

    // Extract workflow title
    let title = workflow_table
        .get("title")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Extract restart step
    let restart = workflow_table
        .get("restart")
        .and_then(|v| v.as_str())
        .unwrap_or("0")
        .to_string();

    // Extract rules
    let rules = workflow_table
        .get("rules")
        .and_then(|v| v.as_table())
        .and_then(|t| t.get("text"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Extract steps as array of tables
    let steps_array = workflow_table
        .get("steps")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            "workflow.steps must be an array of tables ([[workflow.steps]])".to_string()
        })?;

    let mut steps = Vec::new();
    for (idx, step_val) in steps_array.iter().enumerate() {
        let step_table = step_val
            .as_table()
            .ok_or_else(|| format!("workflow.steps[{}] must be a table", idx))?;

        let id = step_table
            .get("id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("Step {} missing required 'id' field", idx))?
            .to_string();

        let critical = step_table
            .get("critical")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let content = step_table
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| format!("Step {} missing required 'content' field", idx))?
            .to_string();

        let next = step_table
            .get("next")
            .and_then(|v| v.as_str())
            .map(|s| Value::String(s.to_string()))
            .unwrap_or(Value::Null);

        steps.push(json!({
            "id": id,
            "critical": critical,
            "content": content,
            "next": next,
        }));
    }

    // Extract loop_restart
    let loop_restart = workflow_table
        .get("loop_restart")
        .and_then(|v| v.as_table())
        .and_then(|t| t.get("text"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Extract output_template
    let output_table = toml_value
        .get("output_template")
        .ok_or_else(|| "Missing [output_template] table".to_string())?;

    let output_format = output_table
        .get("format")
        .and_then(|v| v.as_str())
        .unwrap_or("markdown");

    let output_content = output_table
        .get("content")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    // Build result JSON
    Ok(json!({
        "workflow": {
            "title": title,
            "restart": restart,
            "rules": rules,
            "steps": steps,
            "loop_restart": loop_restart,
        },
        "output_template": {
            "format": output_format,
            "content": output_content,
        },
        // Optional meta section if present
        "meta": toml_value.get("meta").and_then(|v| to_value(v.clone()).ok())
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_toml() {
        let input = r##"
[workflow]
title = "Test Workflow"
restart = "0"

[workflow.rules]
text = """
- Rule 1
- Rule 2
"""

[[workflow.steps]]
id = "0"
critical = true
content = "Step 0 content"

[[workflow.steps]]
id = "1"
critical = false
content = "Step 1 content"

[workflow.loop_restart]
text = "Restart from Step 0"

[output_template]
format = "markdown"
content = "# {{Project Name}}"
"##;

        let result = parse_toml_template(input).unwrap();
        assert_eq!(result["workflow"]["title"], "Test Workflow");
        assert_eq!(result["workflow"]["restart"], "0");
        assert!(result["workflow"]["rules"]
            .as_str()
            .unwrap()
            .contains("Rule 1"));
        let steps = result["workflow"]["steps"].as_array().unwrap();
        assert_eq!(steps.len(), 2);
        assert_eq!(steps[0]["id"], "0");
        assert!(steps[0]["critical"].as_bool().unwrap());
        assert_eq!(steps[1]["id"], "1");
        assert!(!steps[1]["critical"].as_bool().unwrap());
        assert!(result["workflow"]["loop_restart"]
            .as_str()
            .unwrap()
            .contains("Restart"));
        assert_eq!(result["output_template"]["format"], "markdown");
    }

    #[test]
    fn test_parse_toml_minimal() {
        let input = r##"
[workflow]
title = "Minimal"
restart = "0"

[workflow.rules]
text = "Rules"

[[workflow.steps]]
id = "0"
critical = false
content = "Content"

[workflow.loop_restart]
text = ""

[output_template]
format = "json"
content = "{}"
"##;

        let result = parse_toml_template(input).unwrap();
        assert_eq!(result["workflow"]["title"], "Minimal");
        assert_eq!(result["output_template"]["format"], "json");
    }

    #[test]
    fn test_parse_toml_with_thai() {
        let input = r##"
[workflow]
title = "การจัดการไฟล์ SPEC.md"
restart = "0"

[workflow.rules]
text = """
        ขั้นตอน: ตรวจสอบการทำงาน
"""

[[workflow.steps]]
id = "0"
critical = true
content = """
**Step 0: ตรวจสอบ Git Directory**
- รัน `git rev-parse --is-inside-work-tree`
"""

[workflow.loop_restart]
text = "เริ่มใหม่ที่ Step 0"

[output_template]
format = "markdown"
content = """
# {{Project Name}}
"""
"##;

        let result = parse_toml_template(input).unwrap();
        assert_eq!(result["workflow"]["title"], "การจัดการไฟล์ SPEC.md");
        assert!(result["workflow"]["rules"]
            .as_str()
            .unwrap()
            .contains("ขั้นตอน"));
        assert!(result["workflow"]["steps"][0]["content"]
            .as_str()
            .unwrap()
            .contains("ตรวจสอบ Git"));
    }

    #[test]
    fn test_parse_toml_missing_required() {
        let input = r##"
[workflow]
title = "Broken"

[output_template]
format = "markdown"
content = "test"
"##;
        let result = parse_toml_template(input);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("steps"));
    }

    #[test]
    fn test_parse_toml_with_meta() {
        let input = r##"
[workflow]
title = "With Meta"
restart = "0"

[workflow.rules]
text = "Rules"

[[workflow.steps]]
id = "0"
critical = false
content = "Content"

[workflow.loop_restart]
text = "Restart"

[output_template]
format = "markdown"
content = "Output"

[meta]
version = "1.0.0"
author = "Test Author"
created_at = "2026-05-08T00:00:00Z"
"##;

        let result = parse_toml_template(input).unwrap();
        assert!(result.get("meta").is_some());
        let meta = result["meta"].as_object().unwrap();
        assert_eq!(meta.get("version").and_then(|v| v.as_str()), Some("1.0.0"));
        assert_eq!(
            meta.get("author").and_then(|v| v.as_str()),
            Some("Test Author")
        );
    }
}
