use anyhow::{anyhow, Result};
use ignore::WalkBuilder;
use reqwest::blocking::Client;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use std::path::Path;

const OLLAMA_API: &str = "http://localhost:11434/api/embeddings";
const MODEL: &str = "qwen3-embedding:0.6b";
const SENSE_DIR: &str = ".sense";
const DB_NAME: &str = "codesense.db";

#[derive(Serialize, Deserialize, Debug)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

pub struct CodeSense {
    db: Connection,
    client: Client,
    project_name: String,
}

impl CodeSense {
    pub fn new(root_dir: &Path) -> Result<Self> {
        let sense_path = root_dir.join(SENSE_DIR);
        if !sense_path.exists() {
            std::fs::create_dir_all(&sense_path)?;
        }
        
        let db_path = sense_path.join(DB_NAME);
        let db = Connection::open(db_path)?;
        let client = Client::new();
        let project_name = root_dir
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        db.execute(
            "CREATE TABLE IF NOT EXISTS chunks (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_path TEXT,
                content TEXT,
                embedding BLOB
            )",
            [],
        )?;

        Ok(Self {
            db,
            client,
            project_name,
        })
    }

    fn get_embedding(&self, text: &str) -> Result<Vec<f32>> {
        let res = self
            .client
            .post(OLLAMA_API)
            .json(&serde_json::json!({
                "model": MODEL,
                "prompt": text
            }))
            .send()
            .map_err(|e| anyhow!("AI API Error: {}. Is Ollama running?", e))?;

        let data: EmbeddingResponse = res.json()?;
        Ok(data.embedding)
    }

    pub fn index(&self, root_dir: &Path) -> Result<()> {
        println!("Indexing project: {}...", self.project_name);

        // Clear old index
        self.db.execute("DELETE FROM chunks", [])?;

        let walker = WalkBuilder::new(root_dir)
            .hidden(false)
            .git_ignore(true)
            .build();

        for entry in walker {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() {
                let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                if !["rs", "ts", "js", "proto", "md", "toml", "json"].contains(&ext) {
                    continue;
                }

                let rel_path = path.strip_prefix(root_dir)?.to_str().unwrap_or("");
                if rel_path.contains("target")
                    || rel_path.contains("node_modules")
                    || rel_path.contains(".git")
                {
                    continue;
                }

                println!("  Processing: {}", rel_path);
                let content = std::fs::read_to_string(path).unwrap_or_default();
                if content.is_empty() {
                    continue;
                }

                // Simple chunking (by lines or fixed size)
                for (i, chunk) in content.as_bytes().chunks(1000).enumerate() {
                    let chunk_str = String::from_utf8_lossy(chunk);
                    if let Ok(emb) = self.get_embedding(&chunk_str) {
                        let emb_blob = bincode::serialize(&emb)?;
                        self.db.execute(
                            "INSERT INTO chunks (file_path, content, embedding) VALUES (?, ?, ?)",
                            params![rel_path, format!("{}: chunk {}", rel_path, i), emb_blob],
                        )?;
                    }
                }
            }
        }

        println!("Indexing complete!");
        Ok(())
    }

    pub fn search(&self, query: &str) -> Result<()> {
        let query_emb = self.get_embedding(query)?;

        let mut stmt = self
            .db
            .prepare("SELECT file_path, content, embedding FROM chunks")?;
        let rows = stmt.query_map([], |row| {
            let file_path: String = row.get(0)?;
            let content: String = row.get(1)?;
            let emb_blob: Vec<u8> = row.get(2)?;
            let emb: Vec<f32> = bincode::deserialize(&emb_blob).unwrap_or_default();
            Ok((file_path, content, emb))
        })?;

        let mut results = Vec::new();
        for row in rows {
            let (path, content, emb) = row?;
            let score = cosine_similarity(&query_emb, &emb);
            results.push((score, path, content));
        }

        results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        println!("\nTop matches for: '{}'", query);
        for (score, path, content) in results.iter().take(5) {
            println!("[{:.2}] {}: {}", score, path, content);
        }

        Ok(())
    }
}

fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    let dot: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let norm_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
    let norm_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 0.0;
    }
    dot / (norm_a * norm_b)
}
