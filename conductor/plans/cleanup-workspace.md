# Workspace Cleanup Plan (Revised)

## Objective
จัดระเบียบ Workspace โดยเริ่มจากการลดความวุ่นวาย (Noise) ด้วยการกวาดขยะที่ลบได้ทันทีก่อน จากนั้นจึงเข้าจัดการซ่อมแซมโค้ด และสุดท้ายคือจัดการสถานะ Git ให้สะท้อนความเป็นจริง

## Implementation Steps

### Phase 1: กวาดขยะและจัดระเบียบเบื้องต้น (Immediate Garbage Collection)
- ย้ายสคริปต์แก้ปัญหาเฉพาะหน้า (`fix_cli_memory.py`, `fix_codesense_calls.py`, `fix_compilation.py`, `fix_memory_complete.py`, `fix_memory_functions.py`, `fix_memory_topic.py`, `fix_memoryentry_topic.py`) เข้าไปเก็บในโฟลเดอร์ `scripts/`
- ลบไฟล์เอกสารและบันทึกชั่วคราวที่หมดความจำเป็น:
  - `BACKGROUND_TASK_STATUS.md`
  - `GOAL_FIX_COMPILATION.md`
  - `QWEN.md`

### Phase 2: ซ่อมแซมระบบ (Fix Broken Code)
- แก้ไขปัญหาคอมไพล์ใน `core/src/memory.rs` (ในฟังก์ชัน `log_audit` ที่ใช้ `serde_json` ผิดพลาดกับ Protobuf struct)
- ยืนยันว่าโค้ดคอมไพล์ผ่านด้วย `cargo check`

### Phase 3: จัดการสิ่งตกค้างและอัปเดต Git (Cleanup Artifacts & Sync Git)
- ลบซากไฟล์ patch ที่ตกค้าง: `core/src/memory.rs.orig`, `core/src/memory.rs.rej`, `memory_fix.patch`
- นำโฟลเดอร์เก่าที่ถูกลบออกจาก Git: `git rm -r app craft`
- เพิ่มโฟลเดอร์โครงสร้างใหม่เข้า Git: `git add server mcp-server agents skills`
- ลบไฟล์ที่ไม่ได้ใช้แล้วและถูก report ใน Git status (เช่น `ARCHITECT.md`, `CHANGELOG.md`, `LICENSE`, `CI_CD_STRATEGY.md`) ออกจากระบบ tracking

## Verification
- รัน `git status` ต้องแสดงเฉพาะรายการไฟล์ที่เปลี่ยน/ย้าย/ลบ พร้อมให้ commit และไม่มีไฟล์ Untracked รกหูรกตา
- รัน `cargo check` และ `cargo clippy` ต้องไม่มีข้อผิดพลาด