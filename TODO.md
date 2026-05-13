# Specgen TODO

<<<<<<< HEAD
## งานที่เสร็จสมบูรณ์แล้ว ✅
- [x] Cleanup: ลบโฟลเดอร์ซ้อนและเปลี่ยนชื่อโปรเจกต์เป็น `specgen`
- [x] Core: เลือกใช้ `renderer.rs` และ `validator.rs` เวอร์ชันที่สมบูรณ์ที่สุด
- [x] Config: อัปเดต `Cargo.toml` เป็น `resolver = "2"` (Rust 2021)
- [x] Quality: กำจัด warnings ทั้งหมด (Unused imports / variables)
- [x] Testing: แก้ไข integration tests ให้ผ่าน 100%
- [x] Format: จัดรูปแบบโค้ดด้วย `cargo fmt`
- [x] Docs: สร้าง `README.md`, `TODO.md`, และ `LICENSE` (MIT)
- [x] Feature: เพิ่มคำสั่ง `new` สำหรับสร้างเทมเพลตใหม่
- [x] Feature: เพิ่มระบบ helpers (`uppercase`, `lowercase`, `trim`) ใน template engine
- [x] Feature: รองรับ output format (`JSON/YAML`) ในคำสั่ง `generate`

## งานที่ต้องทำต่อ

### โครงสร้างและ canonical files
- [ ] ตรวจสอบโครงสร้าง repo จริงด้วย `ls` และ `find`
- [ ] ยืนยันไฟล์/โฟลเดอร์ที่มีอยู่จริงและไฟล์ที่ยังขาด
- [ ] สร้างหรืออัปเดต canonical files ของโปรเจกต์:
  - [ ] `PROJECTS/<project-id>/SPEC.md`
  - [ ] `PROJECTS/<project-id>/PLAN.md`
  - [ ] `PROJECTS/<project-id>/VISION.md`
  - [ ] `PROJECTS/<project-id>/RULE.md`
  - [ ] `PROJECTS/<project-id>/TODO.md`
  - [ ] `PROJECTS/<project-id>/NEXT.md`
  - [ ] `PROJECTS/<project-id>/DESIGN.md`
  - [ ] `PROJECTS/<project-id>/ARCHITECTURE.md`
  - [ ] `PROJECTS/<project-id>/PROTOCOL.md`

- [ ] สร้างหรืออัปเดตไฟล์ระดับระบบ:
  - [ ] `GLOBAL/PLAN.md`
  - [ ] `GLOBAL/VISION.md`
  - [ ] `GLOBAL/RULE.md`
  - [ ] `GLOBAL/index-track.json`
  - [ ] `GLOBAL/templates/track-schema.json`
  - [ ] `GLOBAL/templates/index-schema.json`
  - [ ] `GLOBAL/templates/report-schema.json`

### เนื้อหา canonical spec
- [ ] เขียน `SPEC.md` ให้มี:
  - [ ] Objective
  - [ ] Run & Operate
  - [ ] Stack
  - [ ] Where things live
  - [ ] Architecture decisions
  - [ ] Product
  - [ ] User preferences
  - [ ] Gotchas
  - [ ] Pointers
  - [ ] Validation
  - [ ] Related templates
  - [ ] Related tests
  - [ ] Memory notes
  - [ ] Handoff notes

- [ ] เขียน `PLAN.md` ให้ระบุ:
  - [ ] phase-based implementation
  - [ ] file tree ที่ต้องมี
  - [ ] scaffold-first rules
  - [ ] placeholder rules
  - [ ] validation checkpoints

- [ ] เขียน `RULE.md` ให้ระบุ:
  - [ ] source-of-truth policy
  - [ ] no-guessing policy
  - [ ] fallback policy
  - [ ] validate-before-merge policy
  - [ ] minimal-change policy

### Template library
- [ ] สร้างหรืออัปเดต template library:
  - [ ] `templates/spec/SPEC.md`
  - [ ] `templates/prompt/prompt-transform.md`
  - [ ] `templates/loop/protocol-loop.md`
  - [ ] `templates/loop/track-template.md`
  - [ ] `templates/loop/verify-template.md`
  - [ ] `templates/fragments/common-context.md`
  - [ ] `templates/fragments/output-contract.md`
  - [ ] `templates/fragments/marker-list.md`
  - [ ] `templates/fragments/validation-contract.md`
  - [ ] `templates/fragments/handoff-contract.md`

- [ ] ทำให้ `prompt-transform.md` มี:
  - [ ] input contract
  - [ ] output contract
  - [ ] processing steps
  - [ ] safety rules
  - [ ] style rules
  - [ ] fallback behavior

- [ ] ทำให้ `protocol-loop.md` มี:
  - [ ] setup
  - [ ] track selection
  - [ ] track execution
  - [ ] verify
  - [ ] sync
  - [ ] cleanup
  - [ ] failure policy

