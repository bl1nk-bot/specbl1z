use anyhow::{anyhow, Context, Result};
use rusqlite::{params, Connection, Transaction};
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::bl1nk::MemoryCategory;
use crate::bl1nk::MemoryEntry as ProtoMemoryEntry;
use crate::bl1nk::MemoryScope;

/// MemoryStore is the main interface for memory operations.
/// Backend: SQLite (rusqlite). Supports filtering, linking, audit logging.
pub struct MemoryStore {
    conn: Connection,
}

/// Query filter for memory retrieval
#[derive(Debug, Clone, Default)]
pub struct MemoryQuery {
    pub scope: Option<String>,
    pub category: Option<String>,
    pub min_confidence: Option<f32>,
    pub tags: Vec<String>,
    pub owner: Option<String>,
    pub status: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl MemoryStore {
    /// Open or create a memory database at the given path.
    /// Initializes tables if they don't exist.
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path).context("Failed to open memory database")?;
        let db = MemoryStore { conn };
        db.init_tables()?;
        Ok(db)
    }

    /// Initialize all v2 tables if they don't exist.
    fn init_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS memory_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                scope TEXT NOT NULL CHECK(scope IN ('global', 'project', 'session', 'working', 'policy', 'identity')),
                category TEXT NOT NULL CHECK(category IN ('fact', 'preference', 'history', 'context', 'inference')),
                key TEXT NOT NULL,
                value TEXT NOT NULL,
                source TEXT,
                confidence REAL DEFAULT 0.5 CHECK(confidence >= 0.0 AND confidence <= 1.0),
                created_at INTEGER DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER DEFAULT (strftime('%s', 'now')),
                version INTEGER DEFAULT 1,
                status TEXT DEFAULT 'active',
                tags JSON DEFAULT '[]',
                owner TEXT,
                access_level TEXT DEFAULT 'private',
                provenance JSON,
                expires_at INTEGER,
                UNIQUE(key, scope, owner) ON CONFLICT REPLACE
            )", []
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS memory_links (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                from_entry_id INTEGER NOT NULL REFERENCES memory_entries(id) ON DELETE CASCADE,
                to_entry_id INTEGER NOT NULL REFERENCES memory_entries(id) ON DELETE CASCADE,
                relation_type TEXT NOT NULL,
                metadata JSON DEFAULT '{}',
                created_at INTEGER DEFAULT (strftime('%s', 'now')),
                UNIQUE(from_entry_id, to_entry_id, relation_type)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS memory_audit_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                entry_id INTEGER NOT NULL REFERENCES memory_entries(id) ON DELETE CASCADE,
                operation TEXT NOT NULL,
                performed_by TEXT NOT NULL,
                timestamp INTEGER DEFAULT (strftime('%s', 'now')),
                old_value JSON,
                new_value JSON
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS prompt_blocks (
                id TEXT PRIMARY KEY,
                type TEXT NOT NULL,
                priority INTEGER DEFAULT 5,
                scope TEXT NOT NULL,
                content TEXT NOT NULL,
                source TEXT,
                constraints JSON,
                dependencies JSON DEFAULT '[]',
                version INTEGER DEFAULT 1,
                created_at INTEGER DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER DEFAULT (strftime('%s', 'now'))
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS context_packs (
                session_id TEXT PRIMARY KEY,
                blocks_json JSON NOT NULL,
                assembled_at INTEGER DEFAULT (strftime('%s', 'now')),
                trace_id TEXT,
                total_tokens_estimate INTEGER DEFAULT 0
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS conflict_records (
                id TEXT PRIMARY KEY,
                conflicting_block_ids JSON NOT NULL,
                conflict_type TEXT NOT NULL,
                description TEXT,
                resolution TEXT,
                resolved_by TEXT,
                resolved_at INTEGER,
                is_resolved BOOLEAN DEFAULT FALSE,
                created_at INTEGER DEFAULT (strftime('%s', 'now'))
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS policy_rules (
                id TEXT PRIMARY KEY,
                rule_type TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                condition TEXT,
                action TEXT NOT NULL,
                priority INTEGER DEFAULT 5,
                scope TEXT,
                enabled BOOLEAN DEFAULT TRUE,
                effective_from INTEGER DEFAULT (strftime('%s', 'now')),
                effective_until INTEGER,
                created_at INTEGER DEFAULT (strftime('%s', 'now')),
                updated_at INTEGER DEFAULT (strftime('%s', 'now'))
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS policy_evaluations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                context_id TEXT NOT NULL,
                rule_id TEXT NOT NULL REFERENCES policy_rules(id),
                allowed BOOLEAN NOT NULL,
                modification JSON,
                reason TEXT,
                evaluated_at INTEGER DEFAULT (strftime('%s', 'now'))
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS test_plans (
                id TEXT PRIMARY KEY,
                template_id TEXT NOT NULL,
                version TEXT DEFAULT '1.0',
                suites JSON NOT NULL,
                test_cases JSON NOT NULL,
                coverage_target REAL DEFAULT 0.8,
                generated_by TEXT NOT NULL,
                generated_at INTEGER DEFAULT (strftime('%s', 'now')),
                llm_model_used TEXT,
                status TEXT DEFAULT 'draft'
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS work_instructions (
                id TEXT PRIMARY KEY,
                template_id TEXT NOT NULL,
                test_plan_id TEXT NOT NULL,
                format TEXT NOT NULL,
                content TEXT NOT NULL,
                version TEXT DEFAULT '1.0',
                created_at INTEGER DEFAULT (strftime('%s', 'now')),
                approved_by TEXT,
                approved_at INTEGER
            )",
            [],
        )?;

        // Indices
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_memory_entries_scope ON memory_entries(scope)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_memory_entries_category ON memory_entries(category)",
            [],
        )?;
        self.conn.execute("CREATE INDEX IF NOT EXISTS idx_memory_entries_confidence ON memory_entries(confidence)", [])?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_memory_entries_owner ON memory_entries(owner)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_memory_links_from ON memory_links(from_entry_id)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_memory_links_to ON memory_links(to_entry_id)",
            [],
        )?;
        self.conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_policy_rules_type ON policy_rules(rule_type)",
            [],
        )?;

        Ok(())
    }

    /// Insert a memory entry, returning its assigned ID.
    /// Performs policy check before write (FR2.5).
    pub fn insert(&self, entry: ProtoMemoryEntry) -> Result<i64> {
        // FR2.5: Prevent identity memory overwrite (identity scope is read-only after creation)
        if entry.scope == MemoryScope::ScopeIdentity as i32 {
            return Err(anyhow!(
                "Identity memory is protected; cannot insert identity entries directly"
            ));
        }

        let now = current_timestamp()?;
        // Borrow fields to avoid partial moves; entry is used later in log_audit
        self.conn.execute(
            "INSERT INTO memory_entries (scope, category, key, value, source, confidence, created_at, updated_at, version, status, tags, owner, access_level, provenance, expires_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
            params![
                scope_to_string(entry.scope)?,
                category_to_string(entry.category)?,
                &entry.key,
                &entry.value,
                entry.source.as_deref(),
                entry.confidence,
                entry.created_at.max(now),
                entry.updated_at.max(now),
                entry.version,
                &entry.status,
                serde_json::to_string(&entry.tags).unwrap_or("[]".to_string()),
                entry.owner.as_deref(),
                "private",
                entry.provenance.as_deref(),
                entry.expires_at,
            ],
        )?;
        let id = self.conn.last_insert_rowid();
        let performer = entry.owner.clone().unwrap_or_default();
        self.log_audit(id, "insert", &performer, None, Some(&entry))?;
        Ok(id)
    }

    /// Get a memory entry by ID
    pub fn get_by_id(&self, id: i64) -> Result<Option<ProtoMemoryEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, scope, category, key, value, source, confidence, created_at, updated_at, version, status, tags, owner, access_level, provenance, expires_at
             FROM memory_entries WHERE id = ?1"
        )?;
        let mut rows = stmt.query_map([id], |row| {
            let scope_str: String = row.get(1)?;
            let category_str: String = row.get(2)?;
            Ok(ProtoMemoryEntry {
                id: Some(row.get(0)?),
                scope: string_to_memory_scope(&scope_str).unwrap_or(MemoryScope::ScopeGlobal)
                    as i32,
                category: string_to_memory_category(&category_str)
                    .unwrap_or(MemoryCategory::CategoryFact) as i32,
                key: row.get(3)?,
                value: row.get(4)?,
                source: row.get(5)?,
                confidence: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
                version: row.get(9)?,
                status: row.get(10)?,
                tags: serde_json::from_str::<Vec<String>>(&row.get::<_, String>(11)?)
                    .unwrap_or_default(),
                owner: row.get(12)?,
                access_level: row.get(13)?,
                provenance: row.get::<_, Option<String>>(14)?,
                expires_at: row.get(15)?,
            })
        })?;
        if let Some(row) = rows.next() {
            row.map(Some).map_err(|e| e.into())
        } else {
            Ok(None)
        }
    }

    /// Query memory entries with optional filters.
    /// Returns Vec of entries sorted by confidence descending.
    pub fn query(&self, filter: &MemoryQuery) -> Result<Vec<ProtoMemoryEntry>> {
        let mut sql = String::from(
            "SELECT id, scope, category, key, value, source, confidence, created_at, updated_at, version, status, tags, owner, access_level, provenance, expires_at
             FROM memory_entries WHERE 1=1"
        );
        let mut args: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(ref scope) = filter.scope {
            sql.push_str(" AND scope = ?");
            args.push(Box::new(scope.clone()));
        }
        if let Some(ref cat) = filter.category {
            sql.push_str(" AND category = ?");
            args.push(Box::new(cat.clone()));
        }
        if let Some(min_conf) = filter.min_confidence {
            sql.push_str(" AND confidence >= ?");
            args.push(Box::new(min_conf));
        }
        if let Some(ref owner) = filter.owner {
            sql.push_str(" AND owner = ?");
            args.push(Box::new(owner.clone()));
        }
        if let Some(ref status) = filter.status {
            sql.push_str(" AND status = ?");
            args.push(Box::new(status.clone()));
        }
        if !filter.tags.is_empty() {
            for tag in &filter.tags {
                sql.push_str(" AND tags LIKE ?");
                args.push(Box::new(format!("%{}%", tag)));
            }
        }

        sql.push_str(" ORDER BY confidence DESC, created_at DESC");
        if let Some(limit) = filter.limit {
            sql.push_str(" LIMIT ?");
            args.push(Box::new(limit as i64));
        }
        if let Some(offset) = filter.offset {
            sql.push_str(" OFFSET ?");
            args.push(Box::new(offset as i64));
        }

        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt.query_map(
            rusqlite::params_from_iter(args.iter().map(|a| &**a)),
            |row| {
                let scope_str: String = row.get(1)?;
                let category_str: String = row.get(2)?;
                Ok(ProtoMemoryEntry {
                    id: Some(row.get(0)?),
                    scope: string_to_memory_scope(&scope_str).unwrap_or(MemoryScope::ScopeGlobal)
                        as i32,
                    category: string_to_memory_category(&category_str)
                        .unwrap_or(MemoryCategory::CategoryFact)
                        as i32,
                    key: row.get(3)?,
                    value: row.get(4)?,
                    source: row.get(5)?,
                    confidence: row.get(6)?,
                    created_at: row.get(7)?,
                    updated_at: row.get(8)?,
                    version: row.get(9)?,
                    status: row.get(10)?,
                    tags: serde_json::from_str::<Vec<String>>(&row.get::<_, String>(11)?)
                        .unwrap_or_default(),
                    owner: row.get(12)?,
                    access_level: row.get(13)?,
                    provenance: row.get::<_, Option<String>>(14)?,
                    expires_at: row.get(15)?,
                })
            },
        )?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Update an existing memory entry (full replace).
    /// Version is incremented automatically.
    pub fn update(&self, entry: ProtoMemoryEntry) -> Result<()> {
        let id = entry.id.ok_or_else(|| anyhow!("Missing id for update"))?;
        let old_entry = self.get_by_id(id)?;

        self.conn.execute(
            "UPDATE memory_entries SET
                scope = ?1, category = ?2, key = ?3, value = ?4, source = ?5,
                confidence = ?6, updated_at = ?7, version = version + 1,
                status = ?8, tags = ?9, owner = ?10, access_level = ?11,
                provenance = ?12, expires_at = ?13
             WHERE id = ?14",
            params![
                scope_to_string(entry.scope)?,
                category_to_string(entry.category)?,
                &entry.key,
                &entry.value,
                entry.source.as_deref(),
                entry.confidence,
                current_timestamp()?,
                &entry.status,
                serde_json::to_string(&entry.tags).unwrap_or("[]".to_string()),
                entry.owner.as_deref(),
                &entry.access_level,
                entry.provenance.as_deref(),
                entry.expires_at,
                id,
            ],
        )?;

        let performer = entry.owner.clone().unwrap_or_default();
        self.log_audit(id, "update", &performer, old_entry.as_ref(), Some(&entry))?;
        Ok(())
    }

    /// Delete a memory entry by ID (soft-delete: set status='archived')
    pub fn delete(&self, id: i64) -> Result<()> {
        self.conn.execute(
            "UPDATE memory_entries SET status = 'archived', updated_at = ?1 WHERE id = ?2",
            params![current_timestamp()?, id],
        )?;
        self.log_audit(id, "delete", "system", None, None)?;
        Ok(())
    }

    /// Link two memory entries with a relation type.
    pub fn link(
        &self,
        from_id: i64,
        to_id: i64,
        relation_type: &str,
        metadata: Option<Value>,
    ) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO memory_links (from_entry_id, to_entry_id, relation_type, metadata)
             VALUES (?1, ?2, ?3, ?4)",
            params![
                from_id,
                to_id,
                relation_type,
                metadata.map(|v| serde_json::to_string(&v).unwrap_or_default())
            ],
        )?;
        let id = self.conn.last_insert_rowid();
        Ok(id)
    }

    /// Unlink two entries (delete the link)
    pub fn unlink(&self, from_id: i64, to_id: i64, relation_type: &str) -> Result<()> {
        self.conn
            .execute(
                "DELETE FROM memory_links WHERE from_entry_id = ?1 AND to_entry_id = ?2 AND relation_type = ?3",
                params![from_id, to_id, relation_type],
            )?;
        Ok(())
    }

    /// Get linked entries for a given entry ID.
    pub fn get_links(
        &self,
        entry_id: i64,
        direction: LinkDirection,
    ) -> Result<Vec<(i64, String, Value)>> {
        let sql = match direction {
            LinkDirection::Outgoing => "SELECT to_entry_id, relation_type, metadata FROM memory_links WHERE from_entry_id = ?1",
            LinkDirection::Incoming => "SELECT from_entry_id, relation_type, metadata FROM memory_links WHERE to_entry_id = ?1",
        };
        let mut stmt = self.conn.prepare(sql)?;
        let rows = stmt.query_map([entry_id], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get::<_, Option<String>>(2)?
                    .and_then(|s| serde_json::from_str(&s).ok())
                    .unwrap_or(json!({})),
            ))
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Get audit log entries for a given memory entry.
    pub fn audit_trail(&self, entry_id: i64) -> Result<Vec<AuditEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, entry_id, operation, performed_by, timestamp, old_value, new_value
             FROM memory_audit_log WHERE entry_id = ?1 ORDER BY timestamp DESC, id DESC",
        )?;
        let rows = stmt.query_map([entry_id], |row| {
            Ok(AuditEntry {
                id: row.get(0)?,
                entry_id: row.get(1)?,
                operation: row.get(2)?,
                performed_by: row.get(3)?,
                timestamp: row.get(4)?,
                old_value: row.get(5)?,
                new_value: row.get(6)?,
            })
        })?;
        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    /// Clean up expired entries (TTL). Returns count deleted.
    pub fn cleanup_expired(&self) -> Result<i64> {
        let now = current_timestamp()?;
        let mut stmt = self.conn.prepare(
            "DELETE FROM memory_entries WHERE expires_at IS NOT NULL AND expires_at < ?1",
        )?;
        let count = stmt.execute([now])?;
        Ok(count as i64)
    }

    /// Transactional batch insert (for bulk operations)
    pub fn transaction(&mut self) -> Result<Transaction<'_>> {
        self.conn
            .transaction()
            .map_err(|e| anyhow::anyhow!("Transaction failed: {}", e))
    }

    /// Internal: write audit log entry
    fn log_audit(
        &self,
        entry_id: i64,
        operation: &str,
        performer: &str,
        old: Option<&ProtoMemoryEntry>,
        new: Option<&ProtoMemoryEntry>,
    ) -> Result<()> {
        // Convert entry to serde_json::Value manually (ProtoMemoryEntry may not derive Serialize)
        let entry_to_value = |e: &ProtoMemoryEntry| -> Value {
            json!({
                "id": e.id,
                "scope": scope_to_string(e.scope).ok(),
                "category": category_to_string(e.category).ok(),
                "key": &e.key,
                "value": &e.value,
                "source": &e.source,
                "confidence": e.confidence,
                "created_at": e.created_at,
                "updated_at": e.updated_at,
                "version": e.version,
                "status": &e.status,
                "tags": &e.tags,
                "owner": &e.owner,
                "access_level": &e.access_level,
                "provenance": &e.provenance,
                "expires_at": e.expires_at,
            })
        };
        let old_val = old.map(entry_to_value);
        let new_val = new.map(entry_to_value);
        self.conn.execute(
            "INSERT INTO memory_audit_log (entry_id, operation, performed_by, old_value, new_value)
                 VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                entry_id,
                operation,
                performer,
                old_val
                    .as_ref()
                    .map(|v| serde_json::to_string(v).unwrap_or_default()),
                new_val
                    .as_ref()
                    .map(|v| serde_json::to_string(v).unwrap_or_default())
            ],
        )?;
        Ok(())
    }
}

