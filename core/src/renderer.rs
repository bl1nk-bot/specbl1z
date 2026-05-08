use regex::Regex;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct RenderContext {
    variables: HashMap<String, String>,
}

impl RenderContext {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_var(mut self, key: &str, value: &str) -> Self {
        self.variables.insert(key.to_string(), value.to_string());
        self
    }
    pub fn with_vars(mut self, vars: &[(&str, &str)]) -> Self {
        for (k, v) in vars {
            self.variables.insert(k.to_string(), v.to_string());
        }
        self
    }
    pub fn get(&self, key: &str) -> Option<&str> {
        self.variables.get(key).map(|s| s.as_str())
    }
}

pub fn render_markdown(template: &str, ctx: &RenderContext) -> Result<String, String> {
    // Phase 1: resolve optional blocks
    let mut out = resolve_optional_blocks(template, ctx)?;
    // Phase 2: resolve each blocks
    out = resolve_each_blocks(&out, ctx)?;
    // Phase 3: required placeholders and helpers
    let re_req = Regex::new(r"\{\{([^?#/][^}]*)\}\}").map_err(|e| e.to_string())?;
    let mut repl: Vec<(usize, usize, String)> = Vec::new();
    for cap in re_req.captures_iter(&out) {
        let full = cap.get(0).unwrap();
        let token = cap.get(1).unwrap().as_str().trim();

        let parts: Vec<&str> = token.split_whitespace().collect();
        let val = if parts.len() > 1 && ["uppercase", "lowercase", "trim"].contains(&parts[0]) {
            // Helper call: helper_name arg
            let helper = parts[0];
            let key = parts[1];
            let base_val = ctx
                .get(key)
                .ok_or_else(|| format!("Missing variable for helper '{}': '{}'", helper, key))?;

            match helper {
                "uppercase" => base_val.to_uppercase(),
                "lowercase" => base_val.to_lowercase(),
                "trim" => base_val.trim().to_string(),
                _ => unreachable!(),
            }
        } else {
            ctx.get(token)
                .ok_or_else(|| format!("Missing required variable: '{}'", token))?
                .to_string()
        };
        repl.push((full.start(), full.end(), val));
    }
    for (s, e, v) in repl.into_iter().rev() {
        out.replace_range(s..e, &v);
    }
    Ok(out)
}

