use rusqlite::{params, Connection, Result};
use uuid::Uuid;

/// สร้าง UUID แบบไม่มีขีด
fn new_id() -> String {
    Uuid::new_v4().to_string().replace("-", "")
}

/// เปิดฐานข้อมูลและรัน schema
pub fn open(db_path: &str) -> Result<Connection> {
    let conn = Connection::open(db_path)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    Ok(conn)
}

/// รัน schema จากไฟล์ SQL
pub fn run_schema(conn: &Connection, sql: &str) -> Result<()> {
    conn.execute_batch(sql)?;
    Ok(())
}

/// สร้างเอกสารใหม่
pub fn create_document(conn: &Connection, title: &str) -> Result<String> {
    let id = new_id();
    conn.execute(
        "INSERT INTO documents (id, title) VALUES (?1, ?2)",
        params![id, title],
    )?;
    Ok(id)
}

/// สร้าง Collection ใหม่
pub fn create_collection(conn: &Connection, document_id: &str, name: &str) -> Result<String> {
    let id = new_id();
    conn.execute(
        "INSERT INTO collections (id, document_id, name) VALUES (?1, ?2, ?3)",
        params![id, document_id, name],
    )?;
    Ok(id)
}

/// เพิ่ม Property ให้ Collection
pub fn add_property(
    conn: &Connection,
    collection_id: &str,
    name: &str,
    prop_type: &str,
    order: i32,
) -> Result<()> {
    let id = new_id();
    conn.execute(
        "INSERT INTO collection_properties (id, collection_id, name, type, order_index)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![id, collection_id, name, prop_type, order],
    )?;
    Ok(())
}

/// รายชื่อเอกสารทั้งหมด
pub fn list_documents(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT title FROM documents")?;
    let rows = stmt.query_map([], |row| row.get(0))?;
    let mut results = Vec::new();
    for row in rows {
        results.push(row?);
    }
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_documents() {
        let conn = open(":memory:").unwrap();
        run_schema(&conn, include_str!("../schema.sql")).unwrap();

        create_document(&conn, "Doc 1").unwrap();
        create_document(&conn, "Doc 2").unwrap();

        let docs = list_documents(&conn).unwrap();
        assert_eq!(docs.len(), 2);
        assert!(docs.contains(&"Doc 1".to_string()));
        assert!(docs.contains(&"Doc 2".to_string()));
    }
}
