# สถาปัตยกรรมระบบ (Architecture) — specgen v2

## 1. ภาพรวมสถาปัตยกรรม

specgen v2 เป็น platform แบบ hybrid monorepo ที่รวม:

- **Rust Core Engine**: จัดการ memory, policy, prompt composition, template rendering, และ semantic search
- **TypeScript/Hono Server**: ให้ REST API และ MCP server สำหรับ AI client
- **Protobuf Bridge**: แชร์โครงสร้างข้อมูลระหว่าง Rust และ TS
- **Storage**: SQLite (local) + Neon (cloud) สำหรับ persistent data; ในหน่วยความ remember für session/working state

## 2. Architectural Layers

### 2.1 Cold Storage (Persistent)
- ประกอบด้วย: Memory Store (MemoryEntry, MemoryScope), Craft DB (folders, documents, blocks), Policy Store (กฎ)
- จัดเก็บใน SQLite (`memory.db`, `craft.db`) หรือ Neon
- มีเวอร์ชันและ audit log

### 2.2 Warm Index
- Semantic search index (จาก `core/src/sense.rs`) ที่เก็บ embeddings ของโค้ดและเอกสาร
- ใช้ Ollama embeddings (model `qwen3-embedding:0.6b`)
- ใช้ค้นหา memory หรือโค้ดที่เกี่ยวข้องโดยใช้ cosine similarity

### 2.3 Session Cache
- เก็บ ContextPack และ PromptBlock ที่กำลังใช้งานใน session ปัจจุบัน
- อายุสั้น หมดอายุเมื่อ session จบ
- จัดการใน memory ของ process (Rust HashMap หรือ Redis ในอนาคต)

### 2.4 Working Buffer
- ใช้ชั่วคราวระหว่างการ compose prompt หรือ evaluate policy
- ไม่ถูกบันทึกถาวร ยกเว้นผ่านกระบวนการ validate และ persist

### 2.5 Policy Store
- เก็บกฎทั้งหมด (hard, soft, context, safety) ในรูปแบบ structured data
- มี versioning และ effective date
- Policy Engine โหลดจาก Policy Store เมื่อเริ่มต้นและเมื่อมีการอัปเดต

## 3. องค์ประกอบหลัก (Components)

### 3.1 Rust Core (`core/`)

- **`memory.rs`**: Memory Engine
  - `MemoryStore` struct
  - methods: `insert`, `query(scope, category, confidence_min)`, `link`, `audit_trail`
  - ใช้ `rusqlite` สำหรับ SQLite, รองรับ Neon ผ่าน adapter

- **`policy.rs`**: Policy Engine
  - `PolicyEngine` struct
  - `RuleRegistry`, `Evaluator`, `ConflictResolver`, `ActionGate`, `AuditLogger`
  - ใช้ `serde` สำหรับ serialize rules
  - Evaluator รับ `ContextPack` และ returns `PolicyResult` (allow/deny/modify)

- **`composer.rs`**: Prompt Composer
  - `Composer` struct
  - `compose(blocks: Vec<PromptBlock>) -> ContextPack`
  - เรียงลำดับตาม priority, trim, แทรก guardrails

- **`parser/`, `renderer.rs`, `validator.rs`** (เดิม) — template engine
  - ปรับปรุงให้รองรับ template-to-test-plan generation: เพิ่ม `testgen.rs` ที่รับ `WorkflowTemplate` และสร้าง `TestPlan` structure

- **`sense.rs`** (เดิม) — semantic search
  - ขยายให้ค้นหา memory entries ได้

- **`rules_engine.rs`** (เดิม) — สำหรับ coding standards
  - ใช้ร่วมกับ policy engine ได้

- **`db.rs`** (เดิม) — SQLite สำหรับ bl1nk rules
  - ปรับปรุงให้เก็บ memory entries และ policy rules ได้

### 3.2 TypeScript App (`app/`)

- **`server.ts`**: Hono server
  - REST endpoints สำหรับ UI/dashboard
  - ใช้ Neon สำหรับ cloud memory store

- **`mcp.ts`**: MCP server (stdio)
  - Tools: `memory_*`, `prompt_*`, `policy_*`, `template_*`, `sense_search`, `triage_security`
  - เรียกใช้ Rust CLI (`cargo run -p specgen memory ...`) สำหรับ operations ที่หนัก

- **`craft_tools.ts`**: MCP definitions สำหรับ Craft DB (folders, documents, blocks)

- **`db.ts`**: Neon connection pool

### 3.3 CLI (`cli/`)

