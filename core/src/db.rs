use crate::models::{Category, Section, Tier};
use anyhow::{Context, Result as AnyhowResult};
use rusqlite::{Connection, Result};
use std::fs;
use std::path::Path;

pub struct Database {
    pub(crate) conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> AnyhowResult<Self> {
        let conn = Connection::open(path).context("Failed to open database")?;
        let db = Database { conn };
        db.run_migrations().context("Database migration failed")?;
        Ok(db)
    }

    /// ดำเนินการกลุ่มคำสั่งใน Transaction เดียวกัน
    pub fn transaction<F, T>(&mut self, f: F) -> AnyhowResult<T>
    where
        F: FnOnce(&rusqlite::Transaction) -> AnyhowResult<T>,
    {
        let tx = self
            .conn
            .transaction()
            .context("Failed to start transaction")?;
        let result = f(&tx)?;
        tx.commit().context("Failed to commit transaction")?;
        Ok(result)
    }

    /// ดำเนินการ Migration ฐานข้อมูลจากไฟล์ใน core/migrations/
    fn run_migrations(&self) -> AnyhowResult<()> {
        // สร้างตารางสำหรับติดตาม migration หากยังไม่มี
        self.conn
            .execute(
                "CREATE TABLE IF NOT EXISTS schema_migrations (
                version INTEGER PRIMARY KEY NOT NULL,
                description TEXT NOT NULL,
                installed_on TEXT NOT NULL DEFAULT (datetime('now'))
            );",
                [],
            )
            .context("Failed to create schema_migrations table")?;

        // อ่านไฟล์ migration ทั้งหมด
        let mut migrations_dir = "core/migrations";
        if !Path::new(migrations_dir).exists() {
            migrations_dir = "migrations";
        }

        if !Path::new(migrations_dir).exists() {
            return Ok(());
        }

