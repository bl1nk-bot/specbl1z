-- specgen v3: Semantic Search, Agentic Unification & Code Linking

-- ==================== Semantic Search (v3) ====================
-- ตารางเก็บ Chunk ของโค้ดและเอกสารสำหรับการค้นหาเชิงความหมาย
CREATE TABLE IF NOT EXISTS chunks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_path TEXT NOT NULL,
    content TEXT NOT NULL,
    embedding BLOB, -- เก็บ Vector embedding (รองรับ sqlite-vec ในอนาคต)
    checksum TEXT, -- ใช้ตรวจสอบการเปลี่ยนแปลงของไฟล์
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_chunks_file_path ON chunks(file_path);

-- ==================== Agentic Systems (v3) ====================
-- บันทึกการประมวลผลของ AI เพื่อการตรวจสอบย้อนกลับ (Traceability)
CREATE TABLE IF NOT EXISTS agent_executions (
    id TEXT PRIMARY KEY,
    agent_id TEXT, -- อ้างอิงถึง Agent ใน collections
    command TEXT NOT NULL,
    status TEXT DEFAULT 'pending',
    result TEXT,
    trace_id TEXT, -- ใช้สำหรับเชื่อมโยงการทำงานหลายขั้นตอน
    started_at INTEGER DEFAULT (strftime('%s', 'now')),
    finished_at INTEGER
);

-- ==================== Code Linkage (High Spec) ====================
-- เชื่อมโยงข้อมูลใน DB (เช่น Rules, Tasks) เข้ากับตำแหน่งจริงใน Source Code
CREATE TABLE IF NOT EXISTS code_links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entity_type TEXT NOT NULL, -- 'rule', 'task', 'memory', 'spec'
    entity_id TEXT NOT NULL,
    file_path TEXT NOT NULL,
    start_line INTEGER,
    end_line INTEGER,
    context_snippet TEXT, -- ข้อความแวดล้อมเพื่อความแม่นยำ
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_code_links_entity ON code_links(entity_type, entity_id);

-- Migration version record
INSERT OR IGNORE INTO schema_migrations(version, description) VALUES (3, 'specgen v3: Semantic Search, Agentic Executions, and Code Linkage');
