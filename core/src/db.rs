use crate::bl1nk::{Category, Rule, Section};
use rusqlite::{Connection, Result};

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Database { conn };
        db.init_tables()?;
        Ok(db)
    }

    fn init_tables(&self) -> Result<()> {
        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS categories (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                key TEXT UNIQUE NOT NULL,
                label TEXT NOT NULL,
                icon TEXT DEFAULT '📋',
                order_index INTEGER DEFAULT 0
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS sections (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                category_id INTEGER NOT NULL,
                title TEXT NOT NULL,
                icon TEXT DEFAULT '📁',
                color TEXT,
                text_color TEXT,
                order_index INTEGER DEFAULT 0,
                FOREIGN KEY(category_id) REFERENCES categories(id)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS rules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                section_id INTEGER NOT NULL,
                text TEXT NOT NULL,
                tag INTEGER NOT NULL,
                code TEXT,
                order_index INTEGER DEFAULT 0,
                is_custom BOOLEAN DEFAULT FALSE,
                user_id TEXT,
                FOREIGN KEY(section_id) REFERENCES sections(id)
            )",
            [],
        )?;

        self.conn.execute(
            "CREATE TABLE IF NOT EXISTS user_progress (
                user_id TEXT NOT NULL,
                rule_id INTEGER NOT NULL,
                checked BOOLEAN DEFAULT FALSE,
                PRIMARY KEY(user_id, rule_id),
                FOREIGN KEY(rule_id) REFERENCES rules(id)
            )",
            [],
        )?;
        Ok(())
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

    pub fn get_rules(&self, section_id: i32) -> Result<Vec<Rule>> {
        let mut stmt = self.conn.prepare("SELECT id, section_id, text, tag, code, order_index, is_custom, user_id FROM rules WHERE section_id = ? ORDER BY order_index")?;
        let rule_iter = stmt.query_map([section_id], |row| {
            Ok(Rule {
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
        for rule in rule_iter {
            results.push(rule?);
        }
        Ok(results)
    }
}
