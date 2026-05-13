-- bl1nk Coding Standards Database Schema
-- Run this with: psql $DATABASE_URL -f src/schema.sql
-- Or use Neon console SQL editor

-- Enable extensions if needed
CREATE EXTENSION IF NOT EXISTS "pg_trgm"; -- for text search

-- Categories (tabs)
CREATE TABLE IF NOT EXISTS categories (
  id SERIAL PRIMARY KEY,
  key TEXT UNIQUE NOT NULL,
  label TEXT NOT NULL,
  icon TEXT DEFAULT '📋',
  order_index INTEGER DEFAULT 0,
  created_at TIMESTAMP DEFAULT NOW()
);

-- Sections within categories
CREATE TABLE IF NOT EXISTS sections (
  id SERIAL PRIMARY KEY,
  category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
  title TEXT NOT NULL,
  icon TEXT DEFAULT '📁',
  color TEXT DEFAULT '#EEEDFE',
  text_color TEXT DEFAULT '#3C3489',
  order_index INTEGER DEFAULT 0,
  created_at TIMESTAMP DEFAULT NOW()
);

-- Rules (checklist items)
CREATE TABLE IF NOT EXISTS rules (
  id SERIAL PRIMARY KEY,
  section_id INTEGER NOT NULL REFERENCES sections(id) ON DELETE CASCADE,
  text TEXT NOT NULL,
  tag TEXT NOT NULL CHECK (tag IN ('must', 'should', 'avoid')),
  code TEXT,
  order_index INTEGER DEFAULT 0,
  is_custom BOOLEAN DEFAULT FALSE,
  user_id TEXT, -- NULL for standard rules, user-specific for custom
  created_at TIMESTAMP DEFAULT NOW(),
  search_vector tsvector GENERATED ALWAYS AS (
    to_tsvector('simple', text || ' ' || COALESCE(code, ''))
  ) STORED
);

CREATE INDEX IF NOT EXISTS rules_search_idx ON rules USING GIN (search_vector);
CREATE INDEX IF NOT EXISTS rules_text_trgm_idx ON rules USING GIN (text gin_trgm_ops);

-- User progress (checked state) - works with Neon Auth user IDs
CREATE TABLE IF NOT EXISTS user_progress (
  user_id TEXT NOT NULL,
  rule_id INTEGER NOT NULL REFERENCES rules(id) ON DELETE CASCADE,
  checked BOOLEAN DEFAULT FALSE,
  updated_at TIMESTAMP DEFAULT NOW(),
  PRIMARY KEY (user_id, rule_id)
);

-- For Neon Auth integration (optional, if you enable Neon Auth in dashboard):
-- The table neon_auth_users or similar will be auto-created by Neon Auth.
-- You can then use RLS policies like:
-- ALTER TABLE user_progress ENABLE ROW LEVEL SECURITY;
-- CREATE POLICY user_progress_policy ON user_progress
--   USING (user_id = current_setting('app.current_user_id', true));

COMMENT ON TABLE categories IS 'Main tabs like General, TypeScript, Python, etc.';
COMMENT ON TABLE rules IS 'Individual checklist rules. Use is_custom + user_id for personal rules.';
COMMENT ON TABLE user_progress IS 'Per-user check state. Use Neon Auth user ID as user_id.';

-- ==================== specgen v2: Memory System ====================

-- MemoryEntry: unified memory store with scope-based layering
CREATE TABLE IF NOT EXISTS memory_entries (
    id BIGSERIAL PRIMARY KEY,
    scope TEXT NOT NULL CHECK(scope IN ('global', 'project', 'session', 'working', 'policy', 'identity')),
    category TEXT NOT NULL CHECK(category IN ('fact', 'preference', 'history', 'context', 'inference')),
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    source TEXT,                    -- provenance: where this came from
    confidence REAL DEFAULT 0.5 CHECK(confidence >= 0.0 AND confidence <= 1.0),
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW(),
    version INTEGER DEFAULT 1,
    status TEXT DEFAULT 'active' CHECK(status IN ('active', 'archived', 'expired')),
    tags JSONB DEFAULT '[]'::jsonb,
    owner TEXT,                     -- user_id if scoped to user
    access_level TEXT DEFAULT 'private' CHECK(access_level IN ('public', 'project', 'private', 'secret')),
    provenance JSONB,               -- detailed source metadata
    expires_at TIMESTAMP,
    UNIQUE(key, scope, owner) DEFERRABLE INITIALLY DEFERRED
);

