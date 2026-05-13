use anyhow::{anyhow, Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use specgen_core::{
    parse_template, render_markdown, validate_template, RenderContext, TemplateFormat,
    task_delegator::TaskStatus,
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
    Index {
        /// Run in background
        #[arg(short, long)]
        background: bool,
    },
    /// Search for code logic using semantic search
    Search {
        /// The query to search for
        query: String,
    },
    /// Task delegation and scheduling
    Task {
        #[command(subcommand)]
        cmd: TaskCommands,
    },
    /// Synchronize local changes with remote cloud
    Sync {
        /// Cloud endpoint URL
        #[arg(long, env = "KILOCODE_URL")]
        endpoint: Option<String>,
        /// Push local changes to cloud
        #[arg(long)]
        push: bool,
    },
    /// Setup and manage scheduled jobs (Cron)
    Cron {
        #[command(subcommand)]
        cmd: CronCommands,
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
    /// Manage Skills via PyO3 integration
    Skill {
        #[command(subcommand)]
        cmd: SkillCommands,
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
enum TaskCommands {
    /// Add a new delegated task
    Add {
        /// Task title
        title: String,
        /// Schedule (e.g. "daily 08:00")
        #[arg(long)]
        schedule: Option<String>,
        /// Repeat rule (e.g. "1d")
        #[arg(long)]
        repeat: Option<String>,
        /// Database file path (default: craft.db)
        #[arg(short, long, default_value = "craft.db")]
        database: String,
    },
    /// List all tasks
    List {
        /// Database file path (default: craft.db)
        #[arg(short, long, default_value = "craft.db")]
        database: String,
    },
    /// Start a task worker (runs in background)
    Worker {
        /// Database file path (default: craft.db)
        #[arg(short, long, default_value = "craft.db")]
        database: String,
        /// Polling interval in seconds
        #[arg(long, default_value = "30")]
        interval: u64,
    },
}

#[derive(Subcommand)]
enum CronCommands {
    /// Schedule a daily job
    Add {
        /// Command to run
        command: String,
        /// Time in HH:MM format
        time: String,
    },
    /// List all scheduled jobs
    List,
    /// Remove all jobs
    Clear,
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
enum SkillCommands {
    /// Distill and classify skills from a directory using PyO3
    Distill {
        /// Directory to search for SKILL.md files
        #[arg(short, long)]
        dir: String,
        /// Path to python logic script
        #[arg(long, default_value = "scripts/distiller_logic.py")]
        script: String,
    },
    /// List all distilled skills
    List,
    /// Search for skills using semantic search
    Search {
        /// The query to search for
        query: String,
    },
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
        Commands::Index { background } => {
            let root = std::env::current_dir()?;
            if background {
                let bin = std::env::current_exe()?;
                let log_file = root.join("index.log");
                let command = format!(
                    "{} index > {} 2>&1 && termux-notification --title 'Specgen' --content 'Code indexing complete! 🔍'",
                    bin.display(),
                    log_file.display()
                );
                println!("🚀 Starting indexing in background. Log: {}", log_file.display());
                std::process::Command::new("bash")
                    .arg("-c")
                    .arg(command)
                    .spawn()?;
            } else {
                let sense = specgen_core::sense::CodeSense::new(&root)?;
                sense.index(&root)?;
            }
        }
        Commands::Search { query } => {
            let root = std::env::current_dir()?;
            let sense = specgen_core::sense::CodeSense::new(&root)?;
            sense.search(&query)?;
        }
        Commands::Skill { cmd } => match cmd {
            SkillCommands::Distill { dir, script } => {
                println!("🚀 Starting Skill Distillation via PyO3...");
                let distiller = specgen_core::distiller::SkillDistiller::new(Path::new(&script))?;
                
                let walker = ignore::WalkBuilder::new(&dir).build();
                let mut count = 0;
                let mut slop_count = 0;
                
                for entry in walker.into_iter().filter_map(|e| e.ok()) {
                    if entry.path().is_file() && (entry.path().ends_with("SKILL.md") || entry.path().ends_with("skill.md")) {
                        count += 1;
                        let path = entry.path();
                        match distiller.analyze_file(path) {
                            Ok(meta) => {
                                if meta.is_slop {
                                    slop_count += 1;
                                    println!("🗑️  SLOP: {} (Score: {}/10) - {}", meta.name, meta.quality_score, path.display());
                                } else {
                                    println!("✅ KEEP: {} (Tags: {:?}) - {}", meta.name, meta.tags, path.display());
                                }
                            }
                            Err(e) => {
                                println!("❌ ERROR processing {}: {}", path.display(), e);
                            }
                        }
                    }
                }
                println!("\n📊 Distillation Complete: Processed {} files, found {} low-quality (slop) skills.", count, slop_count);
            }
        },
        Commands::Task { cmd } => match cmd {
            TaskCommands::Add {
                title,
                schedule,
                repeat,
                database,
            } => {
                let delegator = specgen_core::task_delegator::TaskDelegator::new(&database)?;
                let id = delegator.add_task(&title, schedule.as_deref(), repeat.as_deref())?;
                println!("✅ Task added (ID: {})", id);
            }
            TaskCommands::List { database } => {
                let delegator = specgen_core::task_delegator::TaskDelegator::new(&database)?;
                let tasks = delegator.list_tasks()?;
                println!("Delegated Tasks in `{}`:", database);
                for t in tasks {
                    println!(" [{}] {} - {}", t.status, t.title, t.created_at);
                    if let Some(s) = t.schedule {
                        println!("    Schedule: {}", s);
                    }
                }
            }
            TaskCommands::Worker { database, interval } => {
                println!("👷 Task Worker started for `{}`", database);
                println!("Interval: {}s (Ctrl+C to stop)", interval);
                let delegator = specgen_core::task_delegator::TaskDelegator::new(&database)?;
                loop {
                    let tasks = delegator.list_tasks()?;
                    let pending: Vec<_> = tasks.iter().filter(|t| t.status == "todo").collect();
                    
                    if !pending.is_empty() {
                        println!("🔄 Found {} pending tasks. Processing...", pending.len());
                    }
                    
                    for task in pending.iter() {
                        println!("  → Running: {} (ID: {})", task.title, task.id);
                        
                        // Mark as in_progress
                        if let Err(e) = delegator.update_status(&task.id, TaskStatus::InProgress) {
                            println!("  ⚠️  Failed to update status: {}", e);
                            continue;
                        }
                        
                        // Execute the task as a shell command
                        let output = std::process::Command::new("sh")
                            .arg("-c")
                            .arg(&task.title)
                            .output();
                        
                        let result = match output {
                            Ok(out) => {
                                let stdout = String::from_utf8_lossy(&out.stdout);
                                let stderr = String::from_utf8_lossy(&out.stderr);
                                let success = out.status.success();
                                
                                println!("  Status: {}", if success { "✅ done" } else { "❌ failed" });
                                if !stdout.is_empty() {
                                    println!("  stdout: {}", stdout.trim());
                                }
                                if !stderr.is_empty() {
                                    println!("  stderr: {}", stderr.trim());
                                }
                                
                                success
                            }
                            Err(e) => {
                                println!("  ❌ Execution error: {}", e);
                                false
                            }
                        };
                        
                        // Update final status
                        let final_status = if result {
                            TaskStatus::Done
                        } else {
                            TaskStatus::Failed
                        };
                        if let Err(e) = delegator.update_status(&task.id, final_status) {
                            println!("  ⚠️  Failed to update final status: {}", e);
                        }
                    }
                    
                    std::thread::sleep(std::time::Duration::from_secs(interval));
                }
            }
        },
        Commands::Sync { endpoint, push } => {
            let root = std::env::current_dir()?;
            if push {
                println!("📦 Preparing local changes for sync...");
                let patch = specgen_core::sync::SyncManager::prepare_patch(&root)?;
                if patch.is_empty() {
                    println!("✨ No changes to sync.");
                } else {
                    println!("🚀 Pushing patch to remote: {}", endpoint.as_deref().unwrap_or("Kilocode Gateway"));
                    let result = specgen_core::sync::SyncManager::send_patch(&patch, endpoint.as_deref())?;
                    println!("✅ Sync packet sent. Server response:");
                    println!("{}", serde_json::to_string_pretty(&result).unwrap_or_default());
                }
            }
        },
        Commands::Cron { cmd } => match cmd {
            CronCommands::Add { command, time } => {
                // Validate time format HH:MM
                let parts: Vec<&str> = time.split(':').collect();
                if parts.len() != 2 {
                    anyhow::bail!("Invalid time format. Use HH:MM");
                }
                let hour = parts[0];
                let min = parts[1];
                if hour.parse::<u32>().is_err() || min.parse::<u32>().is_err() {
                    anyhow::bail!("Hour and minute must be numbers");
                }
                let cron_entry = format!("{} {} * * * {}", min, hour, command);

                // Read existing crontab
                let existing = match std::process::Command::new("crontab")
                    .arg("-l")
                    .output()
                {
                    Ok(out) => {
                        let s = String::from_utf8_lossy(&out.stdout);
                        if out.status.success() { s.to_string() } else { "".to_string() }
                    }
                    Err(_) => "".to_string(),
                };

                // Check duplicates
                if existing.lines().any(|line| line.trim() == cron_entry) {
                    anyhow::bail!("This cron entry already exists");
                }

                // Append new entry
                let mut new_crontab = existing;
                if !new_crontab.is_empty() && !new_crontab.ends_with('\n') {
                    new_crontab.push('\n');
                }
                new_crontab.push_str(&cron_entry);
                new_crontab.push('\n');

                // Install new crontab
                let mut child = std::process::Command::new("crontab")
                    .arg("-")
                    .stdin(std::process::Stdio::piped())
                    .spawn()
                    .context("Failed to spawn crontab")?;
                {
                    let stdin = child.stdin.as_mut().unwrap();
                    std::io::Write::write_all(stdin, new_crontab.as_bytes())?;
                }
                let status = child.wait()?;
                if status.success() {
                    println!("✅ Cron job installed: '{}' at {} (min hour)", command, time);
                } else {
                    anyhow::bail!("crontab installation failed");
                }
            }
            CronCommands::List => {
                let output = std::process::Command::new("crontab")
                    .arg("-l")
                    .output()
                    .context("Failed to read crontab")?;
                if output.status.success() {
                    let text = String::from_utf8_lossy(&output.stdout);
                    if text.trim().is_empty() {
                        println!("📭 Crontab is empty");
                    } else {
                        println!("📋 Current crontab:");
                        println!("{}", text);
                    }
                } else {
                    anyhow::bail!("crontab -l failed (no crontab?)");
                }
            }
            CronCommands::Clear => {
                print!("⚠️  Remove all cron jobs? (y/N): ");
                std::io::Write::flush(&mut std::io::stdout())?;
                let mut input = String::new();
                std::io::stdin().read_line(&mut input)?;
                if input.trim().eq_ignore_ascii_case("y") {
                    std::process::Command::new("crontab")
                        .arg("-r")
                        .status()
                        .context("Failed to clear crontab")?;
                    println!("🗑️  All cron jobs removed");
                } else {
                    println!("Cancelled.");
                }
            }
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
