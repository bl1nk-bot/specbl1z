use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use serde_json::{json, Value};
use std::io::Cursor;

/// Parse a Markdown document with embedded XML-like tags
/// Supports: <workflow>, <rules>, <step>, <loop_restart>, <output_template>
/// and their closing tags
pub fn parse_markdown_template(input: &str) -> Result<Value, String> {
    // Pre-process: handle self-closing tags and escape sequences
    let processed = preprocess_markdown(input)?;

    let mut reader = Reader::from_reader(Cursor::new(processed.as_bytes()));
    reader.config_mut().trim_text(false);

    let mut buf = Vec::new();
    let mut current_tag = String::new();
    let mut in_rules = false;
    let mut in_step = false;
    let mut in_loop_restart = false;
    let mut in_output_template = false;

    let mut workflow_attrs = json!({});
    let mut rules_content = String::new();
    let mut steps = Vec::new();
    let mut current_step = None;
    let mut loop_restart_content = String::new();
    let mut output_template_content = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                current_tag = tag.to_string();

                match current_tag.as_ref() {
                    "workflow" => {
                        workflow_attrs = extract_attributes(e)?;
                    }
                    "rules" => {
                        in_rules = true;
                        rules_content.clear();
                    }
                    "step" => {
                        in_step = true;
                        let step_attrs = extract_attributes(e)?;
                        let id = step_attrs
                            .get("id")
                            .and_then(|v| v.as_str())
                            .ok_or_else(|| "Step requires 'id' attribute".to_string())?
                            .to_string();
                        let critical = if let Some(v) = step_attrs.get("critical") {
                            match v {
                                Value::String(s) => s.eq_ignore_ascii_case("true") || s == "1",
                                Value::Bool(b) => *b,
                                _ => false,
                            }
                        } else {
                            false
                        };
                        current_step = Some(json!({
                            "id": id,
                            "critical": critical,
                            "content": String::new(),
                        }));
                    }
                    "loop_restart" => {
                        in_loop_restart = true;
                        loop_restart_content.clear();
                    }
                    "output_template" => {
                        in_output_template = true;
                        output_template_content.clear();
                    }
                    _ => {}
                }
            }
            Ok(Event::End(ref e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).into_owned();
                match tag.as_ref() {
                    "workflow" => break, // finished parsing root
                    "rules" => {
                        in_rules = false;
                    }
                    "step" => {
                        in_step = false;
                        if let Some(step) = current_step.take() {
                            steps.push(step);
                        }
                    }
                    "loop_restart" => {
                        in_loop_restart = false;
                    }
                    "output_template" => {
                        in_output_template = false;
                    }
                    _ => {}
                }
                current_tag.clear();
            }
            Ok(Event::Text(e)) => {
                // Unescape common XML entities
                let raw = std::str::from_utf8(e.as_ref()).unwrap_or_default();
                let text = raw
                    .replace("&lt;", "<")
                    .replace("&gt;", ">")
                    .replace("&amp;", "&")
                    .replace("&quot;", "\"")
                    .replace("&apos;", "'");
                if in_rules {
                    rules_content.push_str(&text);
                    rules_content.push('\n');
                } else if in_step {
                    if let Some(ref mut step) = current_step {
                        step["content"] = Value::String(
                            step["content"].as_str().unwrap_or("").to_string() + &text,
                        );
                    }
                } else if in_loop_restart {
                    loop_restart_content.push_str(&text);
                    loop_restart_content.push('\n');
                } else if in_output_template {
                    output_template_content.push_str(&text);
                    output_template_content.push('\n');
                }
            }
            Ok(Event::CData(e)) => {
                let text = std::str::from_utf8(&e).unwrap_or_default().to_string();
                if in_rules {
                    rules_content.push_str(&text);
                } else if in_step {
                    if let Some(ref mut step) = current_step {
                        step["content"] = Value::String(
                            step["content"].as_str().unwrap_or("").to_string() + &text,
                        );
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(format!("XML parse error: {}", e)),
            _ => {}
        }
        buf.clear();
    }

    // Build the final JSON structure matching the schema
    let mut result = json!({
        "workflow": {
            "title": "",
            "restart": "",
            "rules": rules_content.trim().to_string(),
            "steps": json!(steps),
            "loop_restart": loop_restart_content.trim().to_string(),
        },
        "output_template": {
            "format": "markdown",
            "content": output_template_content.trim().to_string(),
        }
    });

    // Merge workflow attributes (title, restart override defaults)
    if let Some(title) = workflow_attrs.get("title") {
        if let Some(title_str) = title.as_str() {
            result["workflow"]["title"] = Value::String(title_str.to_string());
        }
    }
    if let Some(restart) = workflow_attrs.get("restart") {
        if let Some(restart_str) = restart.as_str() {
            result["workflow"]["restart"] = Value::String(restart_str.to_string());
        }
    }

    Ok(result)
}