### Policy files
- [ ] สร้างหรืออัปเดต policy files:
  - [ ] `policies/markers.md`
  - [ ] `policies/versioning-policy.md`
  - [ ] `policies/memory-rules.md`
  - [ ] `policies/docs-rules.md`
  - [ ] `policies/env-rules.md`
  - [ ] `policies/review-rules.md`
  - [ ] `policies/security-rules.md`
  - [ ] `policies/prompt-rules.md`

- [ ] มาตรฐาน marker:
  - [ ] `DOC:*`
  - [ ] `LEARN:*`
  - [ ] `REVIEW:*`
  - [ ] `DESIGN:*`
  - [ ] `SEC:*`
  - [ ] `PLAN:*`
  - [ ] `LOOP:*`

- [ ] frontmatter standard:
  - [ ] `name`
  - [ ] `description`
  - [ ] `version`
  - [ ] `owner`
  - [ ] `created`
  - [ ] `updated`
  - [ ] `tags`
  - [ ] `scope`

### Docs กลาง
- [ ] สร้างหรืออัปเดต docs กลาง:
  - [ ] `docs/jules.md`
  - [ ] `docs/cookbook.md`
  - [ ] `docs/workflow.md`
  - [ ] `docs/handoff.md`
  - [ ] `docs/memory/import-map.md`
  - [ ] `docs/memory/tree.md`

- [ ] เขียน `docs/jules.md` ให้บอก:
  - [ ] วิธีอ่านสเปก
  - [ ] วิธีทำ scaffold-first
  - [ ] วิธีใช้ placeholder
  - [ ] วิธี validate
  - [ ] วิธีรายงานผลลัพธ์

### Automation scripts
- [ ] สร้างหรืออัปเดต scripts:
  - [ ] `scripts/generate-spec.sh`
  - [ ] `scripts/generate-prompt.sh`
  - [ ] `scripts/generate-docs.sh`
  - [ ] `scripts/validate-all.sh`
  - [ ] `scripts/validate-tracks.sh`
  - [ ] `scripts/validate-index.sh`
  - [ ] `scripts/scaffold-template.sh`
  - [ ] `scripts/run-headless.sh`
  - [ ] `scripts/gcommit.sh`

- [ ] script ทุกตัวต้อง:
  - [ ] ตรวจ input ก่อนทำงาน
  - [ ] fail fast เมื่อ error
  - [ ] ตรวจ exit status
  - [ ] รายงานผลลัพธ์ให้ชัดเจน

### Runtime / loop state
- [ ] สร้างหรืออัปเดต runtime files:
  - [ ] `GLOBAL/index-track.json`
  - [ ] `PROJECTS/<project-id>/tracks/*.json`
  - [ ] `PROJECTS/<project-id>/verify/*.md`
  - [ ] `PROJECTS/<project-id>/log/*.log`

- [ ] ตรวจ schema ของ:
  - [ ] track
  - [ ] index
  - [ ] report

### Tests
- [ ] สร้างหรืออัปเดต test suites:
  - [ ] `tests/integration/`
  - [ ] `tests/template-tests/`
  - [ ] `tests/snapshot/`
  - [ ] `tests/fixtures/`

- [ ] เพิ่ม test สำหรับ:
  - [ ] schema validation
  - [ ] template structure
  - [ ] prompt consistency
  - [ ] integration workflow
  - [ ] snapshot output

### Memory import support
- [ ] เพิ่ม support สำหรับ memory import processor แบบ `@file.md`
- [ ] แยก memory/policy/prompt content เป็น fragments
- [ ] ตรวจ circular import protection
- [ ] ตรวจ path validation
- [ ] ยืนยันว่า import tree อ่านได้และปลอดภัย

### Config and formatting
- [ ] ตั้งค่า config สำหรับ:
  - [ ] Rust / Cargo
  - [ ] Markdown
  - [ ] JSON / YAML / TOML
  - [ ] shell scripts
  - [ ] template helpers
  - [ ] headless automation

- [ ] จัด format ให้สม่ำเสมอสำหรับ:
  - [ ] Markdown
  - [ ] Rust
  - [ ] JSON
  - [ ] YAML
  - [ ] TOML
  - [ ] shell scripts

### Integration and completion
- [ ] อัปเดต README / docs ให้สะท้อนโครงสร้างใหม่ทั้งหมด
- [ ] เพิ่มระบบสร้างไฟล์ใหม่จากเทมเพลต (`new`, scaffold, generator)
- [ ] รองรับการ validate ทุกไฟล์ที่สร้างจาก template ก่อน commit
- [ ] อัปเดต TODO นี้เมื่อมีงานใหม่หรือสถานะเปลี่ยน

## งานที่รอการดำเนินการ (Future Tasks) ⏳
- [ ] เพิ่มความสามารถในการลงทะเบียน custom helper ภายนอก
- [ ] พัฒนา dashboard สำหรับดูรายการเทมเพลตผ่านเว็บ (Tauri/Wry)
- [ ] เพิ่มระบบ plugin สำหรับเชื่อมต่อกับ LLM API โดยตรง

