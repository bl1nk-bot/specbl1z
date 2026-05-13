use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;

mod db;
mod markdown;

/// โปรแกรมจัดการฐานข้อมูลแบบ Craft สำหรับใช้ในท้องถิ่น
#[derive(Parser, Debug)]
#[command(name = "craft-local-db")]
#[command(about = "Local Craft-like SQLite database with Markdown import")]
enum Cli {
    /// นำเข้าไฟล์ Markdown
    Import {
        /// ไฟล์ Markdown ที่ต้องการนำเข้า
        file: PathBuf,

        /// ฐานข้อมูล SQLite (ค่าเริ่มต้น: craft.db)
        #[arg(short, long, default_value = "craft.db")]
        database: String,
    },
    /// สร้างฐานข้อมูลเปล่า
    Init {
        /// ฐานข้อมูลที่จะสร้าง
        #[arg(short, long, default_value = "craft.db")]
        database: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli {
        Cli::Init { database } => {
            let conn = db::open(&database)?;
            let schema = include_str!("../schema.sql");
            db::run_schema(&conn, schema)?;

            // สร้างเอกสารหลักสำหรับเก็บ Collection
            let doc_id = db::create_document(&conn, "Project Workspace")?;
            println!("📄 สร้างเอกสาร `Project Workspace` (ID: {})", doc_id);

            // 1. Rule Collection
            let coll_id = db::create_collection(&conn, &doc_id, "Rules")?;
            db::add_property(&conn, &coll_id, "Rule Text", "text", 0)?;
            db::add_property(&conn, &coll_id, "Category", "select", 1)?;
            db::add_property(&conn, &coll_id, "Priority", "select", 2)?;
            println!("✅ สร้าง Collection: Rules");

            // 2. Agent Collection
            let coll_id = db::create_collection(&conn, &doc_id, "Agents")?;
            db::add_property(&conn, &coll_id, "Name", "text", 0)?;
            db::add_property(&conn, &coll_id, "Description", "text", 1)?;
            db::add_property(&conn, &coll_id, "Capability", "multi_select", 2)?;
            println!("✅ สร้าง Collection: Agents");

            // 3. Skill Collection
            let coll_id = db::create_collection(&conn, &doc_id, "Skills")?;
            db::add_property(&conn, &coll_id, "Name", "text", 0)?;
            db::add_property(&conn, &coll_id, "Instructions", "text", 1)?;
            db::add_property(&conn, &coll_id, "Tools", "text", 2)?;
            println!("✅ สร้าง Collection: Skills");

            // 4. Command Collection
            let coll_id = db::create_collection(&conn, &doc_id, "Commands")?;
            db::add_property(&conn, &coll_id, "Name", "text", 0)?;
            db::add_property(&conn, &coll_id, "Usage", "text", 1)?;
            db::add_property(&conn, &coll_id, "Description", "text", 2)?;
            println!("✅ สร้าง Collection: Commands");

            // 5. KB Collection (Knowledge Base)
            let coll_id = db::create_collection(&conn, &doc_id, "KB")?;
            db::add_property(&conn, &coll_id, "Title", "text", 0)?;
            db::add_property(&conn, &coll_id, "Content", "text", 1)?;
            db::add_property(&conn, &coll_id, "Tags", "multi_select", 2)?;
            println!("✅ สร้าง Collection: KB");

            println!(
                "🚀 ฐานข้อมูล `{}` พร้อมใช้งานและตั้งค่า Collection พื้นฐานเรียบร้อยแล้ว",
                database
            );
        }
        Cli::Import { file, database } => {
            let conn = db::open(&database)?;

            // รัน schema ก่อน (ถ้ายังไม่มี)
            let schema = include_str!("../schema.sql");
            db::run_schema(&conn, schema)?;

            let md = fs::read_to_string(&file)?;
            let imported = markdown::import_markdown(&conn, &md, None)?;
            println!(
                "✅ นำเข้า `{}` แล้ว → เอกสาร ID: {} (ชื่อ: {})",
                file.display(),
                imported.document_id,
                imported.title,
            );
        }
    }

    Ok(())
}
