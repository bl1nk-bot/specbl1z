use anyhow::{anyhow, Context, Result};
use rusqlite::params;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::models::{MemoryCategory, MemoryEntry, MemoryScope, MemoryTopic};
use crate::db::Database;

/// MemoryStore handles Agentic Memory using the unified Database.
/// It supports scopes (global/project/session), confidence scoring, and audit logging.
pub struct MemoryStore<'a> {
    db: &'a Database,
}

/// Query filter for memory retrieval
#[derive(Debug, Clone, Default)]
pub struct MemoryQuery {
    pub scope: Option<String>,
    pub category: Option<String>,
    pub topic: Option<String>,
    pub min_confidence: Option<f32>,
    pub tags: Vec<String>,
    pub owner: Option<String>,
    pub status: Option<String>,
    pub limit: Option<usize>,
    pub offset: Option<usize>,
}

impl<'a> MemoryStore<'a> {
    /// Create a new MemoryStore using a reference to the unified Database.
    pub fn new(db: &'a Database) -> Self {
        MemoryStore { db }
    }

    /// Insert or Update a memory entry with atomicity and policy protection.
    pub fn insert(&self, entry: MemoryEntry) -> Result<i64> {
        // Protect Identity scope (v3 Spec)
        if entry.scope == MemoryScope::ScopeIdentity as i32 {
            return Err(anyhow!(
                "Security Violation: Identity memory is immutable after creation"
            ));
        }

        let now = current_timestamp()?;
        self.db.conn.execute(
            "INSERT INTO memory_entries (scope, category, topic, key, value, source, confidence, created_at, updated_at, version, status, tags, owner, access_level, provenance, expires_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)
             ON CONFLICT(key, scope, owner) DO UPDATE SET 
                value = excluded.value, 
                tags = excluded.tags,
                confidence = excluded.confidence,
                updated_at = strftime('%s', 'now'),
                version = version + 1",
            params![
                scope_to_string(entry.scope)?,
                category_to_string(entry.category)?,
                topic_to_string(entry.topic)?,
                &entry.key,
                &entry.value,
                entry.source.as_deref(),
                entry.confidence,
                entry.created_at.max(now),
                entry.updated_at.max(now),
                entry.version,
                &entry.status,
                serde_json::to_string(&entry.tags).unwrap_or_else(|_| "[]".to_string()),
                entry.owner.as_deref(),
                "private",
                entry.provenance.as_deref(),
                entry.expires_at,
            ],
        ).context("MemoryStore: Failed to insert/upsert entry")?;

        let id = self.db.conn.last_insert_rowid();
        let performer = entry.owner.clone().unwrap_or_else(|| "system".to_string());
        self.log_audit(id, "upsert", &performer, None, Some(&entry))?;
        Ok(id)
    }

