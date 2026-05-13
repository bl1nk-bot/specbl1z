# ข้อกำหนดเฉพาะของระบบ (System Specification) — specgen v2

## 1. วิสัยทัศน์ (Vision)
สร้างแพลตฟอร์ม **Spec-Driven Development** ที่มี AI เป็นหัวใจหลักในการช่วยออกแบบข้อกำหนด (Spec), วางแผนทดสอบ (Test Plan), และสร้างคำสั่งงาน (Work Instructions) โดยอัตโนมัติ โดยมีพื้นฐานจากระบบจัดการ Memory ส่วนบุคคลแบบแยกชั้น และเครื่องยนต์การประกอบ Prompt ที่ปลอดภัย

## 2. วัตถุประสงค์ของระบบ

### 2.1 Personal Memory Management
- จัดเก็บความจำผู้ใช้แบบแยก `scope` อย่างน้อย 6 ระดับ: global, project, session, working, policy, identity
- เรียกคืนได้ตาม scope และ confidence
- ป้องกันการปนกันระหว่าง memory หลายชั้น
- รองรับเวอร์ชัน, การอ้างอิง, และ audit trail
- ทุก memory entry ต้องมี `provenance` (แหล่งที่มา) และ `confidence` (ระดับความมั่นใจ)

### 2.2 Prompt Composition & Agent Orchestration
- ประกอบ prompt จากหลายแหล่ง (identity, policy, task, memory, tool) โดยมีลำดับความสำคัญ
- รักษา identity ของ agent ให้คงที่ตลอด session
- ป้องกัน prompt drift, context bleed, hallucinated memory, instruction confusion
- ทำให้ agent แยกแยะได้ว่า "อะไรคือข้อเท็จจริง", "อะไรคือคำสั่ง", "อะไรคือบริบทชั่วคราว"

### 2.3 Template-Driven Workflow (จาก specgen เดิม)
- สร้าง, ตรวจสอบ, และ render เทมเพลตในหลายรูปแบบ (JSON, Markdown, TOML)
- แปลงเทมเพลตเป็น Test Plan และ Work Instructions โดยใช้ AI
- ใช้มาตรฐานการเขียนโค้ด (coding standards) ในการตรวจสอบ template content

### 2.4 Semantic Code Search
- ทำดัชนีโค้ดโปรเจกต์ด้วย Ollama embeddings
- ค้นหาชิ้นส่วนโค้ดที่เกี่ยวข้องกับ requirement เพื่อช่วยออกแบบ test case

### 2.5 Security Triage
- คัดกรองแจ้งเตือนความปลอดภัยจาก GitHub ด้วย weighted scoring และ slop detection

## 3. ข้อกำหนดเชิงฟังก์ชัน (Functional Requirements)

### FR1: Data Model
- FR1.1: ต้องมี `MemoryEntry` ที่เก็บ `id`, `scope`, `category`, `key`, `value`, `source`, `confidence`, `created_at`, `updated_at`, `version`, `status`, `tags`, `owner`, `access_level`, `provenance`, `expires_at`
- FR1.2: ต้องมี `PromptBlock` ที่เก็บ `id`, `type` (system, policy, identity, memory, task, tool_instruction, plan, user_input, guardrail, output_format), `priority`, `scope`, `content`, `source`, `constraints`, `dependencies`, `version`
- FR1.3: ต้องมี `ContextPack` ที่รวบรวม blocks สำหรับการ execute หนึ่งครั้ง
- FR1.4: ต้องมี `ConflictRecord` สำหรับบันทึกความขัดแย้งระหว่าง blocks

### FR2: Memory Engine
- FR2.1: รองรับ CRUD operations โดย filter ตาม scope, category, confidence, tags
- FR2.2: สามารถ link memory entries เข้าด้วยกันเพื่อสร้างความสัมพันธ์
- FR2.3: มี audit log ทุกครั้งที่มีการ write/update/delete
- FR2.4: รองรับ TTL (expires_at) สำหรับ transient memory
- FR2.5: ต้องป้องกันการเขียนทับ identity memory โดยไม่ตั้งใจ

