-- specgen v2: Memory Engine + Policy Engine Schema
-- SQLite (local) - used by core/src/memory.rs and core/src/policy.rs
-- Compatible with rusqlite

PRAGMA foreign_keys = ON;

-- ==================== Memory System ====================

-- MemoryEntry table (FR1.1)
CREATE TABLE IF NOT EXISTS memory_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    scope TEXT NOT NULL CHECK(scope IN ('global', 'project', 'session', 'working', 'policy', 'identity')),
    category TEXT NOT NULL CHECK(category IN ('fact', 'preference', 'history', 'context', 'inference')),
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    source TEXT,                    -- provenance: where this came from
    confidence REAL DEFAULT 0.5 CHECK(confidence >= 0.0 AND confidence <= 1.0),
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER DEFAULT (strftime('%s', 'now')),
    version INTEGER DEFAULT 1,
    status TEXT DEFAULT 'active' CHECK(status IN ('active', 'archived', 'expired')),
    tags JSON DEFAULT '[]',
    owner TEXT,                     -- user_id if scoped to user
    access_level TEXT DEFAULT 'private' CHECK(access_level IN ('public', 'project', 'private', 'secret')),
    provenance JSON,                -- detailed source metadata
    expires_at INTEGER,             -- Unix timestamp, optional TTL
    -- Composite index for common queries
    UNIQUE(key, scope, owner) ON CONFLICT REPLACE
);

CREATE INDEX IF NOT EXISTS idx_memory_scope ON memory_entries(scope);
CREATE INDEX IF NOT EXISTS idx_memory_category ON memory_entries(category);
CREATE INDEX IF NOT EXISTS idx_memory_confidence ON memory_entries(confidence);
CREATE INDEX IF NOT EXISTS idx_memory_owner ON memory_entries(owner);
CREATE INDEX IF NOT EXISTS idx_memory_expires ON memory_entries(expires_at);

-- MemoryLink: links between memory entries
CREATE TABLE IF NOT EXISTS memory_links (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    from_entry_id INTEGER NOT NULL REFERENCES memory_entries(id) ON DELETE CASCADE,
    to_entry_id INTEGER NOT NULL REFERENCES memory_entries(id) ON DELETE CASCADE,
    relation_type TEXT NOT NULL,    -- e.g., "references", "depends_on", "contradicts"
    metadata JSON DEFAULT '{}',
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    UNIQUE(from_entry_id, to_entry_id, relation_type)
);

CREATE INDEX IF NOT EXISTS idx_memory_links_from ON memory_links(from_entry_id);
CREATE INDEX IF NOT EXISTS idx_memory_links_to ON memory_links(to_entry_id);

-- MemoryAuditLog: audit trail for all memory operations
CREATE TABLE IF NOT EXISTS memory_audit_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    entry_id INTEGER NOT NULL REFERENCES memory_entries(id) ON DELETE CASCADE,
    operation TEXT NOT NULL,         -- "insert", "update", "delete", "link"
    performed_by TEXT NOT NULL,      -- user_id or "system"
    timestamp INTEGER DEFAULT (strftime('%s', 'now')),
    old_value JSON,
    new_value JSON
);

CREATE INDEX IF NOT EXISTS idx_audit_log_entry ON memory_audit_log(entry_id);
CREATE INDEX IF NOT EXISTS idx_audit_log_time ON memory_audit_log(timestamp);

-- ==================== Prompt System ====================