## กฎการทำงาน
- ห้ามเดาเนื้อหาสำคัญถ้ายังไม่มีข้อมูล
- ถ้าไม่พอ ให้สร้าง placeholder หรือ scaffold ก่อน
- ถ้ามีไฟล์จริงอยู่แล้ว ให้ตรวจความสอดคล้องก่อนแก้
- ต้องตรวจผลลัพธ์ของทุก command / script ที่รัน
- ถ้าคำสั่งใด fail ให้หยุดและรายงานทันที
- ทุกไฟล์ที่สร้างต้องสอดคล้องกับ PLAN.md และ schema ที่เกี่ยวข้อง
=======
## งานที่เสร็จสมบูรณ์แล้ว ✅ (Checkpoint: 2024-05-19)
- [x] **Cleanup:** ลบโฟลเดอร์ซ้อนและเปลี่ยนชื่อโปรเจกต์เป็น `specgen`
- [x] **Core:** เลือกใช้ `renderer.rs` และ `validator.rs` เวอร์ชันที่สมบูรณ์ที่สุด
- [x] **Config:** อัปเดต `Cargo.toml` เป็น `resolver = "2"` (Rust 2021)
- [x] **Quality:** กำจัด Warnings ทั้งหมด (Clippy passed 100%)
- [x] **Testing:** แก้ไข Integration & Unit Tests ให้ผ่าน 100% (46 unit, 9 integration)
- [x] **Format:** จัดรูปแบบโค้ดด้วย `cargo fmt`
- [x] **Docs:** สร้าง `README.md`, `TODO.md`, `LICENSE`, และตั้งค่า `.gitignore` ให้ครอบคลุม
- [x] **Feature:** เพิ่มคำสั่ง `new` สำหรับสร้างเทมเพลตใหม่
- [x] **Feature:** เพิ่มระบบ Helpers (`uppercase`, `lowercase`, `trim`) ใน Template Engine
- [x] **Feature:** รองรับ Output Format (JSON/YAML) ในคำสั่ง `generate`
- [x] **Feature:** เพิ่มคำสั่ง `convert` สำหรับแปลงรูปแบบเทมเพลต (JSON, Markdown, และ TOML)
- [x] **Feature:** เพิ่มระบบ Serialize Markdown (แปลง JSON กลับเป็น Markdown)
- [x] **Monorepo Consolidation:** รวม schema, core-engine, interface และ conductor เข้ามาในโปรเจกต์เดียวกัน
- [x] **Rust Proto Gen:** ตั้งค่า `prost-build` และตรวจสอบการสร้างโค้ด Rust จาก Proto แล้ว
- [x] **Core Logic:** ย้าย Rule Engine และ DB Logic มาใช้โครงสร้างใหม่ (SQLite Implementation in Rust)

## งานที่กำลังดำเนินการ 🚧
- [ ] **Unified Interface Integration:** เชื่อมต่อ CLI เข้ากับระบบ Memory/Rule Engine ใหม่
  - [x] วางโครงสร้างคำสั่ง CLI (`db`, `rule`, `agent`, `index`, `search`)
  - [ ] พัฒนา Implementation ของคำสั่ง CLI ให้ทำงานกับ `MemoryStore`
- [ ] **Semantic Search Implementation:**
  - [x] วางโครงสร้าง `CodeSense` ใน `core`
  - [ ] เชื่อมต่อกับ Ollama (nomic-embed-text) เพื่อสร้าง index จริง

## งานที่รอการดำเนินการ (Future Tasks) ⏳
- [ ] **MCP Implementation:** พัฒนาส่วนเชื่อมต่อ MCP
  - [x] **MCP Server (TS):** สร้าง MCP Server เบื้องต้นโดยใช้ Generated Proto และ DB (Neon)
  - [ ] **MCP Server (Rust):** ย้ายการทำงานของ MCP มาที่ Rust Engine โดยตรง
- [ ] **Hybrid Cloud Integration (model/):**
  - [ ] ออกแบบระบบ **Artifact Sync** (SSH/Patch/S3) ระหว่าง Termux และ Modal Cloud
  - [ ] เพิ่มคำสั่ง `sync` เพื่อส่ง Patch/File ไปรันงานหนักบน Cloud
- [ ] **Template V2:** ออกแบบระบบเทมเพลตเวอร์ชันใหม่เพื่อรองรับฟีเจอร์ขั้นสูง:
  - [ ] **Markdown Parser:** การจัดการ Comment (`<!-- -->`) และ Front Matter (YAML/TOML)
  - [ ] **Schema:** การกำหนดรูปแบบ Input แบบใหม่ (Single/Multiple Choice)
- [ ] **สถาปัตยกรรม Agent (Asynchronous Delegation):**
  - [ ] **Background Execution:** รัน Worker เป็น Background Process
  - [ ] **Notification System:** แจ้งเตือนผ่าน OS เมื่อ Worker ทำงานเสร็จ
>>>>>>> 60e5226 (feat: unify specgen architecture into monorepo and stabilize core)
