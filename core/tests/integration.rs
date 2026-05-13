//! Integration tests for specgen-core parsers (Markdown, TOML, JSON)
//! Tests end-to-end parsing, validation, and rendering.

use specgen_core::load_schema;
use specgen_core::parser::{parse_template_str, TemplateFormat};
use specgen_core::renderer::{render_markdown, RenderContext};
use specgen_core::validator::validate_template;
use std::fs;

/// Helper: read a sample template from ../templates/
fn load_template(name: &str) -> String {
    fs::read_to_string(format!("../templates/{}", name))
        .expect(&format!("Failed to read ../templates/{}", name))
}

/// Render a parsed template Value by extracting output_template.content
fn render_parsed(value: &serde_json::Value) -> Result<String, String> {
    let output = value
        .get("output_template")
        .and_then(|v| v.get("content"))
        .and_then(|v| v.as_str())
        .ok_or_else(|| "Template missing output_template.content".to_string())?;
    let ctx = RenderContext::new().with_var("Project Name", "Test Project");
    render_markdown(output, &ctx)
}

#[test]
fn test_parse_json_template() {
    let src = load_template("spec-workflow.json");
    let value = parse_template_str(&src, TemplateFormat::Json).expect("JSON parse should succeed");
    assert!(value.get("workflow").is_some());
    let wf = &value["workflow"];
    assert!(wf.get("title").is_some());
    assert!(wf.get("steps").is_some());
    let steps = wf["steps"].as_array().expect("steps must be array");
    assert!(!steps.is_empty(), "at least one step expected");
}

#[test]
fn test_parse_markdown_template() {
    let src = load_template("spec-workflow.md");
    let value = parse_template_str(&src, TemplateFormat::Markdown)
        .expect("Markdown+XML parse should succeed");
    assert!(value.get("workflow").is_some());
    let wf = &value["workflow"];
    assert_eq!(wf["title"], "การจัดการไฟล์ SPEC.md");
}

#[test]
fn test_parse_toml_template() {
    let src = load_template("spec-workflow.toml");
    let value = parse_template_str(&src, TemplateFormat::Toml).expect("TOML parse should succeed");
    assert!(value.get("workflow").is_some());
    let wf = &value["workflow"];
    assert_eq!(wf["title"], "การจัดการไฟล์ SPEC.md");
}

#[test]
fn test_validate_all_templates() {
    let schema = load_schema();
    for (name, fmt) in [
        ("spec-workflow.json", TemplateFormat::Json),
        ("spec-workflow.md", TemplateFormat::Markdown),
        ("spec-workflow.toml", TemplateFormat::Toml),
    ] {
        let src = load_template(name);
        let value = parse_template_str(&src, fmt)
            .unwrap_or_else(|e| panic!("parse {} failed: {}", name, e));
        let result = validate_template(&schema, &value);
        assert!(
            result.is_ok(),
            "Validation failed for {}: {:?}",
            name,
            result.err()
        );
    }
}

#[test]
fn test_render_from_markdown() {
    let md = load_template("spec-workflow.md");
    let value = parse_template_str(&md, TemplateFormat::Markdown).unwrap();
    let rendered = render_parsed(&value).expect("render should succeed");
    assert!(rendered.contains("Test Project"));
}

#[test]
fn test_render_from_toml() {
    let toml = load_template("spec-workflow.toml");
    let value = parse_template_str(&toml, TemplateFormat::Toml).unwrap();
    let rendered = render_parsed(&value).expect("render should succeed");
    assert!(rendered.contains("Test Project"));
    assert!(rendered.contains("Run & Operate"));
}

#[test]
fn test_render_from_json() {
    let json = load_template("spec-workflow.json");
    let value = parse_template_str(&json, TemplateFormat::Json).unwrap();
    let rendered = render_parsed(&value).expect("render should succeed");
    assert!(rendered.contains("Test Project"));
}

#[test]
fn test_auto_detect_from_path() {
    let md_path = std::path::Path::new("../templates/spec-workflow.md");
    let toml_path = std::path::Path::new("../templates/spec-workflow.toml");
    let json_path = std::path::Path::new("../templates/spec-workflow.json");
    assert_eq!(
        TemplateFormat::from_path(md_path),
        Some(TemplateFormat::Markdown)
    );
    assert_eq!(
        TemplateFormat::from_path(toml_path),
        Some(TemplateFormat::Toml)
    );
    assert_eq!(
        TemplateFormat::from_path(json_path),
        Some(TemplateFormat::Json)
    );
}

#[test]
fn test_structure_after_parsing() {
    // Ensure all three formats produce equivalent JSON structure
    let formats = [
        ("spec-workflow.json", TemplateFormat::Json),
        ("spec-workflow.md", TemplateFormat::Markdown),
        ("spec-workflow.toml", TemplateFormat::Toml),
    ];

    let mut parsed = Vec::new();
    for (name, fmt) in &formats {
        let src = load_template(name);
        let value = parse_template_str(&src, fmt.clone())
            .unwrap_or_else(|e| panic!("failed to parse {}: {}", name, e));
        parsed.push(value);
    }

    // All should have non-empty workflow title
    for (v, (name, _)) in parsed.iter().zip(&formats) {
        let wf = &v["workflow"];
        let title = wf["title"].as_str().unwrap_or("");
        assert!(!title.trim().is_empty(), "{}: title empty or missing", name);
    }

    // All should have at least one step
    for (v, (name, _)) in parsed.iter().zip(&formats) {
        let steps = v["workflow"]["steps"]
            .as_array()
            .unwrap_or_else(|| panic!("{}: steps missing or not array", name));
        assert!(!steps.is_empty(), "{}: no steps", name);
    }
}