-- PromptBlock: individual blocks used in composition
CREATE TABLE IF NOT EXISTS prompt_blocks (
    id TEXT PRIMARY KEY,            -- UUID
    type TEXT NOT NULL CHECK(type IN ('system', 'policy', 'identity', 'memory', 'task', 'tool_instruction', 'plan', 'user_input', 'guardrail', 'output_format')),
    priority INTEGER DEFAULT 5 CHECK(priority >= 1 AND priority <= 10),
    scope TEXT NOT NULL CHECK(scope IN ('global', 'project', 'session', 'working', 'policy', 'identity')),
    content TEXT NOT NULL,
    source TEXT,                    -- provenance
    constraints JSON,               -- {token_limit, must_include, etc.}
    dependencies JSON DEFAULT '[]',  -- array of block IDs
    version INTEGER DEFAULT 1,
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_prompt_blocks_scope ON prompt_blocks(scope);
CREATE INDEX IF NOT EXISTS idx_prompt_blocks_type ON prompt_blocks(type);

-- ContextPack: assembled context for an execution session
CREATE TABLE IF NOT EXISTS context_packs (
    session_id TEXT PRIMARY KEY,
    blocks_json JSON NOT NULL,       -- serialized PromptBlock[] ordered by priority
    assembled_at INTEGER DEFAULT (strftime('%s', 'now')),
    trace_id TEXT,
    total_tokens_estimate INTEGER DEFAULT 0
);

-- ConflictRecord: records of conflicts and resolutions
CREATE TABLE IF NOT EXISTS conflict_records (
    id TEXT PRIMARY KEY,            -- UUID
    conflicting_block_ids JSON NOT NULL,  -- array of block IDs
    conflict_type TEXT NOT NULL,     -- "scope_violation", "rule_violation", "priority_inversion"
    description TEXT,
    resolution TEXT,                -- "first_wins", "higher_priority", "manual"
    resolved_by TEXT,
    resolved_at INTEGER,
    is_resolved BOOLEAN DEFAULT FALSE,
    created_at INTEGER DEFAULT (strftime('%s', 'now'))
);

-- ==================== Policy System ====================

-- PolicyRule: stored rules for the policy engine
CREATE TABLE IF NOT EXISTS policy_rules (
    id TEXT PRIMARY KEY,            -- UUID
    rule_type TEXT NOT NULL CHECK(rule_type IN ('hard', 'soft', 'context', 'safety')),
    name TEXT NOT NULL,
    description TEXT,
    condition TEXT,                 -- JSONLogic or DSL expression (as string)
    action TEXT NOT NULL CHECK(action IN ('allow', 'deny', 'modify', 'flag')),
    priority INTEGER DEFAULT 5 CHECK(priority >= 1 AND priority <= 100),
    scope TEXT,                     -- comma-separated scopes this applies to
    enabled BOOLEAN DEFAULT TRUE,
    effective_from INTEGER DEFAULT (strftime('%s', 'now')),
    effective_until INTEGER,
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_policy_rules_type ON policy_rules(rule_type);
CREATE INDEX IF NOT EXISTS idx_policy_rules_enabled ON policy_rules(enabled);

-- PolicyEvaluation: log of policy evaluations (for audit)
CREATE TABLE IF NOT EXISTS policy_evaluations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    context_id TEXT NOT NULL,       -- session_id or context identifier
    rule_id TEXT NOT NULL REFERENCES policy_rules(id),
    allowed BOOLEAN NOT NULL,
    modification JSON,              -- suggested changes if action='modify'
    reason TEXT,
    evaluated_at INTEGER DEFAULT (strftime('%s', 'now'))
);

CREATE INDEX IF NOT EXISTS idx_policy_eval_context ON policy_evaluations(context_id);
CREATE INDEX IF NOT EXISTS idx_policy_eval_rule ON policy_evaluations(rule_id);

-- ==================== Test & Work Instructions ====================

-- TestPlan: generated test plans from templates
CREATE TABLE IF NOT EXISTS test_plans (
    id TEXT PRIMARY KEY,            -- UUID
    template_id TEXT NOT NULL,
    version TEXT DEFAULT '1.0',
    suites JSON NOT NULL,           -- array of TestSuite {id, name, test_case_ids[]}
    test_cases JSON NOT NULL,       -- map<string, TestCase>
    coverage_target REAL DEFAULT 0.8 CHECK(coverage_target >= 0.0 AND coverage_target <= 1.0),
    generated_by TEXT NOT NULL,
    generated_at INTEGER DEFAULT (strftime('%s', 'now')),
    llm_model_used TEXT,
    status TEXT DEFAULT 'draft' CHECK(status IN ('draft', 'review', 'approved', 'deprecated'))
);

-- WorkInstruction: rendered work instructions document
CREATE TABLE IF NOT EXISTS work_instructions (
    id TEXT PRIMARY KEY,            -- UUID
    template_id TEXT NOT NULL,
    test_plan_id TEXT NOT NULL,
    format TEXT NOT NULL CHECK(format IN ('markdown', 'json', 'html')),
    content TEXT NOT NULL,          -- rendered document body
    version TEXT DEFAULT '1.0',
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    approved_by TEXT,
    approved_at INTEGER
);

CREATE INDEX IF NOT EXISTS idx_work_instr_template ON work_instructions(template_id);

-- ==================== Migrations tracking ====================

CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    description TEXT NOT NULL,
    applied_at INTEGER DEFAULT (strftime('%s', 'now'))
);

-- Insert initial migration record
INSERT OR IGNORE INTO schema_migrations(version, description) VALUES (2, 'specgen v2: Memory + Policy + TestPlan tables');
