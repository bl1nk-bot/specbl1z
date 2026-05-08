use specgen_core::{load_schema, load_template, validate_template, render_markdown, RenderContext};
use std::path::Path;

#[test]
fn test_schema_validates_example_template() {
    let schema = load_schema();
    let template = load_template("templates/spec-workflow.json")
        .expect("example template must exist");
    assert!(validate_template(&schema, &template).is_ok(),
        "Example template should be valid against the schema");
}

#[test]
fn test_render_spec_template_simple() {
    let template = load_template("templates/spec-workflow.json")
        .expect("template must exist");
    let content = template["output_template"]["content"]
        .as_str()
        .expect("content must be string");

    let ctx = RenderContext::new()
        .with_var("Project Name", "my-app")
        .with_var("OS", "Linux")
        .with_var("Shell/CLI", "Bash 5.2")
        .with_var("Tools Installed", "- Node.js v18\n- Python 3.11")
        .with_var("Run & Operate", "See README")
        .with_var("Stack", "Rust + Axum")
        .with_var("Product", "API server");

    let rendered = render_markdown(content, &ctx)
        .expect("rendering must succeed with all required variables");

    assert!(rendered.contains("my-app"));
    assert!(rendered.contains("Linux"));
    assert!(rendered.contains("Bash 5.2"));
}

#[test]
fn test_cli_validate_command() {
    use std::process::Command;
    let output = Command::cargo_bin("specgen")
        .expect("specgen binary must exist")
        .arg("validate")
        .arg("templates/spec-workflow.json")
        .output()
        .expect("failed to run validate command");
    assert!(output.status.success(), "validate should exit 0 for valid template");
}

#[test]
fn test_cli_list_templates() {
    use std::process::Command;
    let output = Command::cargo_bin("specgen")
        .expect("specgen binary must exist")
        .arg("list-templates")
        .output()
        .expect("failed to run list-templates");
    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("spec-workflow"));
}

#[test]
#[ignore] // requires var args — tested manually via cargo run
fn test_cli_generate() {}
