# Specgen (Unified bl1nk Engine)

ระบบจัดการ Workflow, Memory Store และ Coding Standards ที่รวมประสิทธิภาพของ Rust เข้ากับความยืดหยุ่นของ TypeScript ภายใต้สถาปัตยกรรม Monorepo

## 🏗️ สถาปัตยกรรม (Architecture)

โปรเจกต์นี้ขับเคลื่อนด้วยหลักการ **Single Source of Truth** โดยใช้ Protobuf เป็นตัวเชื่อมกลาง:

- **`/core`**: Engine หลัก (Rust) จัดการ Template, Memory Store, และ Rules Engine
- **`/cli`**: เครื่องมือ Command Line (Rust) สำหรับจัดการ Database, Rules, และ Agents
- **`/craft`**: Local Database Layer (SQLite) สำหรับการเก็บความจำเชิงโครงสร้าง
- **`/schema`**: Protobuf Definitions สำหรับการแลกเปลี่ยนข้อมูลระหว่างภาษา
- **`/app`**: ระบบ Interface และ MCP Server (TypeScript)
- **`/conductor`**: การบริหารจัดการโปรเจกต์ผ่าน **Conductor Protocol**

## 📄 เอกสารสำคัญ (Core Documents)

- [**SPEC.md**](./SPEC.md): รายละเอียดข้อกำหนดและเป้าหมายของระบบ (What & Why)
- [**PLAN.md**](./PLAN.md): แผนการดำเนินงานและ Phase ต่างๆ ของโปรเจกต์ (How)
- [**ARCHITECT.md**](./ARCHITECT.md): รายละเอียดการออกแบบระบบและโครงสร้างข้อมูล (Design)
- [**TODO.md**](./TODO.md): รายการงานที่เสร็จแล้วและงานที่กำลังดำเนินการ (Progress)

## 🚀 สถานะปัจจุบัน (Current Status)

- ✅ **Core Stability**: โค้ดผ่านการตรวจสอบ `fmt`, `clippy` และรันเทสผ่าน 100% (46 unit, 9 integration tests)
- ✅ **Template Engine**: รองรับการแปลงรูปแบบ JSON, Markdown และ TOML อย่างสมบูรณ์
- ✅ **Monorepo Consolidation**: รวบรวมทุกโมดูลเข้ามาอยู่ในโครงสร้างเดียวกันพร้อมระบบ Auto-gen Proto
- 🚧 **In Progress**: การเชื่อมต่อ CLI เข้ากับระบบ Memory Store และการทำ Semantic Search

## 🛠️ การเริ่มใช้งาน

### 1. เครื่องมือ CLI (Rust)
```bash
# ตรวจสอบคำสั่งที่มีให้ใช้งาน
cargo run -p specgen -- --help
```

### 2. การจัดการฐานข้อมูล
```bash
# เริ่มต้นฐานข้อมูลใหม่
cargo run -p specgen -- db init

### 3. คำสั่งเพิ่มเติม (New Commands)

#### Task Worker
```bash
# เพิ่มงานใหม่ (task)
cargo run -p specgen -- task add --title "echo 'Hello World'"

# ดูรายการงาน
cargo run -p specgen -- task list

# รัน Worker (后台 poll ทุก 10 วินาที)
cargo run -p specgen -- task worker --poll-interval 10
```

#### Cron Job Integration
```bash
# เพิ่ม Cron job
cargo run -p specgen -- cron add --expression "*/5 * * * *" --task-id 1

# ดู Cron jobs ที่ติดตั้ง
cargo run -p specgen -- cron list

# ลบ Cron jobs ทั้งหมด (ต้องยืนยัน)
cargo run -p specgen -- cron clear
```

#### Sync (Push to remote)
```bash
# Generate patch และส่ง
cargo run -p specgen -- sync --push --endpoint http://your-server:8000/sync

# Dry run (ไม่ส่ง)
cargo run -p specgen -- sync --push --dry-run
```

#### Indexing (Ollama)
```bash
# ทดสอบ Ollama connection
cargo run -p specgen -- index --verify-connection

# Index all blocks (non-background)
cargo run -p specgen -- index --all

# Index in background daemon
cargo run -p specgen -- index --background --ollama-url http://localhost:11434 --model nomic-embed-text
```

```

## 📜 ใบอนุญาต (License)
MIT License - ดูรายละเอียดในไฟล์ [LICENSE](./LICENSE)