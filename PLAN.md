# แผนการพัฒนา (Development Plan) — specgen v2

## ภาพรวม

พัฒนาต่อยอดจาก specgen เดิมให้เป็น "AI-Native Spec-Driven Development Platform" โดยเพิ่มความสามารถด้าน Memory Management, Policy Engine, Prompt Composition, และ Template-to-Test automation

ระยะเวลา: 6 เดือน (แบ่งเป็น 6 phases)

## Phases, Milestones, and Tasks

### Phase 0: Foundation Cleanup (2 สัปดาห์)
**เป้าหมาย**: ทำให️ codebase ปัจจ_current stable และเขียน test coverage ให้เพียงพอ
- [x] รวม monorepo, build ผ่านทั้ง Rust และ TS (เสร็จแล้ว)
- [ ] เพิ่ม integration tests สำหรับ template rendering ทุกรูปแบบ
- [ ] เพิ่ม unit tests สำหรับ `rules_engine`, `validator`, `renderer`
- [ ] จัดทำ CI/CD pipeline (GitHub Actions)
- [ ] Document current architecture ใน `README.md` (ภาษาไทย)

**Acceptance**: `cargo test` และ `npm test` ผ่าน 100%; CI green

---

### Phase 1: Data Model and Protobuf Schema (3 สัปดาห์)
**เป้าหมาย**: ออกแบบ Protobuf schema สำหรับ Memory, Prompt, Policy

- Task 1.1: ศึกษาและออกแบบ `bl1nk.proto` เสริมด้วย messages:
  - `MemoryEntry`, `MemoryScope` (enum), `MemoryCategory` (enum)
  - `PromptBlock`, `ContextPack`
  - `ConflictRecord`
  - `TestPlan`, `TestCase`, `WorkInstruction`
- Task 1.2: สร้าง protobuf definition และ generate code:
  - Rust: `core/src/bl1nk.rs` (prost-build)
  - TypeScript: `app/src/generated/bl1nk.ts` (ts-proto)
- Task 1.3: อัปเดต schema SQL สำหรับ SQLite/Neon ให้รองรับตารางใหม่: `memory_entries`, `prompt_blocks`, `policy_rules`, `conflicts`
- Task 1.4: สร้าง Rust migration scripts สำหรับฐานข้อมูล (ใช้ `refinery` หรือ custom)
- Task 1.5: Unit tests สำหรับ serialization/deserialization

**Milestone**: Protobuf schema ล็อก, code generated, ตารางฐานข้อมูลพร้อม

---

### Phase 2: Memory Engine Implementation (4 สัปดาห์)
**เป้าหมาย**: สร้าง Memory Engine ใน Rust ที่รองรับ layered memory และ CRUD

- Task 2.1: Implement `core/src/memory.rs`:
  - `MemoryStore` struct ที่มี backend เป็น SQLite (ผ่าน rusqlite)
  - Methods: `insert(MemoryEntry)`, `get_by_id()`, `query(scope, category, confidence_min, tags)`, `update`, `delete`
  - `link(entry_a_id, entry_b_id)` เพื่อสร้างความสัมพันธ์
  - `audit_trail()` สำหรับ log
- Task 2.2: Implement TTL support: background task ที่ลบ expired entries
- Task 2.3: Implement access control: check `access_level` และ `owner` ก่อน allow read/write
- Task 2.4: Implement memory retrieval for prompt composition: `get_relevant_memory(task_description, project_id) -> Vec<MemoryEntry>` โดยใช้ semantic search (sense.rs) หรือ tag matching
- Task 2.5: Unit tests + integration tests (100+ entries)

**Milestone**: สามารถเรียก `specgen memory insert ...` และ `specgen memory search ...` ได้; ทดสอบการแยก scope

---

### Phase 3: Policy Engine (4 สัปดาห์)
**เป้าหมาย**: สร้าง Policy Engine ที่สามารถตรวจสอบและควบคุมการทำงานของระบบ

- Task 3.1: ออกแบบ Rule DSL/format (JSON-based) สำหรับ hard/soft/context/safety rules
- Task 3.2: Implement `core/src/policy.rs`:
  - `RuleRegistry` ที่โหลดกฎจาก DB หรือไฟล์
  - `Evaluator` ที่รับ context (MemoryEntry, PromptBlock, intent) และ return `PolicyResult`
  - `ConflictResolver` ที่ใช้ rule priority และ scope specificity
  - `ActionGate` ที่เช็คก่อน write memory, execute tool
  - `AuditLogger`
