-- specgen v2: Add Memory, Policy, Prompt, TestPlan tables
-- This migration adds the v2 feature tables to an existing v1 database

-- ==================== Memory System ====================
CREATE TABLE IF NOT EXISTS memory_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    scope TEXT NOT NULL CHECK(scope IN ('global', 'project', 'session', 'working', 'policy', 'identity')),
    category TEXT NOT NULL CHECK(category IN ('fact', 'preference', 'history', 'context', 'inference')),
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    source TEXT,
    confidence REAL DEFAULT 0.5,
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER DEFAULT (strftime('%s', 'now')),
    version INTEGER DEFAULT 1,
    status TEXT DEFAULT 'active',
    tags JSON DEFAULT '[]',
    owner TEXT,
    access_level TEXT DEFAULT 'private',
    provenance JSON,
    expires_at INTEGER
);

CREATE INDEX IF NOT EXISTS idx_memory_entries_scope ON memory_entries(scope);
CREATE INDEX IF NOT EXISTS idx_memory_entries_category ON memory_entries(category);

CREATE TABLE IF NOT EXISTS memory_links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    from_entry_id INTEGER NOT NULL REFERENCES memory_entries(id) ON DELETE CASCADE,
    to_entry_id INTEGER NOT NULL REFERENCES memory_entries(id) ON DELETE CASCADE,
    relation_type TEXT NOT NULL,
    metadata JSON DEFAULT '{}',
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    UNIQUE(from_entry_id, to_entry_id, relation_type)
);

CREATE TABLE IF NOT EXISTS memory_audit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_id INTEGER NOT NULL REFERENCES memory_entries(id) ON DELETE CASCADE,
    operation TEXT NOT NULL,
    performed_by TEXT NOT NULL,
    timestamp INTEGER DEFAULT (strftime('%s', 'now')),
    old_value JSON,
    new_value JSON
);

-- ==================== Prompt System ====================

CREATE TABLE IF NOT EXISTS prompt_blocks (
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
);

CREATE TABLE IF NOT EXISTS context_packs (
    session_id TEXT PRIMARY KEY,
    blocks_json JSON NOT NULL,
    assembled_at INTEGER DEFAULT (strftime('%s', 'now')),
    trace_id TEXT,
    total_tokens_estimate INTEGER DEFAULT 0
);

-- ==================== Policy & Conflict ====================

CREATE TABLE IF NOT EXISTS conflict_records (
    id TEXT PRIMARY KEY,
    conflicting_block_ids JSON NOT NULL,
    conflict_type TEXT NOT NULL,
    description TEXT,
    resolution TEXT,
    resolved_by TEXT,
    resolved_at INTEGER,
    is_resolved BOOLEAN DEFAULT FALSE,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE TABLE IF NOT EXISTS policy_rules (
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
);

CREATE TABLE IF NOT EXISTS policy_evaluations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    context_id TEXT NOT NULL,
    rule_id TEXT NOT NULL REFERENCES policy_rules(id),
    allowed BOOLEAN NOT NULL,
    modification JSON,
    reason TEXT,
    evaluated_at INTEGER DEFAULT (strftime('%s', 'now'))
);

-- ==================== Test & Work Instructions ====================

CREATE TABLE IF NOT EXISTS test_plans (
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
);

CREATE TABLE IF NOT EXISTS work_instructions (
    id TEXT PRIMARY KEY,
    template_id TEXT NOT NULL,
    test_plan_id TEXT NOT NULL,
    format TEXT NOT NULL,
    content TEXT NOT NULL,
    version TEXT DEFAULT '1.0',
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    approved_by TEXT,
    approved_at INTEGER
);

-- Migration version record
INSERT OR IGNORE INTO schema_migrations(version, description) VALUES (2, 'specgen v2: Memory + Policy + Prompt + TestPlan tables');