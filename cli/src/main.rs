use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use specgen_core::{
    parse_template, render_markdown, validate_template, RenderContext, TemplateFormat,
};
use std::collections::HashSet;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "specgen")]
#[command(about = "Workflow template generator and validator with integrated Craft database", long_about = None)]
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
        /// Override input format (json|md|toml)
        #[arg(long, value_parser = ["json", "md", "toml"])]
        format: Option<String>,
        /// Output format (markdown|json|yaml)
        #[arg(long, value_enum, default_value_t = OutputFormat::Markdown)]
        output_format: OutputFormat,
        /// Output file path (stdout if omitted)
        #[arg(short, long)]
        out: Option<String>,
        /// Variables in key=value format, can repeat
        #[arg(long, value_parser = parse_keyval)]
        var: Vec<(String, String)>,
    },
    /// Template management operations
    Template {
        #[command(subcommand)]
        cmd: TemplateCommands,
    },
    /// Convert a template from one format to another
    Convert {
        /// Path to source template file
        file: String,
        /// Output format (json|md|toml)
        #[arg(long, value_parser = ["json", "md", "toml"])]
        to: String,
        /// Output file path (stdout if omitted)
        #[arg(short, long)]
        out: Option<String>,
    },
    /// Database operations (Craft Engine)
    Db {
        #[command(subcommand)]
        cmd: DbCommands,
    },
    /// Rule management
    Rule {
        #[command(subcommand)]
        cmd: RuleCommands,
    },
    /// Agent management
    Agent {
        #[command(subcommand)]
        cmd: AgentCommands,
    },
    /// Index the current project for semantic search
    Index,
    /// Search for code logic using semantic search
    Search {
        /// The query to search for
        query: String,
    },
    /// Show system health and status
    Status {
        /// Database file path to check (default: craft.db)
        #[arg(short, long, default_value = "craft.db")]
        database: String,
    },
    /// Memory management
    Memory {
        #[command(subcommand)]
        cmd: MemoryCommands,
    },
    /// Show schema info
    Schema,
}

#[derive(Subcommand)]
enum MemoryCommands {
    /// List all memory entries
    List {
        /// Database file path (default: craft.db)
        #[arg(short, long, default_value = "craft.db")]
        database: String,
        /// Filter by scope
        #[arg(long)]
        scope: Option<String>,
        /// Output format (text|json)
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
    },
    /// Write a new memory entry
    Write {
        /// Database file path (default: craft.db)
        #[arg(short, long, default_value = "craft.db")]
        database: String,
        /// Scope (global|project|session|working|policy|identity)
        #[arg(long)]
        scope: String,
        /// Category (fact|preference|history|context|inference)
        #[arg(long)]
        category: String,
        /// Key
        #[arg(long)]
        key: String,
        /// Value
        #[arg(long)]
        value: String,
    },
}