/// Pre-process Markdown+XML to handle edge cases:
/// - Escape raw '<' and '&' that are not part of allowed tags or entities
/// - Handle unterminated allowed tags by auto-closing them
fn preprocess_markdown(input: &str) -> Result<String, String> {
    // Allowed XML tags for workflow
    const ALLOWED_TAGS: [&str; 5] = [
        "workflow",
        "rules",
        "step",
        "loop_restart",
        "output_template",
    ];

    let mut output = String::with_capacity(input.len() * 2);
    let bytes = input.as_bytes();
    let mut i = 0;
    let mut stack: Vec<&str> = Vec::new();

    while i < bytes.len() {
        if bytes[i] == b'<' {
            // Peek ahead to determine if this is an allowed tag
            let tag_start = i;
            let mut j = i + 1;
            let is_closing = bytes.get(j) == Some(&b'/');
            if is_closing {
                j += 1;
            }
            let name_start = j;
            // Read tag name (ASCII alnum, dash, underscore)
            while j < bytes.len()
                && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'-' || bytes[j] == b'_')
            {
                j += 1;
            }
            let name_slice = if name_start < j {
                &bytes[name_start..j]
            } else {
                b""
            };
            let tag_name = if name_slice.is_empty() {
                // Not a tag, treat as literal '<' below
                ""
            } else {
                unsafe { std::str::from_utf8_unchecked(name_slice) }
            };

            // Determine if allowed
            let is_allowed = ALLOWED_TAGS.contains(&tag_name);

            if is_allowed {
                // Find closing '>'
                while j < bytes.len() && bytes[j] != b'>' {
                    j += 1;
                }
                if j < bytes.len() {
                    // Copy full tag verbatim
                    output.push_str(&input[tag_start..=j]);
                    // Update stack
                    if is_closing {
                        if stack.last() == Some(&tag_name) {
                            stack.pop();
                        }
                    } else {
                        // Check self-closing: look for '/' before '>' ignoring whitespace
                        let mut k = j - 1;
                        while k > name_start && bytes[k].is_ascii_whitespace() {
                            k -= 1;
                        }
                        if k <= name_start || bytes[k] != b'/' {
                            stack.push(tag_name);
                        }
                    }
                    i = j + 1;
                    continue;
                } else {
                    // No '>', copy what we have and advance
                    output.push_str(&input[tag_start..j]);
                    i = j;
                    continue;
                }
            } else {
                // Not an allowed tag — escape the '<'
                output.push_str("&lt;");
                i = tag_start + 1;
                // The remaining characters will be handled by the main loop
                continue;
            }
        } else if bytes[i] == b'&' {
            // Potential entity or bare ampersand
            let mut k = i + 1;
            while k < bytes.len()
                && k < i + 10
                && (bytes[k].is_ascii_alphanumeric() || bytes[k] == b'#')
            {
                k += 1;
            }
            if k < bytes.len() && bytes[k] == b';' {
                let entity = unsafe { std::str::from_utf8_unchecked(&bytes[i + 1..k]) };
                match entity {
                    "lt" | "gt" | "amp" | "quot" | "apos" => {
                        output.push_str(&input[i..=k]);
                        i = k + 1;
                        continue;
                    }
                    _ => {
                        if entity.starts_with('#') {
                            output.push_str(&input[i..=k]);
                            i = k + 1;
                            continue;
                        }
                    }
                }
            }
            // Bare '&' — escape
            output.push_str("&amp;");
            i += 1;
            continue;
        } else {
            // Push the next UTF-8 character (handles non-ASCII)
            let ch = input[i..].chars().next().unwrap();
            output.push(ch);
            i += ch.len_utf8();
        }
    }

    // Append missing closing tags for any remaining stack items
    for tag in stack.iter().rev() {
        output.push_str(&format!("</{}>", tag));
    }

    Ok(output)
}

