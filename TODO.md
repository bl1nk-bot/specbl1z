# Specgen TODO

## งานที่เสร็จสมบูรณ์แล้ว ✅
- [x] **Cleanup:** ลบโฟลเดอร์ซ้อนและเปลี่ยนชื่อโปรเจกต์เป็น `specgen`
- [x] **Core:** เลือกใช้ `renderer.rs` และ `validator.rs` เวอร์ชันที่สมบูรณ์ที่สุด
- [x] **Config:** อัปเดต `Cargo.toml` เป็น `resolver = "2"` (Rust 2021)
- [x] **Quality:** กำจัด Warnings ทั้งหมด (Unused imports/variables)
- [x] **Testing:** แก้ไข Integration Tests ให้ผ่าน 100%
- [x] **Format:** จัดรูปแบบโค้ดด้วย `cargo fmt`
- [x] **Docs:** สร้าง `README.md`, `TODO.md` และ `LICENSE` (MIT)
- [x] **Feature:** เพิ่มคำสั่ง `new` สำหรับสร้างเทมเพลตใหม่
- [x] **Feature:** เพิ่มระบบ Helpers (`uppercase`, `lowercase`, `trim`) ใน Template Engine
- [x] **Feature:** รองรับ Output Format (JSON/YAML) ในคำสั่ง `generate`

## งานที่รอการดำเนินการ (Future Tasks) ⏳
- [ ] เพิ่มความสามารถในการลงทะเบียน Custom Helper ภายนอก
- [ ] พัฒนา Dashboard สำหรับดูรายการเทมเพลตผ่าน Web (Tauri/Wry)
- [ ] เพิ่มระบบ Plugin สำหรับเชื่อมต่อกับ LLM API โดยตรง