#[derive(Subcommand)]
enum TemplateCommands {
    /// List available templates (by ID)
    List,
    /// Create a new template from boilerplate
    New {
        /// Name of the template (ID)
        name: String,
        /// Template format (json|md|toml)
        #[arg(long, default_value = "md", value_parser = ["json", "md", "toml"])]
        format: String,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// Initialize a new database with standard collections
    Init {
        /// Database file path (default: craft.db)
        #[arg(short, long, default_value = "craft.db")]
        database: String,
        /// Force initialization even if database file already exists
        #[arg(short, long)]
        force: bool,
    },
    /// Import a Markdown file into the database as hierarchical blocks
    Import {
        /// Path to markdown file
        file: PathBuf,
        /// Database file path (default: craft.db)
        #[arg(short, long, default_value = "craft.db")]
        database: String,
    },
    /// List all documents in the database
    List {
        /// Database file path (default: craft.db)
        #[arg(short, long, default_value = "craft.db")]
        database: String,
    },
    /// Handle generic tool call (usually from MCP)
    #[command(external_subcommand)]
    Generic(Vec<String>),
}

#[derive(Subcommand)]
enum RuleCommands {
    /// List all rules from the database
    List {
        /// Database file path (default: craft.db)
        #[arg(short, long, default_value = "craft.db")]
        database: String,
        /// Output format (text|json)
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
    },
}

#[derive(Subcommand)]
enum AgentCommands {
    /// List all agents from the database
    List {
        /// Database file path (default: craft.db)
        #[arg(short, long, default_value = "craft.db")]
        database: String,
        /// Output format (text|json)
        #[arg(long, default_value = "text", value_parser = ["text", "json"])]
        format: String,
    },
}

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
enum OutputFormat {
    Markdown,
    Json,
    Yaml,
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
            output_format,
            out,
            var,
        } => {
            // Determine template file path
            let template_path = if template.contains('.') {
                template
            } else {
                if let Some(ref fmt) = format {
                    format!("templates/{}.{}", template, fmt)
                } else {
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

            let fmt_opt = format.map(|f| match f.as_str() {
                "json" => TemplateFormat::Json,
                "md" => TemplateFormat::Markdown,
                "toml" => TemplateFormat::Toml,
                _ => unreachable!(),
            });

            let mut template_value = parse_template(&template_path, fmt_opt)
                .map_err(|e| anyhow!("Failed to load template '{}': {}", template_path, e))?;

            let ctx = var
                .into_iter()
                .fold(RenderContext::new(), |acc, (k, v)| acc.with_var(&k, &v));

            let content = template_value["output_template"]["content"]
                .as_str()
                .ok_or_else(|| anyhow!("output_template.content must be a string"))?;

            let rendered =
                render_markdown(content, &ctx).map_err(|e| anyhow!("Render error: {}", e))?;

            let final_output = match output_format {
                OutputFormat::Markdown => rendered,
                OutputFormat::Json => {
                    template_value["output_template"]["content"] =
                        serde_json::Value::String(rendered);
                    serde_json::to_string_pretty(&template_value)?
                }
                OutputFormat::Yaml => {
                    template_value["output_template"]["content"] =
                        serde_json::Value::String(rendered);
                    serde_yaml::to_string(&template_value)?
                }
            };

            match out {
                Some(path) => {
                    std::fs::write(&path, &final_output)
                        .with_context(|| format!("Failed to write {}", path))?;
                    println!("Output written to {}", path);
                }
                None => {
                    println!("{}", final_output);
                }
            }
        }
        Commands::Template { cmd } => match cmd {
            TemplateCommands::List => {
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
            TemplateCommands::New { name, format } => {
                let dir = "templates";
                if !Path::new(dir).exists() {
                    std::fs::create_dir(dir)
                        .with_context(|| format!("Failed to create {} directory", dir))?;
                }
                let path = format!("{}/{}.{}", dir, name, format);
                if Path::new(&path).exists() {
                    anyhow::bail!("Template '{}' already exists at {}", name, path);
                }
                let boilerplate = match format.as_str() {
                    "json" => {
                        r##"{
  "workflow": {
    "title": "New Workflow",
    "steps": [
      { "id": "0", "critical": true, "content": "First step" }
    ]
  },
  "output_template": {
    "format": "markdown",
    "content": "# New Output\nHello, {{name}}!"
  }
}"##
                    }
                    "toml" => {
                        r##"[workflow]
title = "New Workflow"

[[workflow.steps]]
id = "0"
critical = true
content = "First step"

[output_template]
format = "markdown"
content = """
# New Output
Hello, {{name}}!
"""
"##
                    }
                    "md" => {
                        r##"<workflow title="New Workflow">
<step id="0" critical="true">
First step
</step>
<output_template format="markdown">
# New Output
Hello, {{name}}!
</output_template>
</workflow>
"##
                    }
                    _ => unreachable!(),
                };
                std::fs::write(&path, boilerplate)
                    .with_context(|| format!("Failed to create template {}", path))?;
                println!("Created new {} template: {}", format, path);
            }
        },
        Commands::Convert { file, to, out } => {
            let value = parse_template(&file, None)
                .map_err(|e| anyhow!("Failed to parse {}: {}", file, e))?;

            let out_fmt = match to.as_str() {
                "json" => TemplateFormat::Json,
                "md" => TemplateFormat::Markdown,
                "toml" => TemplateFormat::Toml,
                _ => unreachable!(),
            };

            let converted = specgen_core::serialize_template(&value, out_fmt)
                .map_err(|e| anyhow!("Serialization error: {}", e))?;

            match out {
                Some(path) => {
                    std::fs::write(&path, &converted)
                        .with_context(|| format!("Failed to write to {}", path))?;
                    println!("Successfully converted {} to {}", file, path);
                }
                None => {
                    println!("{}", converted);
                }
            }
        }
        Commands::Db { cmd } => match cmd {
            DbCommands::Init { database, force } => {
                if Path::new(&database).exists() && !force {
                    anyhow::bail!(
                        "Database `{}` already exists. Use --force to overwrite.",
                        database
                    );
                }
                let conn = craft_local_db::db::open(&database)?;
                let schema = include_str!("../../craft/schema.sql");
                craft_local_db::db::run_schema(&conn, schema)?;

                let doc_id = craft_local_db::db::create_document(&conn, "Project Workspace")?;
                println!("📄 Creating document `Project Workspace` (ID: {})", doc_id);

                let collections = [
                    (
                        "Rules",
                        vec![
                            ("Rule Text", "text"),
                            ("Category", "select"),
                            ("Priority", "select"),
                        ],
                    ),
                    (
                        "Agents",
                        vec![
                            ("Name", "text"),
                            ("Description", "text"),
                            ("Capability", "multi_select"),
                        ],
                    ),
                    (
                        "Skills",
                        vec![
                            ("Name", "text"),
                            ("Instructions", "text"),
                            ("Tools", "text"),
                        ],
                    ),
                    (
                        "Commands",
                        vec![("Name", "text"), ("Usage", "text"), ("Description", "text")],
                    ),
                    (
                        "KB",
                        vec![
                            ("Title", "text"),
                            ("Content", "text"),
                            ("Tags", "multi_select"),
                        ],
                    ),
                ];

                for (name, props) in collections {
                    let coll_id = craft_local_db::db::create_collection(&conn, &doc_id, name)?;
                    for (i, (p_name, p_type)) in props.into_iter().enumerate() {
                        craft_local_db::db::add_property(
                            &conn, &coll_id, p_name, p_type, i as i32,
                        )?;
                    }
                    println!("✅ Created Collection: {}", name);
                }
                println!(
                    "🚀 Database `{}` initialized with standard collections.",
                    database
                );
            }
            DbCommands::List { database } => {
                if !Path::new(&database).exists() {
                    anyhow::bail!("Database file `{}` not found.", database);
                }
                let conn = craft_local_db::db::open(&database)?;
                let docs = craft_local_db::db::list_documents(&conn)?;
                println!("Documents in `{}`:", database);
                for doc in docs {
                    println!(" - {}", doc);
                }
            }
            DbCommands::Import { file, database } => {
                let conn = craft_local_db::db::open(&database)?;
                let schema = include_str!("../../craft/schema.sql");
                craft_local_db::db::run_schema(&conn, schema)?;

                let md = std::fs::read_to_string(&file)?;
                let imported = craft_local_db::markdown::import_markdown(&conn, &md, None)?;
                println!(
                    "✅ Imported `{}` -> Document ID: {} (Title: {})",
                    file.display(),
                    imported.document_id,
                    imported.title,
                );
            }
            DbCommands::Generic(args) => {
                println!("Generic tool call received: {:?}", args);
                println!("Status: Not implemented in CLI yet. Setting up plumbing...");
                // Here we would parse args, identify the tool name (first arg),
                // and then execute the corresponding craft logic.
            }
        },
        Commands::Rule { cmd } => match cmd {
            RuleCommands::List { database, format } => {
                if !Path::new(&database).exists() {
                    anyhow::bail!(
                        "Database file `{}` not found. Please run `specgen db init` first.",
                        database
                    );
                }
                let conn = craft_local_db::db::open(&database)?;
                let mut stmt = conn.prepare(
                    "SELECT content FROM blocks b 
                     JOIN documents d ON b.document_id = d.id 
                     WHERE d.title = 'Rules' OR b.type = 'text'",
                )?;
                let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
                let mut rules = Vec::new();
                for row in rows {
                    rules.push(row?);
                }

                if format == "json" {
                    println!("{}", serde_json::to_string_pretty(&rules)?);
                } else {
                    println!("Current Rules:");
                    for rule in rules {
                        println!(" - {}", rule);
                    }
                }
            }
        },
        Commands::Agent { cmd } => match cmd {
            AgentCommands::List { database, format } => {
                if !Path::new(&database).exists() {
                    anyhow::bail!(
                        "Database file `{}` not found. Please run `specgen db init` first.",
                        database
                    );
                }
                let conn = craft_local_db::db::open(&database)?;
                let mut stmt =
                    conn.prepare("SELECT name FROM collections WHERE name = 'Agents'")?;
                let mut rows = stmt.query([])?;
                let mut agents = Vec::new();
                while let Some(row) = rows.next()? {
                    agents.push(row.get::<_, String>(0)?);
                }

                if format == "json" {
                    println!("{}", serde_json::to_string_pretty(&agents)?);
                } else {
                    println!("Current Agents:");
                    for agent in agents {
                        println!(" - {}", agent);
                    }
                }
            }
        },
        Commands::Index => {
            let root = std::env::current_dir()?;
            let sense = specgen_core::sense::CodeSense::new(&root)?;
            sense.index(&root)?;
        }
        Commands::Search { query } => {
            let root = std::env::current_dir()?;
            let sense = specgen_core::sense::CodeSense::new(&root)?;
            sense.search(&query)?;
        }
        Commands::Status { database } => {
            println!("📊 Project Status");

            // Templates
            let template_count = std::fs::read_dir("templates")
                .map(|d| d.filter_map(|e| e.ok()).count())
                .unwrap_or(0);
            println!(
                "   - Template Engine: READY ({} templates found)",
                template_count
            );

            // Database
            if Path::new(&database).exists() {
                let conn = craft_local_db::db::open(&database)?;
                let table_exists: bool = conn.query_row(
                    "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='collections'",
                    [],
                    |r| r.get::<_, i32>(0).map(|c| c > 0)
                ).unwrap_or(false);

                if table_exists {
                    let coll_count: i32 =
                        conn.query_row("SELECT COUNT(*) FROM collections", [], |r| r.get(0))?;
                    println!(
                        "   - Database: READY (`{}`, {} collections initialized)",
                        database, coll_count
                    );
                } else {
                    println!("   - Database: UNINITIALIZED (`{}` exists but schema is missing. Run `specgen db init`)", database);
                }
            } else {
                println!("   - Database: NOT FOUND (Run `specgen db init`)");
            }

            // Search Index
            let sense_dir = Path::new(".sense");
            if sense_dir.exists() {
                println!("   - Search Index: READY");
            } else {
                println!("   - Search Index: NOT INDEXED (run `specgen index`)");
            }

            // MCP Server
            let mcp_src = Path::new("app/src/mcp.ts");
            if mcp_src.exists() {
                println!("   - MCP Server: CONFIGURED (source found)");
            } else {
                println!("   - MCP Server: NOT FOUND");
            }
        }
        Commands::Memory { cmd } => match cmd {
            MemoryCommands::List {
                database,
                scope,
                format,
            } => {
                let store = specgen_core::memory::MemoryStore::new(&database)?;
                let query = specgen_core::memory::MemoryQuery {
                    scope,
                    ..Default::default()
                };
                let entries = store.query(&query)?;

                if format == "json" {
                    let entries_json: Vec<serde_json::Value> = entries
                        .into_iter()
                        .map(|e| {
                            serde_json::json!({
                                "id": e.id,
                                "scope": e.scope,
                                "category": e.category,
                                "key": e.key,
                                "value": e.value,
                                "confidence": e.confidence,
                            })
                        })
                        .collect();
                    println!("{}", serde_json::to_string_pretty(&entries_json)?);
                } else {
                    println!("Memory Entries in `{}`:", database);
                    for e in entries {
                        let scope_str = specgen_core::memory::scope_to_string(e.scope)
                            .unwrap_or_else(|_| e.scope.to_string());
                        println!(
                            " [{}] {}: {} (conf: {:.2})",
                            scope_str, e.key, e.value, e.confidence
                        );
                    }
                }
            }
            MemoryCommands::Write {
                database,
                scope,
                category,
                key,
                value,
            } => {
                let store = specgen_core::memory::MemoryStore::new(&database)?;

                let scope_enum = specgen_core::memory::string_to_memory_scope(&scope)
                    .ok_or_else(|| anyhow::anyhow!("Invalid scope: {}", scope))?;
                let category_enum = specgen_core::memory::string_to_memory_category(&category)
                    .ok_or_else(|| anyhow::anyhow!("Invalid category: {}", category))?;

                let entry = specgen_core::bl1nk::MemoryEntry {
                    id: None,
                    scope: scope_enum as i32,
                    category: category_enum as i32,
                    key,
                    value,
                    source: None,
                    confidence: 1.0,
                    status: "active".into(),
                    created_at: specgen_core::memory::current_timestamp()?,
                    updated_at: specgen_core::memory::current_timestamp()?,
                    version: 1,
                    tags: vec![],
                    owner: None,
                    access_level: "private".into(),
                    provenance: None,
                    expires_at: None,
                };

                store.insert(entry)?;
                println!("✅ Memory entry written to `{}`", database);
            }
        },
        Commands::Schema => {
            println!("Schema: schema/template_schema.json");
            println!("Version: 0.1.0");
        }
    }
    Ok(())
}