        let mut paths: Vec<_> = fs::read_dir(migrations_dir)
            .context("Failed to read migrations directory")?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|ext| ext == "sql"))
            .collect();
        paths.sort();

        for path in paths {
            let filename = path.file_name().unwrap().to_string_lossy();
            let version_str = filename.split('_').next().unwrap_or("0");
            let version: i32 = version_str.parse().unwrap_or(0);

            let count: i32 = self
                .conn
                .query_row(
                    "SELECT COUNT(*) FROM schema_migrations WHERE version = ?1",
                    [version],
                    |row| row.get(0),
                )
                .context("Failed to query migration status")?;

            if count == 0 {
                let sql = fs::read_to_string(&path)
                    .with_context(|| format!("Failed to read migration file: {}", filename))?;
                self.conn
                    .execute_batch(&sql)
                    .with_context(|| format!("Failed to apply migration: {}", filename))?;
            }
        }
        Ok(())
    }

    // =========================================================================
    // --- Memory Operations ---
    // =========================================================================

    pub fn insert_memory(
        &self,
        scope: &str,
        category: &str,
        key: &str,
        value: &str,
        tags: &[&str],
        owner: Option<&str>,
    ) -> AnyhowResult<i64> {
        let tags_json = serde_json::to_string(tags)?;
        self.conn
            .execute(
                "INSERT INTO memory_entries (scope, category, key, value, tags, owner) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)
             ON CONFLICT(key, scope, owner) DO UPDATE SET 
                value = excluded.value, 
                tags = excluded.tags,
                updated_at = strftime('%s', 'now'),
                version = version + 1",
                rusqlite::params![scope, category, key, value, tags_json, owner],
            )
            .context("Failed to insert memory entry")?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn query_memory(
        &self,
        scope: Option<&str>,
        category: Option<&str>,
        tag: Option<&str>,
    ) -> AnyhowResult<Vec<serde_json::Value>> {
        let mut sql = "SELECT id, scope, category, key, value, tags, owner FROM memory_entries WHERE status = 'active'".to_string();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(s) = scope {
            sql.push_str(" AND scope = ?");
            params.push(Box::new(s.to_string()));
        }
        if let Some(c) = category {
            sql.push_str(" AND category = ?");
            params.push(Box::new(c.to_string()));
        }
        if let Some(t) = tag {
            sql.push_str(" AND EXISTS (SELECT 1 FROM json_each(tags) WHERE json_each.value = ?)");
            params.push(Box::new(t.to_string()));
        }

        let mut stmt = self
            .conn
            .prepare(&sql)
            .context("Failed to prepare memory query")?;
        let rows = stmt.query_map(rusqlite::params_from_iter(params), |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, i64>(0)?,
                "scope": row.get::<_, String>(1)?,
                "category": row.get::<_, String>(2)?,
                "key": row.get::<_, String>(3)?,
                "value": row.get::<_, String>(4)?,
                "tags": serde_json::from_str::<serde_json::Value>(&row.get::<_, String>(5)?).unwrap_or(serde_json::Value::Array(vec![])),
                "owner": row.get::<_, Option<String>>(6)?,
            }))
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    // =========================================================================
    // --- Read Operations ---
    // =========================================================================

    pub fn get_tiers(&self, section_id: i32) -> Result<Vec<Tier>> {
        let mut stmt = self.conn.prepare("SELECT id, section_id, text, tag, code, order_index, is_custom, user_id FROM rules WHERE section_id = ? ORDER BY order_index")?;
        let tier_iter = stmt.query_map([section_id], |row| {
            Ok(Tier {
                id: row.get(0)?,
                section_id: row.get(1)?,
                text: row.get(2)?,
                tag: row.get(3)?,
                code: row.get(4)?,
                order_index: row.get(5)?,
                is_custom: row.get(6)?,
                user_id: row.get(7)?,
            })
        })?;

        let mut results = Vec::new();
        for tier in tier_iter {
            results.push(tier?);
        }
        Ok(results)
    }

    pub fn get_categories(&self) -> Result<Vec<Category>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, key, label, icon, order_index FROM categories ORDER BY order_index",
        )?;
        let cat_iter = stmt.query_map([], |row| {
            Ok(Category {
                id: row.get(0)?,
                key: row.get(1)?,
                label: row.get(2)?,
                icon: row.get(3)?,
                order_index: row.get(4)?,
            })
        })?;

        let mut results = Vec::new();
        for cat in cat_iter {
            results.push(cat?);
        }
        Ok(results)
    }

    pub fn get_sections(&self, category_id: i32) -> Result<Vec<Section>> {
        let mut stmt = self.conn.prepare("SELECT id, category_id, title, icon, color, text_color, order_index FROM sections WHERE category_id = ? ORDER BY order_index")?;
        let section_iter = stmt.query_map([category_id], |row| {
            Ok(Section {
                id: row.get(0)?,
                category_id: row.get(1)?,
                title: row.get(2)?,
                icon: row.get(3)?,
                color: row.get(4)?,
                text_color: row.get(5)?,
                order_index: row.get(6)?,
            })
        })?;

        let mut results = Vec::new();
        for section in section_iter {
            results.push(section?);
        }
        Ok(results)
    }

    // =========================================================================
    // --- Document / Collection / Agent / Skill Operations ---
    // =========================================================================

    pub fn create_document(&self, title: &str) -> AnyhowResult<String> {
        let id = uuid::Uuid::new_v4().to_string();
        self.conn
            .execute(
                "INSERT INTO documents (id, title) VALUES (?1, ?2)",
                rusqlite::params![id, title],
            )
            .context("Failed to create document")?;
        Ok(id)
    }

    pub fn list_documents(&self) -> AnyhowResult<Vec<serde_json::Value>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, title, folder_id, created_at, updated_at FROM documents ORDER BY created_at DESC")
            .context("Failed to prepare document list query")?;
        let rows = stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, String>(0)?,
                "title": row.get::<_, String>(1)?,
                "folder_id": row.get::<_, Option<String>>(2)?,
                "created_at": row.get::<_, String>(3)?,
                "updated_at": row.get::<_, String>(4)?,
            }))
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn create_collection(&self, document_id: &str, name: &str) -> AnyhowResult<String> {
        let id = uuid::Uuid::new_v4().to_string();
        self.conn
            .execute(
                "INSERT INTO collections (id, document_id, name) VALUES (?1, ?2, ?3)",
                rusqlite::params![id, document_id, name],
            )
            .context("Failed to create collection")?;
        Ok(id)
    }

    pub fn add_property(
        &self,
        collection_id: &str,
        name: &str,
        prop_type: &str,
        order_index: i32,
    ) -> AnyhowResult<String> {
        let id = uuid::Uuid::new_v4().to_string();
        self.conn.execute(
            "INSERT INTO collection_properties (id, collection_id, name, type, order_index) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![id, collection_id, name, prop_type, order_index],
        ).context("Failed to add property")?;
        Ok(id)
    }

    pub fn list_agents(&self) -> AnyhowResult<Vec<serde_json::Value>> {
        let table_exists: bool = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='agents'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .map(|c| c > 0)
            .unwrap_or(false);

        if !table_exists {
            return Ok(Vec::new());
        }

        let mut stmt = self
            .conn
            .prepare("SELECT id, name, status FROM agents ORDER BY name")
            .context("Failed to prepare agent list query")?;
        let rows = stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, String>(0)?,
                "name": row.get::<_, String>(1)?,
                "status": row.get::<_, String>(2)?,
            }))
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn list_skills(&self) -> AnyhowResult<Vec<serde_json::Value>> {
        let table_exists: bool = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='skills'",
                [],
                |row| row.get::<_, i32>(0),
            )
            .map(|c| c > 0)
            .unwrap_or(false);

        if !table_exists {
            return Ok(Vec::new());
        }

        let mut stmt = self
            .conn
            .prepare("SELECT id, name, category, status FROM skills ORDER BY name")
            .context("Failed to prepare skill list query")?;
        let rows = stmt.query_map([], |row| {
            Ok(serde_json::json!({
                "id": row.get::<_, String>(0)?,
                "name": row.get::<_, String>(1)?,
                "category": row.get::<_, String>(2)?,
                "status": row.get::<_, String>(3)?,
            }))
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    pub fn table_exists(&self, table_name: &str) -> AnyhowResult<bool> {
        let count: i32 = self
            .conn
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name=?1",
                [table_name],
                |row| row.get(0),
            )
            .context("Failed to check table existence")?;
        Ok(count > 0)
    }

    pub fn count_table_rows(&self, table_name: &str) -> AnyhowResult<i64> {
        let sql = format!("SELECT COUNT(*) FROM \"{}\"", table_name.replace('"', ""));
        let count: i64 = self
            .conn
            .query_row(&sql, [], |row| row.get(0))
            .context(format!("Failed to count rows in table: {}", table_name))?;
        Ok(count)
    }
}