- Task 3.3: Define built-in rules:
  - ห้ามใช้ memory ที่ไม่มี provenance ด้วย confidence สูง
  - Identity memory ห้ามถูก overwrite โดย session memory
  - ถ้า conflict ระหว่าง project กับ global ให้ project ชนะ
- Task 3.4: Integrate Policy Engine เข้าไปใน flow ของ MCP tools: ทุก tool call จะถูก gate โดย Policy Engine
- Task 3.5: Tests: scenario testing เช่น จำลอง conflict resolution

**Milestone**: สามารถรัน `specgen policy evaluate --context-file context.json` และระบบ MCP tools ทุกตัวผ่าน policy gate

---

### Phase 4: Prompt Composer (3 สัปดาห์)
**เป้าหมาย**: สร้าง Prompt Composer ที่ประกอบ blocks ตามลำดับและ trim ได้

- Task 4.1: Implement `core/src/composer.rs`:
  - ฟังก์ชัน `compose(blocks: Vec<PromptBlock>) -> ContextPack`
  - รองรับการเรียงลำดับตาม priority
  - รองรับ trimming กรณี token limit (ใช้ tiktoken หรือ approximate)
  - ใส่ fallback block ในกรณีที่จำเป็น
- Task 4.2: Integrate Composer กับ Memory Engine: ดึง memory ที่เกี่ยวข้องและแปลงเป็น PromptBlock
- Task 4.3: สร้าง template สำหรับ prompt composition (เช่น identity block template)
- Task 4.4: MCP tool `prompt.compose` ที่รับ task description และ project id, return ContextPack
- Task 4.5: Tests: ตรวจสอบว่า output เรียงลำดับถูกต้อง และไม่เกิน limit

**Milestone**: AI client สามารถเรียก `prompt.compose` และได้ prompt ที่พร้อมใช้งานสำหรับ agent execution

---

### Phase 5: Template-to-Test & Work Instructions (4 สัปดาห์)
**เป้าหมาย**: เชื่อม template engine เข้ากับ test generation และสร้าง work instructions อัตโนมัติ

- Task 5.1: ออกแบบ `TestPlan` model (protobuf) ที่ประกอบด้วย test suites, test cases (แต่ละ case มี type: unit, integration, e2e, edge), coverage goals
- Task 5.2: Implement `core/src/testgen.rs`:
  - รับ `WorkflowTemplate` และ `coding_standards`
  - สร้าง test outlines ตาม rules เช่น critical steps → unit tests, loop_restart → edge cases, output_template → output validation tests
  - ใช้ LLM (optional) เพื่อเติมรายละเอียด test cases โดยส่ง prompt ไปยัง AI service (ผ่าน REST)
- Task 5.3: Implement `core/src/work_instructions.rs`:
  - รับ template + test plan → สร้าง Markdown/JSON document
  - Render โดยใช้ `renderer.rs` ของ specgen เดิม (ใช้ handlebars style)
- Task 5.4: MCP tools: `template.generate_test_plan`, `template.generate_work_instructions`
- Task 5.5: CLI commands: `specgen testplan generate <template>`, `specgen work-instructions generate <template> <testplan>`
- Task 5.6: Integration tests: ใช้ template จริงจาก `templates/spec-workflow.md` และตรวจสอบว่า test plan ที่สร้างครอบคลุม

**Milestone**: เราสามารถสร้าง test plan และ work instructions ได้จาก template

---

### Phase 6: AI Client Integration & End-to-End Workflow (4 สัปดาห์)
**เป้าหมาย**: สาธิตการทำงานร่วมกับ AI client (Claude Code) แบบครบวงจร

- Task 6.1: ปรับปรุง MCP server ให้มี tools ครบถ้วน (memory.*, prompt.*, policy.*, template.*, sense.search)
- Task 6.2: สร้างตัวอย่าง workflow script (Claude Code custom slash command) ที่:
  1. ผู้ใช้พูดว่า "create spec for feature X"
  2. AI เรียก `template.create_from_prompt(...)` → ได้ template
  3. AI เรียก `template.generate_test_plan(...)` → ได้ test plan
  4. AI เรียก `template.generate_work_instructions(...)` → ได้เอกสาร
  5. AI แสดงผลและถาม human approval
- Task 6.3: Implement feedback loop: หลัง AI execute test, ผลลัพธ์จะถูก feed กลับมาเพื่อปรับปรุง memory และ policy
- Task 6.4: Dashboard (Tauri) พื้นฐาน: แสดง memory tree, templates, test plans, work instructions
- Task 6.5: Documentation: คู่มือการใช้งาน (ภาษาไทย), API docs
- Task 6.6: Performance tuning และ security hardening

