use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand};
use specgen_core::{
    parse_template, render_markdown, validate_template, RenderContext, TemplateFormat,
};
use std::collections::HashSet;
use std::path::Path;

#[derive(Parser)]
#[command(name = "specgen")]
#[command(about = "Workflow template generator and validator", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Validate a template file (JSON, Markdown+XML, or TOML) against the schema
    Validate {
        /// Path to template file
        file: String,
        /// Override format detection (json|md|toml)
        #[arg(long, value_parser = ["json", "md", "toml"])]
        format: Option<String>,
    },
    /// Render output from a template using variables
    Generate {
        /// Template ID (name without extension) or path to template file
        template: String,
        /// Override format (json|md|toml)
        #[arg(long, value_parser = ["json", "md", "toml"])]
        format: Option<String>,
        /// Output file path (stdout if omitted)
        #[arg(short, long)]
        out: Option<String>,
        /// Variables in key=value format, can repeat
        #[arg(long, value_parser = parse_keyval)]
        var: Vec<(String, String)>,
    },
    /// List available templates (by ID)
    ListTemplates,
    /// Show schema info
    Schema,
}

fn parse_keyval(s: &str) -> Result<(String, String), String> {
    let pos = s.find('=').ok_or("KEY=VALUE format required".to_string())?;
    Ok((s[..pos].to_string(), s[pos + 1..].to_string()))
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate { file, format } => {
            let fmt_opt = format.map(|f| match f.as_str() {
                "json" => TemplateFormat::Json,
                "md" => TemplateFormat::Markdown,
                "toml" => TemplateFormat::Toml,
                _ => unreachable!(),
            });
            let instance =
                parse_template(&file, fmt_opt).map_err(|e| anyhow!("Template error: {}", e))?;
            let schema = specgen_core::load_schema();
            match validate_template(&schema, &instance) {
                Ok(()) => {
                    println!("Template is valid.");
                }
                Err(errors) => {
                    anyhow::bail!(
                        "Template is invalid:\n{}",
                        errors
                            .iter()
                            .map(|e| format!(" - {}", e))
                            .collect::<Vec<_>>()
                            .join("\n")
                    );
                }
            }
        }
        Commands::Generate {
            template,
            format,
            out,
            var,
        } => {
            // Determine template file path
            let template_path = if template.contains('.') {
                // Assume it's a file path (relative or absolute)
                template
            } else {
                // If format explicitly provided, use that extension directly
                if let Some(ref fmt) = format {
                    format!("templates/{}.{}", template, fmt)
                } else {
                    // Search in templates/ directory with supported extensions
                    let extensions = ["json", "md", "toml"];
                    let mut found = None;
                    for ext in &extensions {
                        let p = format!("templates/{}.{}", template, ext);
                        if Path::new(&p).exists() {
                            found = Some(p);
                            break;
                        }
                    }
                    match found {
                        Some(p) => p,
                        None => {
                            return Err(anyhow!(
                                "Template '{}' not found in templates/ with supported extensions",
                                template
                            ));
                        }
                    }
                }
            };

            // Override format if provided explicitly
            let fmt_opt = format.map(|f| match f.as_str() {
                "json" => TemplateFormat::Json,
                "md" => TemplateFormat::Markdown,
                "toml" => TemplateFormat::Toml,
                _ => unreachable!(),
            });

            let template_value = parse_template(&template_path, fmt_opt)
                .map_err(|e| anyhow!("Failed to load template '{}': {}", template_path, e))?;

            let ctx = var
                .into_iter()
                .fold(RenderContext::new(), |acc, (k, v)| acc.with_var(&k, &v));

            let content = template_value["output_template"]["content"]
                .as_str()
                .ok_or_else(|| anyhow!("output_template.content must be a string"))?;

            let rendered =
                render_markdown(content, &ctx).map_err(|e| anyhow!("Render error: {}", e))?;

            match out {
                Some(path) => {
                    std::fs::write(&path, &rendered)
                        .with_context(|| format!("Failed to write {}", path))?;
                    println!("Output written to {}", path);
                }
                None => {
                    println!("{}", rendered);
                }
            }
        }
        Commands::ListTemplates => {
            let dir = "templates";
            if let Ok(entries) = std::fs::read_dir(dir) {
                let mut ids = HashSet::new();
                for entry in entries.flatten() {
                    let p = entry.path();
                    if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
                        match ext.to_lowercase().as_str() {
                            "json" | "md" | "toml" => {
                                if let Some(stem) = p.file_stem().and_then(|s| s.to_str()) {
                                    ids.insert(stem.to_string());
                                }
                            }
                            _ => {}
                        }
                    }
                }
                if ids.is_empty() {
                    println!("No templates found in {}/", dir);
                } else {
                    println!("Available templates:");
                    let mut sorted: Vec<_> = ids.into_iter().collect();
                    sorted.sort();
                    for id in sorted {
                        println!("  - {}", id);
                    }
                }
            } else {
                anyhow::bail!("templates/ directory not found");
            }
        }
        Commands::Schema => {
            println!("Schema: schema/template_schema.json");
            println!("Version: 0.1.0");
        }
    }
    Ok(())
}