    /// Retrieve memory with complex filters and confidence sorting (High Spec).
    pub fn query(&self, filter: &MemoryQuery) -> Result<Vec<MemoryEntry>> {
        let mut sql = String::from(
            "SELECT id, scope, category, topic, key, value, source, confidence, created_at, updated_at, version, status, tags, owner, access_level, provenance, expires_at
             FROM memory_entries WHERE status = 'active'"
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
        if let Some(ref topic) = filter.topic {
            sql.push_str(" AND topic = ?");
            args.push(Box::new(topic.clone()));
        }
        if let Some(min_conf) = filter.min_confidence {
            sql.push_str(" AND confidence >= ?");
            args.push(Box::new(min_conf));
        }
        if let Some(ref owner) = filter.owner {
            sql.push_str(" AND owner = ?");
            args.push(Box::new(owner.clone()));
        }

        sql.push_str(" ORDER BY confidence DESC, updated_at DESC");

        if let Some(limit) = filter.limit {
            sql.push_str(&format!(" LIMIT {}", limit));
        }

        let mut stmt = self.db.conn.prepare(&sql)?;
        let rows = stmt.query_map(rusqlite::params_from_iter(args), |row| {
            let tags_json: String = row.get(12)?;
            Ok(MemoryEntry {
                id: Some(row.get(0)?),
                scope: string_to_scope(&row.get::<_, String>(1)?).unwrap_or(MemoryScope::ScopeGlobal) as i32,
                category: string_to_category(&row.get::<_, String>(2)?).unwrap_or(MemoryCategory::CategoryFact) as i32,
                topic: string_to_topic(&row.get::<_, String>(3)?).unwrap_or(MemoryTopic::TopicLearn) as i32,
                key: row.get(4)?,
                value: row.get(5)?,
                source: row.get(6)?,
                confidence: row.get(7)?,
                created_at: row.get(8)?,
                updated_at: row.get(9)?,
                version: row.get(10)?,
                status: row.get(11)?,
                tags: serde_json::from_str(&tags_json).unwrap_or_default(),
                owner: row.get(13)?,
                access_level: row.get(14)?,
                provenance: row.get(15)?,
                expires_at: row.get(16)?,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }

    fn log_audit(&self, entry_id: i64, op: &str, actor: &str, _old: Option<&MemoryEntry>, _new: Option<&MemoryEntry>) -> Result<()> {
        let old_json: Option<String> = None;
        let new_json: Option<String> = None;
        
        self.db.conn.execute(
            "INSERT INTO memory_audit_log (entry_id, operation, performed_by, old_value, new_value)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![entry_id, op, actor, old_json, new_json],
        )?;
        Ok(())
    }
}

pub fn current_timestamp() -> Result<i64> {
    Ok(SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .context("Time went backwards")?
        .as_secs() as i64)
}

pub fn scope_to_string(scope: i32) -> Result<String> {
    match MemoryScope::try_from(scope).map_err(|_| anyhow!("Invalid Scope ID: {}", scope))? {
        MemoryScope::ScopeGlobal => Ok("global".into()),
        MemoryScope::ScopeProject => Ok("project".into()),
        MemoryScope::ScopeSession => Ok("session".into()),
        MemoryScope::ScopeWorking => Ok("working".into()),
        MemoryScope::ScopePolicy => Ok("policy".into()),
        MemoryScope::ScopeIdentity => Ok("identity".into()),
    }
}

pub fn string_to_scope(scope: &str) -> Option<MemoryScope> {
    match scope {
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
    match MemoryCategory::try_from(cat).map_err(|_| anyhow!("Invalid Category ID: {}", cat))? {
        MemoryCategory::CategoryFact => Ok("fact".into()),
        MemoryCategory::CategoryPreference => Ok("preference".into()),
        MemoryCategory::CategoryHistory => Ok("history".into()),
        MemoryCategory::CategoryContext => Ok("context".into()),
        MemoryCategory::CategoryInference => Ok("inference".into()),
    }
}

pub fn string_to_category(cat: &str) -> Option<MemoryCategory> {
    match cat {
        "fact" => Some(MemoryCategory::CategoryFact),
        "preference" => Some(MemoryCategory::CategoryPreference),
        "history" => Some(MemoryCategory::CategoryHistory),
        "context" => Some(MemoryCategory::CategoryContext),
        "inference" => Some(MemoryCategory::CategoryInference),
        _ => None,
    }
}

pub fn topic_to_string(topic: i32) -> Result<String> {
    match MemoryTopic::try_from(topic).map_err(|_| anyhow!("Invalid Topic ID: {}", topic))? {
        MemoryTopic::TopicLearn => Ok("LEARN".into()),
        MemoryTopic::TopicWork => Ok("WORK".into()),
        MemoryTopic::TopicTool => Ok("TOOL".into()),
        MemoryTopic::TopicInterest => Ok("INTEREST".into()),
        MemoryTopic::TopicProject => Ok("PROJECT".into()),
        MemoryTopic::TopicIdentify => Ok("IDENTIFY".into()),
        MemoryTopic::TopicIntest => Ok("INTEST".into()),
        MemoryTopic::TopicFm25 => Ok("FM25".into()),
    }
}

pub fn string_to_topic(topic: &str) -> Option<MemoryTopic> {
    match topic {
        "LEARN" => Some(MemoryTopic::TopicLearn),
        "WORK" => Some(MemoryTopic::TopicWork),
        "TOOL" => Some(MemoryTopic::TopicTool),
        "INTEREST" => Some(MemoryTopic::TopicInterest),
        "PROJECT" => Some(MemoryTopic::TopicProject),
        "IDENTIFY" => Some(MemoryTopic::TopicIdentify),
        "INTEST" => Some(MemoryTopic::TopicIntest),
        "FM25" => Some(MemoryTopic::TopicFm25),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    fn setup_db() -> (NamedTempFile, Database) {
        let tmp = NamedTempFile::new().unwrap();
        let db = Database::new(tmp.path().to_str().unwrap()).unwrap();
        (tmp, db)
    }

    #[test]
    fn test_memory_lifecycle() {
        let (_tmp, db) = setup_db();
        let store = MemoryStore::new(&db);

        let entry = MemoryEntry {
            id: None,
            scope: MemoryScope::ScopeProject as i32,
            category: MemoryCategory::CategoryFact as i32,
            topic: MemoryTopic::TopicProject as i32,
            key: "arch_rule".into(),
            value: "Rust First".into(),
            confidence: 1.0,
            status: "active".into(),
            source: None,
            created_at: 0,
            updated_at: 0,
            version: 1,
            tags: vec![],
            owner: None,
            access_level: "private".into(),
            provenance: None,
            expires_at: None,
        };

        let id = store.insert(entry).unwrap();
        assert!(id > 0);

        let query = MemoryQuery { scope: Some("project".into()), ..Default::default() };
        let results = store.query(&query).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].key, "arch_rule");
        assert_eq!(results[0].topic, MemoryTopic::TopicProject as i32);
    }
}

// Alias functions for backward compatibility
pub fn string_to_memory_scope(s: &str) -> Option<MemoryScope> {
    string_to_scope(s)
}

pub fn string_to_memory_category(s: &str) -> Option<MemoryCategory> {
    string_to_category(s)
}