**Milestone**: Demo จบ: สร้าง spec + test plan + work instructions ภายใน 5 นาที โดย AI client ทำงานผ่าน MCP

---

## คำสั่งงาน (Work Instructions)

ต่อไปนี้เป็นตัวอย่างคำสั่งงานสำหรับทีมพัฒนา โดยอิงจาก Phase 1 Task 1.1–1.3:

### งานที่ 1: ออกแบบและ Implement Protobuf Schema สำหรับ Memory
- **ผู้รับผิดชอบ**: ทีม backend (Rust + TS)
- **ขั้นตอน**:
  1. เปิดไฟล์ `schema/bl1nk.proto` (หรือสร้างใหม่) เพิ่ม message `MemoryEntry` ตามข้อกำหนด
  2. ตรวจสอบว่า enum `MemoryScope` และ `MemoryCategory` ครอบคลุม
  3. Run `buf generate` หรือ script สร้าง code: สำหรับ Rust (`core/build.rs`), สำหรับ TS (`protoc --ts_out=...`)
  4. เขียน unit test ใน Rust (`core/tests/proto_tests.rs`) ว่า serialize/deserialize ได้ถูกต้อง
  5. อัปเดต `app/src/generated/bl1nk.ts` และตรวจสอบ types
  6. สร้างตารางใน SQLite (`core/schema.sql`) และ Neon (`app/src/schema.sql`) โดยใช้ migration script
  7. Push branch และสร้าง PR พร้อม review
- **Acceptance**:
  - สามารถสร้าง `MemoryEntry` ใน Rust และแปลงเป็น JSON ได้
  - ตารางใน DB สร้างได้
  - TypeScript types ถูกต้องและ compile ผ่าน

### งานที่ 2: Implement MemoryStore ใน Rust
- **ผู้รับผิดชอบ**: Rust developer
- **ขั้นตอน**:
  1. สร้าง module `core/src/memory.rs`
  2. สร้าง struct `MemoryStore` ที่ถือ `rusqlite::Connection`
  3. Implement:
     - `MemoryStore::new(db_path) -> Result<Self>`
     - `insert(entry: MemoryEntry) -> Result<i64>` (return id)
     - `get_by_id(id: i64) -> Result<Option<MemoryEntry>>`
     - `query(filter: MemoryQuery) -> Result<Vec<MemoryEntry>>` โดย `MemoryQuery` ประกอบด้วย `scope`, `category`, `min_confidence`, `tags`
     - `update(entry: MemoryEntry) -> Result<()>`
     - `delete(id: i64) -> Result<()>`
  4. เพิ่ม logging และ error handling
  5. เขียน unit tests: insert หลาย entries  différente scope แล้ว query แยก scope และ confidence
  6. Integrate เข้ากับ CLI: เพิ่มคำสั่ง `specgen memory insert --key ... --scope global` (ใช้ clap)
  7. สร้าง PR
- **Acceptance**:
  - ทุก tests ผ่าน
  - สามารถใช้ CLI insert และ search memory ได้
  - Query ด้วย scope "global" ไม่คืน project entries

### งานที่ 3: Integrate MCP Tools (Memory Tools)
- **ผู้รับผิดชอบ**: TypeScript developer
- **ขั้นตอน**:
  1. ใน `app/src/mcp.ts` เพิ่ม handler สำหรับ `memory_search`, `memory_write` (โดยเรียก `cargo run -p specgen memory search ...` หรือใช้ embedded Rust library ผ่าน napi-rs ในอนาคต แต่ตอนนี้ใช้ CLI ก่อน)
  2. กำหนด input schema ตาม Protobuf
  3. ทดสอบด้วย MCP inspector (`npx @modelcontextprotocol/inspector`)
  4. เขียน integration test: เรียก tool จาก TypeScript แล้วตรวจสอบผลลัพธ์
  5. จัดการ error และ logging
  6. PR
- **Acceptance**:
  - AI client (Claude Code) สามารถเรียก `memory_search` และได้รับ JSON response ที่ถูกต้อง

---

**หมายเหตุ**: คำสั่งงานข้างต้นเป็นตัวอย่างสำหรับ Phase 1 และ 2 เท่านั้น ในแต่ละ phase จะมีงานย่อยลักษณะเดียวกัน ซึ่งควรจัดทำใน project management tool และกำหนด assignee และ deadline.