CREATE INDEX IF NOT EXISTS idx_memory_entries_scope ON memory_entries(scope);
CREATE INDEX IF NOT EXISTS idx_memory_entries_category ON memory_entries(category);
CREATE INDEX IF NOT EXISTS idx_memory_entries_confidence ON memory_entries(confidence);
CREATE INDEX IF NOT EXISTS idx_memory_entries_owner ON memory_entries(owner);
CREATE INDEX IF NOT EXISTS idx_memory_entries_expires ON memory_entries(expires_at);
CREATE INDEX IF NOT EXISTS idx_memory_entries_status ON memory_entries(status);

-- GIN index for JSONB tags (if needed for tag queries)
CREATE INDEX IF NOT EXISTS idx_memory_entries_tags ON memory_entries USING GIN (tags);

-- MemoryLink: links between memory entries (graph relationships)
CREATE TABLE IF NOT EXISTS memory_links (
    id BIGSERIAL PRIMARY KEY,
    from_entry_id BIGINT NOT NULL REFERENCES memory_entries(id) ON DELETE CASCADE,
    to_entry_id BIGINT NOT NULL REFERENCES memory_entries(id) ON DELETE CASCADE,
    relation_type TEXT NOT NULL,    -- 'references', 'depends_on', 'contradicts', etc.
    metadata JSONB DEFAULT '{}'::jsonb,
    created_at TIMESTAMP DEFAULT NOW(),
    UNIQUE(from_entry_id, to_entry_id, relation_type)
);

CREATE INDEX IF NOT EXISTS idx_memory_links_from ON memory_links(from_entry_id);
CREATE INDEX IF NOT EXISTS idx_memory_links_to ON memory_links(to_entry_id);

-- MemoryAuditLog: audit trail for memory operations
CREATE TABLE IF NOT EXISTS memory_audit_log (
    id BIGSERIAL PRIMARY KEY,
    entry_id BIGINT NOT NULL REFERENCES memory_entries(id) ON DELETE CASCADE,
    operation TEXT NOT NULL,        -- 'insert', 'update', 'delete', 'link'
    performed_by TEXT NOT NULL,     -- user_id or 'system'
    timestamp TIMESTAMP DEFAULT NOW(),
    old_value JSONB,
    new_value JSONB
);

CREATE INDEX IF NOT EXISTS idx_memory_audit_entry ON memory_audit_log(entry_id);
CREATE INDEX IF NOT EXISTS idx_memory_audit_time ON memory_audit_log(timestamp);
CREATE INDEX IF NOT EXISTS idx_memory_audit_by ON memory_audit_log(performed_by);

-- ==================== Prompt System ====================