- เพิ่ม subcommands:
  - `specgen memory <crud>` — จัดการ memory
  - `specgen policy <evaluate|check|list>` — evaluate policy
  - `specgen compose <task-id>` — compose prompt
  - `specgen testplan <template>` — สร้าง test plan จาก template
  - `specgen work-instructions <template> <testplan>` — สร้าง work instructions

### 3.4 Schema & Protobuf

- `bl1nk.proto`: เพิ่ม messages สำหรับ MemoryEntry, MemoryScope (enum), PromptBlock, ContextPack, ConflictRecord, TestPlan, WorkInstruction
- Generated code: `core/src/bl1nk.rs` (Rust) และ `app/src/generated/bl1nk.ts` (TS)

## 4. Data Flow (Request Processing)

```text
[AI Client (MCP)]
        ↓
[mcp.ts] receives tool call
        ↓
[Classification & Routing]
        ↓
[CLI (Rust)] if heavy, or [Rust Core via FFI/embedded] (future)
        ↓
[Policy Engine] evaluate
        ↓
[Memory Engine] retrieve/write
        ↓
[Composer] if prompt compose needed
        ↓
[Response back to AI Client]
```

สำหรับการสร้าง spec/test/workflow:

1. AI client เรียก template.create_from_prompt(prompt)
2. MCP handler ส่งให้ Rust CLI หรือ HTTP endpoint ภายใน
3. Rust core ใช้ LLM (เรียกผ่าน API ภายใน) สร้าง workflow template (markdown/json) โดยอิงจาก coding standards
4. บันทึก template ลง Craft DB
5. AI client เรียก template.generate_test_plan(template_id)
6. Rust testgen.rs วิเคราะห์ template และสร้าง TestPlan structure (JSON)
7. AI client เรียก template.generate_work_instructions(template_id, testplan_id)
8. Rust composer สร้างเอกสาร Markdown รวม spec + test plan + coverage target

## 5. Integration Points

- กับ Ollama: ใช้ HTTP POST /api/embeddings จาก身在 Rust core (ผ่าน reqwest) สำหรับ semantic search
- กับ GitHub CLI: สำหรับ security triage
- กับ Neon: ผ่าน TypeScript @neondatabase/serverless หรือผ่าน Rust tokio-postgres ในอนาคต
- กับ MCP: ทุก function ใน app/src/mcp.ts ต้องมี schema แบบเดียวกับ Protobuf

## 6. Security Design

- Memory ที่มี access_level=private ต้องไม่ถูกส่งออกไปนอก user scope
- PII redaction ใน prompt blocks ก่อนส่งให้ LLM
- Policy rules ห้ามถูก override โดย AI client โดย没有 human confirmation
- ทุก MCP tool ต้องผ่าน policy engine evaluation ก่อน execute (โดยเฉพาะ write operations)

## 7. ข้อควรพิจารณาในการขยาย (Extensibility)

- Policy rules สามารถเพิ่มได้เรื่อย ๆ โดยไม่ต้องเปลี่ยนโค้ด (ใช้ external config)
- Template parser รองรับ plugin format (เช่น custom tags)
- MCP tools สามารถลงทะเบียนเพิ่มแบบ dynamic
- Memory engine สามารถเปลี่ยน backend เป็น Vector DB ได้ในอนาคต (สำหรับ retrieval augmented generation)

## 8. ución de Capas

```
┌─────────────────────────────────────────┐
│      AI Client (Claude Code / Cursor)   │
└───────────────┬─────────────────────────┘
                │ MCP (stdio)
┌───────────────▼─────────────────────────┐
│      MCP Server (TypeScript/Hono)       │
│  — tool dispatch & validation           │
│  — protocol compliance                  │
└───────────────┬─────────────────────────┘
                │ CLI invocation or HTTP
┌───────────────▼─────────────────────────┐
│      Rust Core Engine                   │
│  ┌───────────────────────────────────┐  │
│  │  Policy Engine (gate, evaluate)   │  │
│  ├───────────────────────────────────┤  │
│  │  Memory Engine (CRUD, query)      │  │
│  ├───────────────────────────────────┤  │
│  │  Prompt Composer (assemble)       │  │
│  ├───────────────────────────────────┤  │
│  │  Template Engine (render/parse)   │  │
│  ├───────────────────────────────────┤  │
│  │  Semantic Search (sense.rs)       │  │
│  └───────────────────────────────────┘  │
└───────────────┬─────────────────────────┘
                │ SQL / HTTP
┌───────────────▼─────────────────────────┐
│      Storage Layer                       │
│  — SQLite (local): memory.db, craft.db  │
│  — Neon (cloud): Postgres via HTTP      │
│  — Ollama: embeddings API (localhost)   │
└─────────────────────────────────────────┘
```
