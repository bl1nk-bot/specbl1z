# ข้อกำหนดเฉพาะของระบบ (System Specification) — specgen v3

## 1. วิสัยทัศน์ (Vision)
สร้างแพลตฟอร์ม **AI-Native Spec-Driven Development** ที่รวดเร็ว โปร่งใส และทนทาน โดยใช้สถาปัตยกรรม Rust-centric ที่รวมประสิทธิภาพของ Core Engine เข้ากับ MCP Server โดยตรง พร้อมระบบจัดการเวอร์ชันข้อมูลแบบ Git เพื่อการทำงานข้ามแพลตฟอร์มอย่างไร้รอยต่อ

## 2. วัตถุประสงค์ของระบบ (v3 Consolidation)

### 2.1 Unified Architecture (Monorepo)
- รวม `core`, `cli`, และ `mcp-server` เข้าด้วยกันใน Rust เพื่อลดความซับซ้อนและเพิ่มประสิทธิภาพ
- **Performance**: ลด latency ของ MCP tool calls ลง 3 เท่า (150ms → <50ms) โดยใช้การเรียกใช้ฟังก์ชันโดยตรง (Direct FFI) แทนการสร้าง process ใหม่

### 2.2 Open Bridge Architecture (Non-Black-Box)
- CLI ทำหน้าที่เป็นสะพานที่โปร่งใสระหว่าง Core Engine และสคริปต์ภายนอก
- รองรับ **Standardized JSON I/O** ในทุกคำสั่งเพื่อให้ง่ายต่อการใช้ร่วมกับ `jq` หรือ Python
- สนับสนุน **Python First** เป็นทางเลือกหลักในการเขียนสคริปต์ Workflow ที่ซับซ้อนแทน Bash/PS1

### 2.3 Distributed Data Integrity (Git-like Versioning)
- จัดเก็บข้อมูลใน SQLite (`craft.db`) โดยใช้ระบบ **Append-Only Immutable Rows**
- มีการทำ **Hashing (SHA-256)** และ **Parent-tracking** ในทุกแถวข้อมูลเพื่อป้องกันความขัดแย้ง (Conflicts) เมื่อ Sync ข้าม OS
- รองรับการตรวจจับและแก้ไข Conflict ในระดับฐานข้อมูลเหมือนการทำ `git merge`

### 2.4 Advanced Memory & Context Management
- จัดเก็บความจำแบบแยก `scope` (global, project, session, working, policy, identity)
- ระบบ **Dependency Bootstrapper**: ตรวจสอบและติดตั้งเครื่องมือจำเป็น (`jq`, `rg`) อัตโนมัติเมื่อต้องการใช้งาน

## 3. ข้อกำหนดเชิงฟังก์ชัน (Functional Requirements)

### FR1: Unified Data Model (Protobuf)
- ใช้ `bl1nk.proto` เป็น Single Source of Truth สำหรับโครงสร้างข้อมูลทั้งหมด (MemoryEntry, PromptBlock, ContextPack, ConflictRecord, TestPlan)

### FR2: High-Performance Memory Engine
- รองรับ CRUD พร้อมระบบ Versioning และ Hashing ในตัว
- ค้นหาข้อมูลเชิงความหมาย (Semantic Search) ผ่าน Ollama embeddings โดยตรงจาก Rust

### FR3: Transparent CLI & Scripting
- ทุกความสามารถของระบบต้องเรียกใช้ได้ผ่าน CLI
- คำสั่ง `specgen run <script>` สำหรับการรันสคริปต์ภายนอกพร้อมส่งบริบทระบบ (Context Injection)

### FR4: Direct Rust MCP Server
- พัฒนา MCP Server ด้วย Rust เพื่อการเชื่อมต่อที่รวดเร็วที่สุดกับ AI Assistants (Claude Code, Cursor)

## 4. ข้อกำหนดที่ไม่ใช่ฟังก์ชัน (Non-Functional Requirements)

- **Speed**: MCP latency < 50ms p99
- **Portability**: รองรับการ Sync ฐานข้อมูลข้าม OS (Android/Termux, Windows, Linux) ผ่าน Git
- **Reliability**: ข้อมูลต้องไม่หายแม้เกิด Conflict (Append-only history)
- **Extensibility**: ผู้ใช้ต้องสามารถขยายความสามารถได้เองผ่าน Python scripts

## 5. เทคโนโลยีหลัก (Tech Stack v3)
- **Core & CLI**: Rust (with workspace consolidation)
- **MCP Server**: Rust (direct integration)
- **Dashboard/Web**: TypeScript/Hono (server/ directory)
- **Storage**: Unified SQLite with Git-like versioning schema
- **Protocol**: Protobuf (bl1nk.proto)
- **External Tools**: `jq`, `rg`, `python3` (auto-bootstrapped)