-- PromptBlock: reusable prompt components
CREATE TABLE IF NOT EXISTS prompt_blocks (
    id TEXT PRIMARY KEY,           -- UUID
    type TEXT NOT NULL CHECK(type IN ('system', 'policy', 'identity', 'memory', 'task', 'tool_instruction', 'plan', 'user_input', 'guardrail', 'output_format')),
    priority INTEGER DEFAULT 5 CHECK(priority >= 1 AND priority <= 10),
    scope TEXT NOT NULL CHECK(scope IN ('global', 'project', 'session', 'working', 'policy', 'identity')),
    content TEXT NOT NULL,
    source TEXT,                   -- provenance
    constraints JSONB,             -- {token_limit, must_include, etc.}
    dependencies JSONB DEFAULT '[]'::jsonb,  -- array of block IDs
    version INTEGER DEFAULT 1,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_prompt_blocks_scope ON prompt_blocks(scope);
CREATE INDEX IF NOT EXISTS idx_prompt_blocks_type ON prompt_blocks(type);
CREATE INDEX IF NOT EXISTS idx_prompt_blocks_source ON prompt_blocks(source);

-- ContextPack: assembled context for a session
CREATE TABLE IF NOT EXISTS context_packs (
    session_id TEXT PRIMARY KEY,
    blocks_json JSONB NOT NULL,    -- array of PromptBlock ordered by priority
    assembled_at TIMESTAMP DEFAULT NOW(),
    trace_id TEXT,
    total_tokens_estimate INTEGER DEFAULT 0
);

CREATE INDEX IF NOT EXISTS idx_context_packs_trace ON context_packs(trace_id);

-- ConflictRecord: records of policy/temporal conflicts and resolutions
CREATE TABLE IF NOT EXISTS conflict_records (
    id TEXT PRIMARY KEY,           -- UUID
    conflicting_block_ids JSONB NOT NULL,  -- array of block IDs
    conflict_type TEXT NOT NULL,   -- 'scope_violation', 'rule_violation', 'priority_inversion'
    description TEXT,
    resolution TEXT,               -- 'first_wins', 'higher_priority', 'manual'
    resolved_by TEXT,
    resolved_at TIMESTAMP,
    is_resolved BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_conflicts_type ON conflict_records(conflict_type);
CREATE INDEX IF NOT EXISTS idx_conflicts_resolved ON conflict_records(is_resolved);

-- ==================== Policy Engine ====================

-- PolicyRule: rules for policy evaluation
CREATE TABLE IF NOT EXISTS policy_rules (
    id TEXT PRIMARY KEY,           -- UUID
    rule_type TEXT NOT NULL CHECK(rule_type IN ('hard', 'soft', 'context', 'safety')),
    name TEXT NOT NULL,
    description TEXT,
    condition TEXT,                -- JSONLogic or DSL expression (stored as text)
    action TEXT NOT NULL CHECK(action IN ('allow', 'deny', 'modify', 'flag')),
    priority INTEGER DEFAULT 5 CHECK(priority >= 1 AND priority <= 100),
    scope TEXT,                    -- comma-separated scopes this applies to
    enabled BOOLEAN DEFAULT TRUE,
    effective_from TIMESTAMP DEFAULT NOW(),
    effective_until TIMESTAMP,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_policy_rules_type ON policy_rules(rule_type);
CREATE INDEX IF NOT EXISTS idx_policy_rules_enabled ON policy_rules(enabled);
CREATE INDEX IF NOT EXISTS idx_policy_rules_scope ON policy_rules(scope);

-- PolicyEvaluation: log of policy evaluations for audit & debugging
CREATE TABLE IF NOT EXISTS policy_evaluations (
    id BIGSERIAL PRIMARY KEY,
    context_id TEXT NOT NULL,      -- session_id or context identifier
    rule_id TEXT NOT NULL REFERENCES policy_rules(id),
    allowed BOOLEAN NOT NULL,
    modification JSONB,            -- suggested changes if action='modify'
    reason TEXT,
    evaluated_at TIMESTAMP DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_policy_eval_context ON policy_evaluations(context_id);
CREATE INDEX IF NOT EXISTS idx_policy_eval_rule ON policy_evaluations(rule_id);
CREATE INDEX IF NOT EXISTS idx_policy_eval_time ON policy_evaluations(evaluated_at);

-- ==================== Test & Work Instructions (FR6) ====================

-- TestPlan: generated test plans from templates
CREATE TABLE IF NOT EXISTS test_plans (
    id TEXT PRIMARY KEY,           -- UUID
    template_id TEXT NOT NULL,
    version TEXT DEFAULT '1.0',
    suites JSONB NOT NULL,         -- array of TestSuite {id, name, test_case_ids[]}
    test_cases JSONB NOT NULL,     -- map<string, TestCase> as JSONB
    coverage_target REAL DEFAULT 0.8 CHECK(coverage_target >= 0.0 AND coverage_target <= 1.0),
    generated_by TEXT NOT NULL,
    generated_at TIMESTAMP DEFAULT NOW(),
    llm_model_used TEXT,
    status TEXT DEFAULT 'draft' CHECK(status IN ('draft', 'review', 'approved', 'deprecated'))
);

CREATE INDEX IF NOT EXISTS idx_test_plans_template ON test_plans(template_id);
CREATE INDEX IF NOT EXISTS idx_test_plans_status ON test_plans(status);

-- WorkInstruction: rendered work instructions document
CREATE TABLE IF NOT EXISTS work_instructions (
    id TEXT PRIMARY KEY,           -- UUID
    template_id TEXT NOT NULL,
    test_plan_id TEXT NOT NULL,
    format TEXT NOT NULL CHECK(format IN ('markdown', 'json', 'html', 'pdf')),
    content TEXT NOT NULL,         -- rendered document body
    version TEXT DEFAULT '1.0',
    created_at TIMESTAMP DEFAULT NOW(),
    approved_by TEXT,
    approved_at TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_work_instr_template ON work_instructions(template_id);
CREATE INDEX IF NOT EXISTS idx_work_instr_testplan ON work_instructions(test_plan_id);

-- ==================== Extensions & Utilities ====================

-- Full-text search on memory value (simple)
ALTER TABLE memory_entries ADD COLUMN IF NOT EXISTS search_vector tsvector 
    GENERATED ALWAYS AS (to_tsvector('simple', value)) STORED;
CREATE INDEX IF NOT EXISTS idx_memory_search ON memory_entries USING GIN (search_vector);

-- Trigger to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Apply trigger to tables that have updated_at
DROP TRIGGER IF EXISTS update_memory_entries_updated_at ON memory_entries;
CREATE TRIGGER update_memory_entries_updated_at
    BEFORE UPDATE ON memory_entries
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_prompt_blocks_updated_at ON prompt_blocks;
CREATE TRIGGER update_prompt_blocks_updated_at
    BEFORE UPDATE ON prompt_blocks
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

DROP TRIGGER IF EXISTS update_policy_rules_updated_at ON policy_rules;
CREATE TRIGGER update_policy_rules_updated_at
    BEFORE UPDATE ON policy_rules
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

-- ==================== Row Level Security (RLS) ====================
-- Enable RLS for multi-tenant support (future)
-- Patterns:
--   - Global memory: access_level='public' readable by all
--   - Project memory: access_level='project' readable by project members
--   - Private memory: access_level='private' only owner can read
-- ALTER TABLE memory_entries ENABLE ROW LEVEL SECURITY;
-- CREATE POLICY memory_read_policy ON memory_entries
--   USING (
--     access_level = 'public' OR
--     access_level = 'project' AND owner = current_setting('app.current_project_id', true) OR
--     owner = current_setting('app.current_user_id', true)
--   );

-- ==================== Schema Version Tracking ====================

CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    description TEXT NOT NULL,
    applied_at TIMESTAMP DEFAULT NOW()
);

-- Insert initial migration for v2 if not exists
INSERT INTO schema_migrations(version, description)
VALUES (2, 'specgen v2: Memory + Policy + Prompt + TestPlan tables')
ON CONFLICT (version) DO NOTHING;

-- Comments
COMMENT ON TABLE memory_entries IS 'Unified memory store with scope-based layering (FR1.1)';
COMMENT ON TABLE memory_links IS 'Graph relationships between memory entries';
COMMENT ON TABLE memory_audit_log IS 'Audit trail for all memory CRUD operations';
COMMENT ON TABLE prompt_blocks IS 'Reusable prompt blocks for composition (FR1.2)';
COMMENT ON TABLE context_packs IS 'Assembled context for execution sessions (FR1.3)';
COMMENT ON TABLE conflict_records IS 'Policy and scope conflict records (FR1.4)';
COMMENT ON TABLE policy_rules IS 'Policy rules: hard, soft, context, safety (FR3)';
COMMENT ON TABLE policy_evaluations IS 'Audit log of policy evaluations';
COMMENT ON TABLE test_plans IS 'Generated test plans from templates (FR6)';
COMMENT ON TABLE work_instructions IS 'Rendered work instruction documents (FR6)';