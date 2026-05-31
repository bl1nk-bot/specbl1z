-- Migration 004: Add memory topic enforcement
-- This migration adds the 'topic' column to the memory_entries table

ALTER TABLE memory_entries ADD COLUMN topic TEXT NOT NULL DEFAULT 'LEARN' CHECK(topic IN ('LEARN', 'WORK', 'TOOL', 'INTEREST', 'PROJECT', 'IDENTIFY'));

-- Re-create the index to include topic if needed (optional for now)

INSERT OR IGNORE INTO schema_migrations(version, description) VALUES (4, 'Add memory topic enforcement (LEARN, WORK, TOOL, INTEREST, PROJECT, IDENTIFY)');