fn resolve_optional_blocks(text: &str, ctx: &RenderContext) -> Result<String, String> {
    // {{?key}}...{{/key}}  — closing tag is any {{/...}}, key match not enforced
    let re = Regex::new(r"(?s)\{\{\?([^}]+)\}\}(.*?)\{\{\/[^}]*\}\}").map_err(|e| e.to_string())?;
    let mut result = String::new();
    let mut last = 0;
    for cap in re.captures_iter(text) {
        let full = cap.get(0).unwrap();
        result.push_str(&text[last..full.start()]);
        let key = cap.get(1).unwrap().as_str().trim();
        let inner = cap.get(2).unwrap().as_str();
        if ctx.get(key).is_some() {
            let rendered = render_markdown(inner, ctx)?;
            result.push_str(&rendered);
        }
        last = full.end();
    }
    result.push_str(&text[last..]);
    Ok(result)
}
fn resolve_each_blocks(text: &str, ctx: &RenderContext) -> Result<String, String> {
    // Scan for {{#...}}...{{/...}} blocks, handling nesting
    let mut result = String::new();
    let mut pos = 0;

    while let Some(open_rel) = text[pos..].find("{{#") {
        let abs_open = pos + open_rel;
        result.push_str(&text[pos..abs_open]);

        // Parse opening tag content (between {{# and }})
        let after_hash = abs_open + 3;
        let close_tag_rel = match text[after_hash..].find("}}") {
            Some(idx) => idx,
            None => {
                pos = abs_open + 3;
                continue;
            }
        };
        let token = text[after_hash..after_hash + close_tag_rel].trim();

        // Determine variable name (list to iterate) and expected closing tag
        let (var_name, expected_closing) = if token.starts_with("each ") {
            let var = token["each".len()..].trim();
            (var, "each")
        } else {
            (token, token)
        };

        let open_tag_end = after_hash + close_tag_rel + 2; // after "}}"

        // Find matching closing tag with proper nesting
        let mut stack: Vec<&str> = vec![expected_closing];
        let mut scan = open_tag_end;
        let body_start = open_tag_end;
        let mut body_end = 0;
        let mut closing_end: Option<usize> = None;

        while scan < text.len() && !stack.is_empty() {
            let next_open = text[scan..].find("{{#");
            let next_close = text[scan..].find("{{/");
            let (next_pos, is_open) = match (next_open, next_close) {
                (Some(o), Some(c)) => {
                    if o < c {
                        (scan + o, true)
                    } else {
                        (scan + c, false)
                    }
                }
                (Some(o), None) => (scan + o, true),
                (None, Some(c)) => (scan + c, false),
                (None, None) => break,
            };

            if is_open {
                // Parse nested opening token
                let after_hash2 = next_pos + 3;
                let close_offset = match text[after_hash2..].find("}}") {
                    Some(idx) => idx,
                    None => {
                        scan = next_pos + 3;
                        continue;
                    }
                };
                let tok = text[after_hash2..after_hash2 + close_offset].trim();
                let closing = if tok.starts_with("each ") {
                    "each"
                } else {
                    tok
                };
                stack.push(closing);
                scan = after_hash2 + close_offset + 2;
            } else {
                // Parse closing tag
                let after_slash = next_pos + 3;
                let close_offset = match text[after_slash..].find("}}") {
                    Some(idx) => idx,
                    None => {
                        scan = next_pos + 3;
                        continue;
                    }
                };
                let closing_name = text[after_slash..after_slash + close_offset].trim();
                if let Some(top) = stack.last() {
                    if *top == closing_name {
                        stack.pop();
                        if stack.is_empty() {
                            body_end = next_pos;
                            closing_end = Some(after_slash + close_offset + 2);
                            break;
                        }
                    }
                }
                scan = after_slash + close_offset + 2;
            }
        }

        let closing_end = match closing_end {
            Some(ce) => ce,
            None => {
                // No matching close; skip opening tag
                pos = open_tag_end;
                continue;
            }
        };

        let body = &text[body_start..body_end];

        // Iterate over list
        let items_json = ctx
            .get(var_name)
            .ok_or_else(|| format!("Missing list variable for each: '{}'", var_name))?;
        let items: Value = serde_json::from_str(items_json)
            .map_err(|e| format!("Invalid JSON array for '{}': {}", var_name, e))?;
        let arr = items
            .as_array()
            .ok_or_else(|| format!("'{}' must be a JSON array", var_name))?;

        for (idx, item) in arr.iter().enumerate() {
            let mut item_ctx = RenderContext::new();
            if let Some(obj) = item.as_object() {
                for (k, v) in obj {
                    let val_str = match v {
                        Value::String(s) => s.as_str(),
                        _ => &v.to_string(),
                    };
                    item_ctx = item_ctx.with_var(k, val_str);
                }
            }
            item_ctx = item_ctx.with_var("item", &item.to_string());

            let mut rendered_body = body.replace("{{index}}", &idx.to_string());
            rendered_body = render_markdown(&rendered_body, &item_ctx)?;
            result.push_str(&rendered_body);
        }

        pos = closing_end;
    }

    result.push_str(&text[pos..]);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_required() {
        let tpl = "Hello, {{name}}!";
        let ctx = RenderContext::new().with_var("name", "World");
        assert_eq!(render_markdown(tpl, &ctx).unwrap(), "Hello, World!");
    }

    #[test]
    fn test_multiple_placeholders() {
        let tpl = "{{greeting}}, {{name}}! Today is {{day}}.";
        let ctx = RenderContext::new()
            .with_var("greeting", "Hi")
            .with_var("name", "Alice")
            .with_var("day", "Monday");
        assert_eq!(
            render_markdown(tpl, &ctx).unwrap(),
            "Hi, Alice! Today is Monday."
        );
    }

    #[test]
    fn test_missing_required_returns_error() {
        let tpl = "Hello, {{name}}!";
        let ctx = RenderContext::new();
        let res = render_markdown(tpl, &ctx);
        assert!(res.is_err());
        assert!(res.unwrap_err().contains("Missing required variable"));
    }

    #[test]
    fn test_optional_present() {
        let tpl = "Start{{?opt}} and {{opt}}{{/opt}} End";
        let ctx = RenderContext::new().with_var("opt", "YES");
        assert_eq!(render_markdown(tpl, &ctx).unwrap(), "Start and YES End");
    }

    #[test]
    fn test_optional_absent() {
        let tpl = "Start{{?opt}} and {{opt}}{{/opt}} End";
        let ctx = RenderContext::new();
        assert_eq!(render_markdown(tpl, &ctx).unwrap(), "Start End");
    }

    #[test]
    fn test_optional_nested_required() {
        let tpl = "{{?show}}Hello, {{name}}!{{/show}}";
        let ctx = RenderContext::new().with_var("name", "Bob");
        // show is not set, block omitted
        assert_eq!(render_markdown(tpl, &ctx).unwrap(), "");
    }

    #[test]
    fn test_each_simple() {
        let tpl = "Items: {{#each items}}{{name}}, {{/each}}done.";
        let ctx = RenderContext::new().with_var("items", r#"[{"name":"A"},{"name":"B"}]"#);
        assert_eq!(render_markdown(tpl, &ctx).unwrap(), "Items: A, B, done.");
    }

    #[test]
    fn test_each_with_index() {
        let tpl = "{{#each items}}{{index}}: {{name}}\n{{/each}}";
        let ctx = RenderContext::new().with_var("items", r#"[{"name":"X"},{"name":"Y"}]"#);
        assert_eq!(render_markdown(tpl, &ctx).unwrap(), "0: X\n1: Y\n");
    }

    #[test]
    fn test_each_item_alias() {
        let tpl = "{{#each items}}raw: {{item}}\n{{/each}}";
        let ctx = RenderContext::new().with_var("items", r#"[{"a":1},{"a":2}]"#);
        let out = render_markdown(tpl, &ctx).unwrap();
        assert!(out.contains("a"));
    }

    #[test]
    fn test_nested_each() {
        let tpl = "{{#outer}}[{{#inner}}{{name}}{{/inner}}]{{/outer}}";
        let ctx = RenderContext::new()
            .with_var(
                "outer",
                r#"[{"inner":[{"name":"A"},{"name":"B"}]},{"inner":[{"name":"C"}]}]"#,
            )
            .with_var("inner", r#"[{"name":"X"}]"#);
        let out = render_markdown(tpl, &ctx).unwrap();
        assert_eq!(out, "[AB][C]");
    }

    #[test]
    fn test_helpers() {
        let tpl = "{{uppercase name}} | {{lowercase city}} | {{trim spaced}}";
        let ctx = RenderContext::new()
            .with_var("name", "bob")
            .with_var("city", "PARIS")
            .with_var("spaced", "  word  ");
        assert_eq!(
            render_markdown(tpl, &ctx).unwrap(),
            "BOB | paris | word"
        );
    }
}