/// Direction for link traversal
#[derive(Debug, Clone, Copy)]
pub enum LinkDirection {
    Outgoing, // from_entry_id -> to_entry_id
    Incoming, // to_entry_id <- from_entry_id
}

/// Audit entry record
#[derive(Debug, Clone)]
pub struct AuditEntry {
    pub id: i64,
    pub entry_id: i64,
    pub operation: String,
    pub performed_by: String,
    pub timestamp: i64,
    pub old_value: Option<Value>,
    pub new_value: Option<Value>,
}

// ==================== Helper Functions ====================

pub fn current_timestamp() -> Result<i64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .map_err(|e| anyhow!("Failed to get timestamp: {}", e))
}

pub fn scope_to_string(scope: i32) -> Result<String> {
    match MemoryScope::try_from(scope) {
        Ok(MemoryScope::ScopeGlobal) => Ok("global".to_string()),
        Ok(MemoryScope::ScopeProject) => Ok("project".to_string()),
        Ok(MemoryScope::ScopeSession) => Ok("session".to_string()),
        Ok(MemoryScope::ScopeWorking) => Ok("working".to_string()),
        Ok(MemoryScope::ScopePolicy) => Ok("policy".to_string()),
        Ok(MemoryScope::ScopeIdentity) => Ok("identity".to_string()),
        Err(_) => Err(anyhow!("Invalid MemoryScope value: {}", scope)),
    }
}