/// Extract attributes from a quick-xml BytesStart element into a JSON object
fn extract_attributes(e: &BytesStart) -> Result<Value, String> {
    let mut attrs = json!({});
    for attr in e.attributes() {
        let attr = attr.map_err(|e| format!("Attribute parse error: {}", e))?;
        let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
        let value = String::from_utf8_lossy(attr.value.as_ref()).into_owned();
        attrs[key] = Value::String(value);
    }
    Ok(attrs)
}

/// Serialize a template JSON Value back into Markdown+XML format
pub fn serialize_markdown_template(value: &Value) -> Result<String, String> {
    let mut output = String::new();

    let workflow = value
        .get("workflow")
        .ok_or_else(|| "Missing 'workflow' object".to_string())?;

    let title = workflow.get("title").and_then(|v| v.as_str()).unwrap_or("");
    let restart = workflow
        .get("restart")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    output.push_str(&format!(
        "<workflow title=\"{}\" restart=\"{}\">\n",
        title, restart
    ));

    if let Some(rules) = workflow.get("rules").and_then(|v| v.as_str()) {
        output.push_str("<rules>\n");
        output.push_str(rules);
        output.push_str("\n</rules>\n");
    }

    if let Some(steps) = workflow.get("steps").and_then(|v| v.as_array()) {
        for step in steps {
            let id = step.get("id").and_then(|v| v.as_str()).unwrap_or("");
            let critical = step
                .get("critical")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let content = step.get("content").and_then(|v| v.as_str()).unwrap_or("");

            output.push_str(&format!("<step id=\"{}\" critical=\"{}\">\n", id, critical));
            output.push_str(content);
            output.push_str("\n</step>\n");
        }
    }

    if let Some(loop_restart) = workflow.get("loop_restart").and_then(|v| v.as_str()) {
        output.push_str("<loop_restart>\n");
        output.push_str(loop_restart);
        output.push_str("\n</loop_restart>\n");
    }

    if let Some(output_template) = value.get("output_template") {
        let format = output_template
            .get("format")
            .and_then(|v| v.as_str())
            .unwrap_or("markdown");
        let content = output_template
            .get("content")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        output.push_str(&format!("<output_template format=\"{}\">\n", format));
        output.push_str(content);
        output.push_str("\n</output_template>\n");
    }

    output.push_str("</workflow>\n");

    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_markdown_workflow() {
        let input = r#"<workflow title="Test" restart="0">
<rules>
These are the rules.
</rules>
<step id="0" critical="true">
**Step 0: Check**
- Do this
</step>
<output_template>
# {{Project Name}}
</output_template>
</workflow>"#;

        let result = parse_markdown_template(input).unwrap();
        assert_eq!(result["workflow"]["title"], "Test");
        assert_eq!(result["workflow"]["restart"], "0");
        assert!(result["workflow"]["rules"]
            .as_str()
            .unwrap()
            .contains("rules"));
        assert_eq!(result["workflow"]["steps"].as_array().unwrap().len(), 1);
        assert_eq!(result["workflow"]["steps"][0]["id"], "0");
        assert!(result["workflow"]["steps"][0]["critical"]
            .as_bool()
            .unwrap());
        assert!(result["workflow"]["steps"][0]["content"]
            .as_str()
            .unwrap()
            .contains("Check"));
        assert!(result["output_template"]["content"]
            .as_str()
            .unwrap()
            .contains("Project Name"));
    }

    #[test]
    fn test_parse_multiple_steps() {
        let input = r#"<workflow title="Multi-Step" restart="0">
<step id="1" critical="false">Step 1 content</step>
<step id="2" critical="true">Step 2 content</step>
<step id="3">Step 3 content</step>
</workflow>"#;

        let result = parse_markdown_template(input).unwrap();
        let steps = result["workflow"]["steps"].as_array().unwrap();
        assert_eq!(steps.len(), 3);
        assert_eq!(steps[0]["id"], "1");
        assert!(!steps[0]["critical"].as_bool().unwrap());
        assert_eq!(steps[1]["id"], "2");
        assert!(steps[1]["critical"].as_bool().unwrap());
        assert_eq!(steps[2]["id"], "3");
        // Default critical should be false
        assert!(!steps[2]["critical"].as_bool().unwrap());
    }

    #[test]
    fn test_parse_with_special_characters() {
        let input = r#"<workflow title="测试" restart="0">
<rules>
- Rule with "quotes" and <angle brackets>
</rules>
<step id="0" critical="true">
Content with & ampersand and Thai: สวัสดี
</step>
</workflow>"#;

        let result = parse_markdown_template(input).unwrap();
        assert_eq!(result["workflow"]["title"], "测试");
        assert!(result["workflow"]["rules"]
            .as_str()
            .unwrap()
            .contains("quotes"));
        assert!(result["workflow"]["steps"][0]["content"]
            .as_str()
            .unwrap()
            .contains("สวัสดี"));
    }

    #[test]
    fn test_parse_unterminated_tag_auto_close() {
        // Without closing </workflow>
        let input = r#"<workflow title="Incomplete">
<step id="0">Content
</step>"#;

        let result = parse_markdown_template(input);
        assert!(result.is_ok());
        let val = result.unwrap();
        assert_eq!(val["workflow"]["title"], "Incomplete");
        assert_eq!(val["workflow"]["steps"].as_array().unwrap().len(), 1);
    }

    #[test]
    fn test_parse_with_entities() {
        let input = r#"<workflow title="Entity Test">
<rules>
&lt;script&gt;alert()&lt;/script&gt;
</rules>
<step id="0">Content &amp; more</step>
</workflow>"#;

        let result = parse_markdown_template(input).unwrap();
        let rules = result["workflow"]["rules"].as_str().unwrap();
        assert!(rules.contains("<script>"));
        assert!(rules.contains("</script>"));
        let content = result["workflow"]["steps"][0]["content"].as_str().unwrap();
        assert!(content.contains("&"));
    }

    #[test]
    fn test_serialize_markdown() {
        let input = json!({
            "workflow": {
                "title": "Test Serial",
                "restart": "1",
                "rules": "Rule 1",
                "steps": [
                    { "id": "0", "critical": true, "content": "Step 0" }
                ],
                "loop_restart": "Restart info"
            },
            "output_template": {
                "format": "markdown",
                "content": "# Output"
            }
        });

        let serialized = serialize_markdown_template(&input).unwrap();
        assert!(serialized.contains("<workflow title=\"Test Serial\" restart=\"1\">"));
        assert!(serialized.contains("<rules>\nRule 1\n</rules>"));
        assert!(serialized.contains("<step id=\"0\" critical=\"true\">\nStep 0\n</step>"));
        assert!(serialized
            .contains("<output_template format=\"markdown\">\n# Output\n</output_template>"));
    }
}
