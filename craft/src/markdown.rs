use anyhow::Result;
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use rusqlite::Connection;
use uuid::Uuid;

/// บล็อกที่ยังไม่ถูกเขียนลง DB
#[derive(Debug, Clone)]
pub struct RawBlock {
    pub id: String,
    pub block_type: String,
    pub content: String,
    pub extra: serde_json::Value,
    pub children: Vec<RawBlock>,
}

/// ผลลัพธ์การนำเข้า
#[derive(Debug)]
pub struct ImportedDocument {
    pub document_id: String,
    pub title: String,
}

/// สร้าง UUID แบบไม่มีขีด
fn new_id() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}

/// แยก frontmatter YAML (ระหว่าง `---`) และเนื้อหา Markdown
fn split_frontmatter(input: &str) -> (Option<serde_yaml::Value>, String) {
    let trimmed = input.trim();
    if let Some(rest) = trimmed.strip_prefix("---\n") {
        if let Some((yaml_part, md)) = rest.split_once("---\n") {
            if let Ok(meta) = serde_yaml::from_str::<serde_yaml::Value>(yaml_part) {
                return (Some(meta), md.trim().to_string());
            }
        }
    }
    (None, input.to_string())
}

/// แปลง Markdown เป็นบล็อก tree
pub fn parse_markdown_to_blocks(markdown: &str) -> Vec<RawBlock> {
    let parser = Parser::new_ext(markdown, pulldown_cmark::Options::all());

    let mut root_blocks: Vec<RawBlock> = Vec::new();
    let mut stack: Vec<RawBlock> = Vec::new();
    let mut current_text = String::new();

    let close_block = |text: &mut String, stack: &mut Vec<RawBlock>, roots: &mut Vec<RawBlock>| {
        if let Some(mut block) = stack.pop() {
            block.content = text.clone();
            text.clear();
            if let Some(parent) = stack.last_mut() {
                parent.children.push(block);
            } else {
                roots.push(block);
            }
        }
    };

    for event in parser {
        match event {
            Event::Start(tag) => {
                let (block_type, extra) = match &tag {
                    Tag::Heading { level, .. } => {
                        ("heading", serde_json::json!({"level": *level as u8}))
                    }
                    Tag::Paragraph => ("text", serde_json::json!({})),
                    Tag::CodeBlock(kind) => {
                        let lang = match kind {
                            pulldown_cmark::CodeBlockKind::Fenced(l) => l.to_string(),
                            pulldown_cmark::CodeBlockKind::Indented => String::new(),
                        };
                        ("code", serde_json::json!({"language": lang}))
                    }
                    Tag::BlockQuote => ("quote", serde_json::json!({})),
                    Tag::List(first_number) => {
                        if first_number.is_some() {
                            ("numbered_list", serde_json::json!({}))
                        } else {
                            ("bulleted_list", serde_json::json!({}))
                        }
                    }
                    Tag::Item => ("list_item", serde_json::json!({})),
                    _ => continue,
                };

                stack.push(RawBlock {
                    id: new_id(),
                    block_type: block_type.to_string(),
                    content: String::new(),
                    extra,
                    children: Vec::new(),
                });
            }
            Event::End(
                TagEnd::Heading(_)
                | TagEnd::Paragraph
                | TagEnd::CodeBlock
                | TagEnd::BlockQuote
                | TagEnd::List(_)
                | TagEnd::Item,
            ) => {
                close_block(&mut current_text, &mut stack, &mut root_blocks);
            }
            Event::Text(text) | Event::Code(text) => {
                current_text.push_str(&text);
            }
            Event::SoftBreak | Event::HardBreak => {
                current_text.push('\n');
            }
            _ => {}
        }
    }

    root_blocks
}

/// เขียนบล็อก tree ลงฐานข้อมูล
fn insert_blocks(
    conn: &Connection,
    document_id: &str,
    blocks: &[RawBlock],
    parent_id: Option<&str>,
) -> Result<()> {
    for (idx, block) in blocks.iter().enumerate() {
        conn.execute(
            "INSERT INTO blocks (id, document_id, parent_id, type, content, extra, index_position)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                block.id,
                document_id,
                parent_id,
                block.block_type,
                block.content,
                serde_json::to_string(&block.extra)?,
                idx as i32,
            ],
        )?;

        if !block.children.is_empty() {
            insert_blocks(conn, document_id, &block.children, Some(&block.id))?;
        }
    }
    Ok(())
}

/// นำเข้า Markdown ทั้งหมดเข้าสู่ฐานข้อมูล
pub fn import_markdown(
    conn: &Connection,
    markdown: &str,
    folder_id: Option<&str>,
) -> Result<ImportedDocument> {
    let (frontmatter, body) = split_frontmatter(markdown);

    // หา title
    let title = frontmatter
        .as_ref()
        .and_then(|fm| fm.get("title"))
        .and_then(|t| t.as_str())
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            body.lines()
                .next()
                .unwrap_or("Untitled")
                .trim_start_matches("# ")
                .to_string()
        });

    // metadata
    let metadata = serde_json::to_string(&frontmatter.unwrap_or(serde_yaml::Value::Null))?;

    // สร้างเอกสาร
    let doc_id = new_id();
    conn.execute(
        "INSERT INTO documents (id, title, folder_id, metadata) VALUES (?1, ?2, ?3, ?4)",
        rusqlite::params![doc_id, title, folder_id, metadata],
    )?;

    // แปลงและบันทึกบล็อก
    let blocks = parse_markdown_to_blocks(&body);
    insert_blocks(conn, &doc_id, &blocks, None)?;

    Ok(ImportedDocument {
        document_id: doc_id,
        title,
    })
}