pub fn string_to_memory_scope(s: &str) -> Option<MemoryScope> {
    match s {
        "global" => Some(MemoryScope::ScopeGlobal),
        "project" => Some(MemoryScope::ScopeProject),
        "session" => Some(MemoryScope::ScopeSession),
        "working" => Some(MemoryScope::ScopeWorking),
        "policy" => Some(MemoryScope::ScopePolicy),
        "identity" => Some(MemoryScope::ScopeIdentity),
        _ => None,
    }
}

pub fn category_to_string(cat: i32) -> Result<String> {
    match MemoryCategory::try_from(cat) {
        Ok(MemoryCategory::CategoryFact) => Ok("fact".to_string()),
        Ok(MemoryCategory::CategoryPreference) => Ok("preference".to_string()),
        Ok(MemoryCategory::CategoryHistory) => Ok("history".to_string()),
        Ok(MemoryCategory::CategoryContext) => Ok("context".to_string()),
        Ok(MemoryCategory::CategoryInference) => Ok("inference".to_string()),
        Err(_) => Err(anyhow!("Invalid MemoryCategory value: {}", cat)),
    }
}

pub fn string_to_memory_category(s: &str) -> Option<MemoryCategory> {
    match s {
        "fact" => Some(MemoryCategory::CategoryFact),
        "preference" => Some(MemoryCategory::CategoryPreference),
        "history" => Some(MemoryCategory::CategoryHistory),
        "context" => Some(MemoryCategory::CategoryContext),
        "inference" => Some(MemoryCategory::CategoryInference),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_memory_store_crud() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let entry = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeProject as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "test_key".to_string(),
            value: "test_value".to_string(),
            source: Some("test".to_string()),
            confidence: 0.9,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".to_string(),
            tags: vec!["test".to_string()],
            owner: Some("user1".to_string()),
            access_level: "project".to_string(),
            provenance: None,
            expires_at: None,
        };
        let id = store.insert(entry.clone()).unwrap();
        assert!(id > 0);

        let retrieved = store.get_by_id(id).unwrap().unwrap();
        assert_eq!(retrieved.key, "test_key");
        assert_eq!(retrieved.value, "test_value");

        let results = store
            .query(&MemoryQuery {
                scope: Some("project".to_string()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(results.len(), 1);

        let mut upd = entry.clone();
        upd.id = Some(id);
        upd.value = "updated_value".to_string();
        store.update(upd).unwrap();
        let updated = store.get_by_id(id).unwrap().unwrap();
        assert_eq!(updated.value, "updated_value");
        assert_eq!(updated.version, 2);

        store.delete(id).unwrap();
        let deleted = store.get_by_id(id).unwrap().unwrap();
        assert_eq!(deleted.status, "archived");

        let audit = store.audit_trail(id).unwrap();
        assert!(!audit.is_empty());
    }

    #[test]
    fn test_identity_scope_protected() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let entry = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeIdentity as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "identity_key".to_string(),
            value: "identity_value".to_string(),
            source: None,
            confidence: 1.0,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".to_string(),
            tags: vec![],
            owner: None,
            access_level: "private".to_string(),
            provenance: None,
            expires_at: None,
        };
        let result = store.insert(entry);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("protected"));
    }

    #[test]
    fn test_link_entries() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let e1 = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "a".to_string(),
            value: "1".to_string(),
            source: None,
            confidence: 0.5,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".to_string(),
            tags: vec![],
            owner: None,
            access_level: "public".to_string(),
            provenance: None,
            expires_at: None,
        };
        let e2 = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "b".to_string(),
            value: "2".to_string(),
            source: None,
            confidence: 0.5,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".to_string(),
            tags: vec![],
            owner: None,
            access_level: "public".to_string(),
            provenance: None,
            expires_at: None,
        };
        let id1 = store.insert(e1).unwrap();
        let id2 = store.insert(e2).unwrap();

        let link_id = store.link(id1, id2, "references", None).unwrap();
        assert!(link_id > 0);

        let out = store.get_links(id1, LinkDirection::Outgoing).unwrap();
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].0, id2);
        assert_eq!(out[0].1, "references");

        store.unlink(id1, id2, "references").unwrap();
        let out2 = store.get_links(id1, LinkDirection::Outgoing).unwrap();
        assert_eq!(out2.len(), 0);
    }

    #[test]
    fn test_query_filters() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let now = current_timestamp().unwrap();
        // Insert entries with varying fields
        let e1 = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeProject as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "k1".into(),
            value: "v1".into(),
            source: Some("src1".into()),
            confidence: 0.8,
            created_at: now,
            updated_at: now,
            version: 1,
            status: "active".into(),
            tags: vec!["a".into(), "b".into()],
            owner: Some("alice".into()),
            access_level: "team".into(),
            provenance: None,
            expires_at: None,
        };
        let e2 = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryHistory as i32,
            key: "k2".into(),
            value: "v2".into(),
            source: Some("src2".into()),
            confidence: 0.4,
            created_at: now,
            updated_at: now,
            version: 1,
            status: "archived".into(),
            tags: vec!["b".into()],
            owner: Some("bob".into()),
            access_level: "public".into(),
            provenance: None,
            expires_at: None,
        };
        let id1 = store.insert(e1).unwrap();
        let id2 = store.insert(e2).unwrap();

        // Filter by scope (exact match string)
        let res = store
            .query(&MemoryQuery {
                scope: Some("project".into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, Some(id1));

        // Filter by category
        let res = store
            .query(&MemoryQuery {
                category: Some("history".into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, Some(id2));

        // Filter by owner
        let res = store
            .query(&MemoryQuery {
                owner: Some("alice".into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, Some(id1));

        // Filter by min_confidence
        let res = store
            .query(&MemoryQuery {
                min_confidence: Some(0.5),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, Some(id1));

        // Filter by status
        let res = store
            .query(&MemoryQuery {
                status: Some("archived".into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, Some(id2));

        // Tag filtering: entries with tag "b"
        let res = store
            .query(&MemoryQuery {
                tags: vec!["b".into()],
                ..Default::default()
            })
            .unwrap();
        assert_eq!(res.len(), 2); // both contain b

        // Limit and offset
        let res = store
            .query(&MemoryQuery {
                limit: Some(1),
                offset: Some(0),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(res.len(), 1);
        // The ordering is confidence DESC then created_at DESC, so e1 should be first
        assert_eq!(res[0].id, Some(id1));

        // Verify ordering: confidence 0.8 > 0.4
        let all = store
            .query(&MemoryQuery {
                ..Default::default()
            })
            .unwrap();
        assert_eq!(all[0].id, Some(id1));
        assert_eq!(all[1].id, Some(id2));
    }

    #[test]
    fn test_link_with_metadata() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let e1 = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "x".into(),
            value: "1".into(),
            source: None,
            confidence: 1.0,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".into(),
            tags: vec![],
            owner: None,
            access_level: "public".into(),
            provenance: None,
            expires_at: None,
        };
        let e2 = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "y".into(),
            value: "2".into(),
            source: None,
            confidence: 1.0,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".into(),
            tags: vec![],
            owner: None,
            access_level: "public".into(),
            provenance: None,
            expires_at: None,
        };
        let id1 = store.insert(e1).unwrap();
        let id2 = store.insert(e2).unwrap();

        let meta = json!({ "since": "2025-01-01", "strength": 0.7 });
        let link_id = store.link(id1, id2, "related", Some(meta.clone())).unwrap();
        assert!(link_id > 0);

        // Retrieve link and check metadata stored correctly
        let links = store.get_links(id1, LinkDirection::Outgoing).unwrap();
        assert_eq!(links.len(), 1);
        assert_eq!(links[0].0, id2);
        // metadata stored as TEXT, should deserialize back to json value
        // Instead we can query memory_links directly via a method or raw query
        // But we don't have a public get_link_metadata. We'll use raw query for test
        let row: Option<String> = store
            .conn
            .query_row(
                "SELECT metadata FROM memory_links WHERE id = ?1",
                [link_id],
                |row| row.get(0),
            )
            .ok();
        let meta_str = row.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&meta_str).unwrap();
        assert_eq!(parsed, meta);
    }

    #[test]
    fn test_get_links_incoming() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let e1 = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "a".into(),
            value: "1".into(),
            source: None,
            confidence: 0.5,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".into(),
            tags: vec![],
            owner: None,
            access_level: "public".into(),
            provenance: None,
            expires_at: None,
        };
        let e2 = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "b".into(),
            value: "2".into(),
            source: None,
            confidence: 0.5,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".into(),
            tags: vec![],
            owner: None,
            access_level: "public".into(),
            provenance: None,
            expires_at: None,
        };
        let id1 = store.insert(e1).unwrap();
        let id2 = store.insert(e2).unwrap();

        // Link e1 -> e2
        store.link(id1, id2, "child", None).unwrap();
        // Incoming to e2
        let incoming = store.get_links(id2, LinkDirection::Incoming).unwrap();
        assert_eq!(incoming.len(), 1);
        assert_eq!(incoming[0].0, id1);
        assert_eq!(incoming[0].1, "child");
    }

    #[test]
    fn test_cleanup_expired() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let now = current_timestamp().unwrap();
        let past = now - 3600;
        let future = now + 3600;

        let e_expired = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "exp".into(),
            value: "old".into(),
            source: None,
            confidence: 0.5,
            created_at: now,
            updated_at: now,
            version: 1,
            status: "active".into(),
            tags: vec![],
            owner: None,
            access_level: "public".into(),
            provenance: None,
            expires_at: Some(past),
        };
        let e_valid = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "valid".into(),
            value: "new".into(),
            source: None,
            confidence: 0.5,
            created_at: now,
            updated_at: now,
            version: 1,
            status: "active".into(),
            tags: vec![],
            owner: None,
            access_level: "public".into(),
            provenance: None,
            expires_at: Some(future),
        };
        let id_expired = store.insert(e_expired).unwrap();
        let id_valid = store.insert(e_valid).unwrap();

        // Ensure both exist
        assert!(store.get_by_id(id_expired).unwrap().is_some());
        assert!(store.get_by_id(id_valid).unwrap().is_some());

        let deleted = store.cleanup_expired().unwrap();
        assert_eq!(deleted, 1);
        // Expired deleted
        assert!(store.get_by_id(id_expired).unwrap().is_none());
        // Valid still present
        assert!(store.get_by_id(id_valid).unwrap().is_some());
    }

    #[test]
    fn test_transaction_batch_insert() {
        let tmp = NamedTempFile::new().unwrap();
        let mut store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        {
            let tx = store.transaction().unwrap();
            let _now = current_timestamp().unwrap();
            for i in 0..5 {
                let key = format!("batch_{}", i);
                let value = format!("val_{}", i);
                tx.execute(
                    "INSERT INTO memory_entries (scope, category, key, value) VALUES (?1, ?2, ?3, ?4)",
                    params![
                        "global",
                        "fact",
                        key,
                        value
                    ],
                ).unwrap();
            }
            tx.commit().unwrap();
        }

        let all = store.query(&MemoryQuery::default()).unwrap();
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn test_transaction_rollback() {
        let tmp = NamedTempFile::new().unwrap();
        let mut store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        // Insert one entry first
        let e1 = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "persist".into(),
            value: "keep".into(),
            source: None,
            confidence: 1.0,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".into(),
            tags: vec![],
            owner: None,
            access_level: "public".into(),
            provenance: None,
            expires_at: None,
        };
        let id1 = store.insert(e1).unwrap();
        assert_eq!(store.query(&MemoryQuery::default()).unwrap().len(), 1);

        // Start a transaction, insert another, then drop without committing (rollback)
        {
            let tx = store.transaction().unwrap();
            tx.execute(
                "INSERT INTO memory_entries (scope, category, key, value) VALUES (?1, ?2, ?3, ?4)",
                params!["global", "fact", "temp", "rollback"],
            )
            .unwrap();
            // Dropping tx without commit triggers rollback
        }

        // Only the original entry remains
        let all = store.query(&MemoryQuery::default()).unwrap();
        assert_eq!(all.len(), 1);
        assert_eq!(all[0].id, Some(id1));
    }

    #[test]
    fn test_audit_log_contents() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let entry = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeProject as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "audit_key".into(),
            value: "v1".into(),
            source: Some("src".into()),
            confidence: 0.9,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".into(),
            tags: vec!["test".into()],
            owner: Some("tester".into()),
            access_level: "private".into(),
            provenance: None,
            expires_at: None,
        };
        let id = store.insert(entry.clone()).unwrap();

        let audit = store.audit_trail(id).unwrap();
        assert_eq!(audit.len(), 1);
        let entry_log = &audit[0];
        assert_eq!(entry_log.operation, "insert");
        assert_eq!(entry_log.performed_by, "tester");
        // Verify that old_value is None and new_value contains the inserted entry JSON
        let new_val = &entry_log.new_value;
        assert!(new_val.is_some());
        let new_obj = new_val.as_ref().unwrap();
        assert_eq!(new_obj["key"], "audit_key");
        assert_eq!(new_obj["value"], "v1");

        // Update and check another audit entry
        let mut upd = entry.clone();
        upd.id = Some(id);
        upd.value = "v2".into();
        store.update(upd).unwrap();
        let audit2 = store.audit_trail(id).unwrap();
        assert_eq!(audit2.len(), 2);
        let upd_log = &audit2[0];
        assert_eq!(upd_log.operation, "update");
        let old_val = &upd_log.old_value;
        assert!(old_val.is_some());
        assert_eq!(old_val.as_ref().unwrap()["value"], "v1");
        let new_val2 = &upd_log.new_value;
        assert!(new_val2.is_some());
        assert_eq!(new_val2.as_ref().unwrap()["value"], "v2");
    }

    #[test]
    fn test_query_with_multiple_filters() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let now = current_timestamp().unwrap();
        let make_entry = |key: &str,
                          scope: i32,
                          cat: i32,
                          owner: Option<&str>,
                          status: &str,
                          tags: Vec<&str>| {
            ProtoMemoryEntry {
                id: None,
                scope,
                category: cat,
                key: key.into(),
                value: "v".into(),
                source: None,
                confidence: 0.7,
                created_at: now,
                updated_at: now,
                version: 1,
                status: status.into(),
                tags: tags.into_iter().map(String::from).collect(),
                owner: owner.map(String::from),
                access_level: "public".into(),
                provenance: None,
                expires_at: None,
            }
        };
        let id_a = store
            .insert(make_entry(
                "a",
                MemoryScope::ScopeProject as i32,
                MemoryCategory::CategoryFact as i32,
                Some("u1"),
                "active",
                vec!["x", "y"],
            ))
            .unwrap();
        let _id_b = store
            .insert(make_entry(
                "b",
                MemoryScope::ScopeGlobal as i32,
                MemoryCategory::CategoryFact as i32,
                Some("u2"),
                "active",
                vec!["y"],
            ))
            .unwrap();
        let id_c = store
            .insert(make_entry(
                "c",
                MemoryScope::ScopeProject as i32,
                MemoryCategory::CategoryFact as i32,
                Some("u1"),
                "archived",
                vec!["x"],
            ))
            .unwrap();

        // Combine: scope=project, owner=u1
        let res = store
            .query(&MemoryQuery {
                scope: Some("project".into()),
                owner: Some("u1".into()),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(res.len(), 2);
        let ids: Vec<_> = res.iter().map(|e| e.id).collect();
        assert!(ids.contains(&Some(id_a)));
        assert!(ids.contains(&Some(id_c)));

        // Combine: scope=project, status=active, tags contains x
        let res = store
            .query(&MemoryQuery {
                scope: Some("project".into()),
                status: Some("active".into()),
                tags: vec!["x".into()],
                ..Default::default()
            })
            .unwrap();
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].id, Some(id_a));
    }

    #[test]
    fn test_get_links_duplicate_uniqueness() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let e1 = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "e1".into(),
            value: "1".into(),
            source: None,
            confidence: 1.0,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".into(),
            tags: vec![],
            owner: None,
            access_level: "public".into(),
            provenance: None,
            expires_at: None,
        };
        let e2 = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "e2".into(),
            value: "2".into(),
            source: None,
            confidence: 1.0,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".into(),
            tags: vec![],
            owner: None,
            access_level: "public".into(),
            provenance: None,
            expires_at: None,
        };
        let id1 = store.insert(e1).unwrap();
        let id2 = store.insert(e2).unwrap();

        // Link same pair twice should error due to UNIQUE constraint
        store.link(id1, id2, "refs", None).unwrap();
        let result = store.link(id1, id2, "refs", None);
        assert!(result.is_err());

        // Ensure only one link exists
        let links = store.get_links(id1, LinkDirection::Outgoing).unwrap();
        assert_eq!(links.len(), 1);
    }

    #[test]
    fn test_audit_log_empty_after_delete_sequence() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let entry = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeProject as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "temp".into(),
            value: "v".into(),
            source: None,
            confidence: 0.5,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".into(),
            tags: vec![],
            owner: Some("u".into()),
            access_level: "private".into(),
            provenance: None,
            expires_at: None,
        };
        let id = store.insert(entry).unwrap();
        // Delete immediately
        store.delete(id).unwrap();
        let audit = store.audit_trail(id).unwrap();
        // Should have insert + delete (two records)
        assert_eq!(audit.len(), 2);
        let ops: Vec<&str> = audit.iter().map(|a| a.operation.as_str()).collect();
        assert_eq!(ops, vec!["delete", "insert"]);
    }

    #[test]
    fn test_query_pagination() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let now = current_timestamp().unwrap();
        for i in 0..10 {
            let entry = ProtoMemoryEntry {
                id: None,
                scope: MemoryScope::ScopeGlobal as i32,
                category: MemoryCategory::CategoryFact as i32,
                key: format!("k{}", i),
                value: format!("v{}", i),
                source: None,
                confidence: 0.5,
                created_at: now - (10 - i) as i64, // older first
                updated_at: now,
                version: 1,
                status: "active".into(),
                tags: vec![],
                owner: None,
                access_level: "public".into(),
                provenance: None,
                expires_at: None,
            };
            store.insert(entry).unwrap();
        }

        // offset 3 limit 2 => returns ids 4 and 5 in order (confidence same, created_at descending)
        let page = store
            .query(&MemoryQuery {
                limit: Some(2),
                offset: Some(3),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(page.len(), 2);
        // Check keys correspond: created_at DESC => most recent first. Our insert sequence created later entries with larger i (newer). So ordering by created_at DESC yields i=9,8,...,0.
        // Offset 3 means skip first 3 (9,8,7). So page should be 6,5.
        let keys: Vec<String> = page.into_iter().map(|e| e.key).collect();
        assert_eq!(keys, vec!["k6", "k5"]);
    }

    #[test]
    fn test_query_no_results() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let res = store
            .query(&MemoryQuery {
                scope: Some("nonexistent".into()),
                ..Default::default()
            })
            .unwrap();
        assert!(res.is_empty());
    }

    #[test]
    fn test_get_by_id_invalid() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let res = store.get_by_id(99999).unwrap();
        assert!(res.is_none());
    }

    #[test]
    fn test_update_missing_id() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let entry = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "x".into(),
            value: "y".into(),
            source: None,
            confidence: 0.5,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".into(),
            tags: vec![],
            owner: None,
            access_level: "public".into(),
            provenance: None,
            expires_at: None,
        };
        let err = store.update(entry).unwrap_err();
        assert!(err.to_string().contains("Missing id"));
    }

    #[test]
    fn test_unlink_nonexistent() {
        let tmp = NamedTempFile::new().unwrap();
        let store = MemoryStore::new(tmp.path().to_str().unwrap()).unwrap();

        let e = ProtoMemoryEntry {
            id: None,
            scope: MemoryScope::ScopeGlobal as i32,
            category: MemoryCategory::CategoryFact as i32,
            key: "e".into(),
            value: "v".into(),
            source: None,
            confidence: 0.5,
            created_at: current_timestamp().unwrap(),
            updated_at: current_timestamp().unwrap(),
            version: 1,
            status: "active".into(),
            tags: vec![],
            owner: None,
            access_level: "public".into(),
            provenance: None,
            expires_at: None,
        };
        let id = store.insert(e).unwrap();
        // Unlink nonexistent link should be ok (idempotent) or return error? In code, unlink executes DELETE and returns row count, but we ignore row count. Usually it doesn't error if 0 rows affected.
        let res = store.unlink(id, 9999, "nonexistent");
        assert!(res.is_ok()); // no error even if nothing deleted
    }
}
