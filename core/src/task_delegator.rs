use anyhow::{Result, Context};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
    Failed,
}

impl TaskStatus {
    pub fn as_str(&self) -> &str {
        match self {
            TaskStatus::Todo => "todo",
            TaskStatus::InProgress => "in_progress",
            TaskStatus::Done => "done",
            TaskStatus::Failed => "failed",
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub status: String,
    pub schedule: Option<String>,
    pub repeat_rule: Option<String>,
    pub created_at: String,
}

pub struct TaskDelegator {
    conn: Connection,
}

impl TaskDelegator {
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path).context("Failed to open task database")?;
        // Table 'tasks' should already exist from craft/schema.sql, but let's be safe
        conn.execute(
            "CREATE TABLE IF NOT EXISTS tasks (
                id TEXT PRIMARY KEY,
                block_id TEXT UNIQUE,
                title TEXT NOT NULL,
                document_id TEXT,
                due_date TEXT,
                status TEXT DEFAULT 'todo',
                schedule TEXT,
                repeat_rule TEXT,
                created_at TEXT DEFAULT (datetime('now')),
                updated_at TEXT DEFAULT (datetime('now')),
                FOREIGN KEY (block_id) REFERENCES blocks(id) ON DELETE SET NULL,
                FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
            )",
            [],
        )?;
        Ok(Self { conn })
    }

    pub fn add_task(&self, title: &str, schedule: Option<&str>, repeat: Option<&str>) -> Result<String> {
        let id = Uuid::new_v4().to_string().replace("-", "");
        self.conn.execute(
            "INSERT INTO tasks (id, title, status, schedule, repeat_rule) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![id, title, TaskStatus::Todo.as_str(), schedule, repeat],
        )?;
        Ok(id)
    }

    pub fn list_tasks(&self) -> Result<Vec<Task>> {
        let mut stmt = self.conn.prepare("SELECT id, title, status, schedule, repeat_rule, created_at FROM tasks ORDER BY created_at DESC")?;
        let rows = stmt.query_map([], |row| {
            Ok(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                schedule: row.get(3)?,
                repeat_rule: row.get(4)?,
                created_at: row.get(5)?,
            })
        })?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(row?);
        }
        Ok(tasks)
    }

    /// Update the status of a task.
    pub fn update_status(&self, id: &str, status: TaskStatus) -> Result<()> {
        self.conn.execute(
            "UPDATE tasks SET status = ?1 WHERE id = ?2",
            params![status.as_str(), id],
        )?;
        Ok(())
    }

    /// Get a single task by ID.
    pub fn get_task(&self, id: &str) -> Result<Option<Task>> {
        let mut stmt = self.conn.prepare("SELECT id, title, status, schedule, repeat_rule, created_at FROM tasks WHERE id = ?1")?;
        let mut rows = stmt.query(params![id])?;
        if let Some(row) = rows.next()? {
            Ok(Some(Task {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                schedule: row.get(3)?,
                repeat_rule: row.get(4)?,
                created_at: row.get(5)?,
            }))
        } else {
            Ok(None)
        }
    }




}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::time::{SystemTime, UNIX_EPOCH};
    use rusqlite::Connection;

    // Helper: initialize the minimal DB schema required for TaskDelegator
    fn init_full_schema(conn: &Connection) -> rusqlite::Result<()> {
        // Create parent tables first (documents and blocks) with minimal schema
        conn.execute(
            "CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                created_at TEXT DEFAULT (datetime('now'))
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS blocks (
                id TEXT PRIMARY KEY,
                document_id TEXT,
                content TEXT,
                created_at TEXT DEFAULT (datetime('now')),
                FOREIGN KEY (document_id) REFERENCES documents(id) ON DELETE CASCADE
            )",
            [],
        )?;
        // Tasks table is created by TaskDelegator::new, but foreign keys now resolve
        Ok(())
    }

    #[test]
    fn test_add_task() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test_tasks.db");
        let db_file = db_path.to_str().unwrap();

        // Open connection and init parent tables first
        let conn = Connection::open(db_file).unwrap();
        init_full_schema(&conn).unwrap();
        drop(conn); // close so TaskDelegator can open fresh

        let delegator = TaskDelegator::new(db_file).unwrap();

        let id = delegator.add_task("Test Task", Some("daily 09:00"), Some("1d")).unwrap();
        assert!(!id.is_empty());

        let tasks = delegator.list_tasks().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].title, "Test Task");
        assert_eq!(tasks[0].status, "todo");
        assert_eq!(tasks[0].schedule, Some("daily 09:00".to_string()));
    }

    #[test]
    fn test_list_tasks_empty() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("empty_tasks.db");
        let db_file = db_path.to_str().unwrap();

        // Init parent tables
        let conn = Connection::open(db_file).unwrap();
        init_full_schema(&conn).unwrap();
        drop(conn);

        let delegator = TaskDelegator::new(db_file).unwrap();
        let tasks = delegator.list_tasks().unwrap();
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_add_multiple_tasks() {
        use std::time::{SystemTime, UNIX_EPOCH};

        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("multi_tasks.db");
        let db_file = db_path.to_str().unwrap();

        // Init parent tables
        let conn = Connection::open(db_file).unwrap();
        init_full_schema(&conn).unwrap();
        drop(conn);

        let delegator = TaskDelegator::new(db_file).unwrap();

        // Insert Task A with explicit older created_at
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let older_ts = format!("{}", now - 10);
        delegator.conn.execute(
            "INSERT INTO tasks (id, title, status, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![
                Uuid::new_v4().to_string().replace("-", ""),
                "Task A",
                "todo",
                older_ts
            ],
        ).unwrap();

        // Insert Task B with explicit newer created_at
        let newer_ts = format!("{}", now);
        delegator.conn.execute(
            "INSERT INTO tasks (id, title, status, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![
                Uuid::new_v4().to_string().replace("-", ""),
                "Task B",
                "todo",
                newer_ts
            ],
        ).unwrap();

        let tasks = delegator.list_tasks().unwrap();
        assert_eq!(tasks.len(), 2);
        // Verify order: newest first
        assert_eq!(tasks[0].title, "Task B");
        assert_eq!(tasks[1].title, "Task A");
    }

    #[test]
    fn test_task_status_transition() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("status_tasks.db");
        let db_file = db_path.to_str().unwrap();

        // Init parent tables
        let conn = Connection::open(db_file).unwrap();
        init_full_schema(&conn).unwrap();
        drop(conn);

        let delegator = TaskDelegator::new(db_file).unwrap();
        let id = delegator.add_task("Status Test", None, None).unwrap();

        // Manually update status to 'done' using direct DB access
        delegator.conn.execute(
            "UPDATE tasks SET status = 'done' WHERE id = ?1",
            params![id],
        ).unwrap();

        let tasks = delegator.list_tasks().unwrap();
        assert_eq!(tasks[0].status, "done");
    }
}
