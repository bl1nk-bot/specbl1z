# Project Mandates (Specgen)

## 🚨 Project Context & Documentation Hierarchy

Agent ต้องยึดถือเอกสารชุดนี้เป็นเข็มทิศในการทำงานเสมอ:
- [**SPEC.md**](./SPEC.md): เป้าหมายและขอบเขตงาน (ห้ามทำเกินขอบเขตที่ระบุ)
- [**PLAN.md**](./PLAN.md): ขั้นตอนการทำงาน (ห้ามข้าม Phase โดยไม่จำเป็น)
- [**ARCHITECT.md**](./ARCHITECT.md): โครงสร้างระบบและโมดูล (ต้องเขียนโค้ดให้ตรงตาม Pattern นี้)
- [**TODO.md**](./TODO.md): สถานะงานรายวัน (ต้องอัปเดตทุกครั้งที่จบงาน)

## 🏗️ Monorepo Rules
- **Single Source of Truth**: ใช้ Protobuf ใน `/schema` เป็นตัวกำหนด Interface เสมอ
- **Rust First**: Logic หลักของ Rule Engine และ DB ต้องอยู่ที่ `/core` และ `/craft`
- **Zero Warnings Policy**: โค้ด Rust ต้องผ่าน `cargo clippy -- -D warnings` และ `cargo test` 100% ก่อนส่งงาน

## 🚨 Anti-Token Waste Protocol (MANDATORY)

1. **Zero-Empty-Value Policy**: ห้ามเรียก Tool โดยไม่มี Parameter หรือ Parameter ว่างเปล่า
2. **Pre-Tool Verification**: ตรวจสอบ Path ไฟล์ด้วย `ls` หรือ `glob` ก่อนเรียก `read_file` หรือ `replace` เสมอ
3. **Loop Prevention**: หากรันคำสั่งพลาดเกิน 2 ครั้ง ให้หยุดและรายงานผู้ใช้ทันที ห้ามลองสุ่มทางแก้
4. **Context Efficiency**: ใช้ `grep_search` แทนการอ่านไฟล์ขนาดใหญ่ทั้งไฟล์

---
*Updated on: 2024-05-19*
*Reason: Update documentation hierarchy and monorepo standards.*