### FR3: Policy Engine
- FR3.1: มี Rule Registry สำหรับ hard rules, soft rules, context rules, safety rules
- FR3.2: Evaluator ที่ตรวจสอบ memory retrieval ทุกครั้งว่าไม่ละเมิด policy
- FR3.3: Conflict Resolver ที่ตัดสินใจเมื่อมี rule ขัดแย้งกัน (เช่น project memory ชนะ global memory)
- FR3.4: Action Gate ที่ควบคุมว่าการกระทำใด (เช่น write memory, execute tool) ได้รับอนุญาต

### FR4: Prompt Composer
- FR4.1: ประกอบ prompt ตามลำดับ: identity → policy → task → relevant memory → tool instructions → output constraints
- FR4.2: สามารถ trim context ให้พอดีกับ token limit โดยคงความสำคัญ
- FR4.3: แสดง provenance ของแต่ละ block ใน context pack

### FR5: MCP Tools
- FR5.1: `memory.search`, `memory.write`, `memory.update`, `memory.delete`, `memory.link`
- FR5.2: `prompt.compose`, `prompt.validate`, `prompt.trim`
- FR5.3: `policy.evaluate`, `policy.check_scope`, `policy.gate_action`
- FR5.4: `template.create_from_prompt`, `template.generate_test_plan`, `template.generate_work_instructions`
- FR5.5: `sense.search` (semantic search)
- FR5.6: `triage.security` (GitHub security)

### FR6: Template-to-Test Integration
- FR6.1: จาก workflow template (ขั้นตอน, rules, loop_restart, output_template) ต้องสร้าง test plan outline ได้
- FR6.2: ต้องสร้าง edge cases จาก loop_restart, critical steps, และ potential null/missing variable
- FR6.3: ต้องสร้าง E2E test scenarios ที่เชื่อม step ตามลำดับ
- FR6.4: สามารถ render work instructions document (Markdown/JSON) ที่รวม spec, test plan, และ coverage target

### FR7: AI Client Collaboration
- FR7.1: AI client (Claude Code, Cursor) สามารถเรียก MCP tools ทั้งหมดได้
- FR7.2: ระบบต้องสามารถรับ feedback จาก AI หลัง verification และปรับปรุง memory/policy ตาม (ด้วยความระมัดระวัง)
- FR7.3: ต้องมี guardrail ป้องกันไม่ให้ AI client เขียนทับ policy memory โดยตรง

## 4. ข้อกำหนดที่ไม่ใช่ฟังก์ชัน (Non-Functional Requirements)

- **Performance**: การดึง memory ต้องต่ำกว่า 100ms สำหรับ 1,000 entries
- **Scalability**: รองรับโปรเจกต์ขนาดใหญ่ (10,000+ files) สำหรับ semantic indexing
- **Security**: ห้าม PII รั่วใน prompt, ห้ามใช้ forbidden source, มี sanitization
- **Reliability**: Policy engine ต้องทำงาน 100% consistent, ห้ามมี silent failure
- **Observability**: ทุก operation ต้องมี audit log และ trace ID

## 5. ข้อจำกัด (Constraints)
- ใช้ Protobuf เป็น single source of truth สำหรับ data structures
- Core logic ใน Rust, Web API และ MCP server ใน TypeScript (Hono)
- ใช้ SQLite สำหรับ local storage, Neon (Postgres) สำหรับ cloud
- ใช้ Ollama สำหรับ embeddings (ไม่มี cloud dependency)

## 6. ข้อกำหนดเชิงสถาปัตยกรรม (Architectural Constraints)
- แยกระบบเป็น layers: Cold storage, Warm index, Session cache, Working buffer, Policy store
- State management ต้องมี persistent, session, ephemeral states แยกกัน
- ใช้ flow: Input Validation → Classification → Scope Resolution → Memory Retrieval → Policy Evaluation → Prompt Composition → Agent Execution → Observation → Memory Writeback / Audit
