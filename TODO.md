# Specgen TODO

## งานที่เสร็จสมบูรณ์แล้ว ✅
- [x] **Cleanup:** ลบโฟลเดอร์ซ้อนและเปลี่ยนชื่อโปรเจกต์เป็น `specgen`
- [x] **Core:** เลือกใช้ `renderer.rs` และ `validator.rs` เวอร์ชันที่สมบูรณ์ที่สุด
- [x] **Config:** อัปเดต `Cargo.toml` เป็น `resolver = "2"` (Rust 2021)
- [x] **Quality:** กำจัด Warnings ทั้งหมด (Unused imports/variables)
- [x] **Testing:** แก้ไข Integration Tests ให้ผ่าน 100%
- [x] **Format:** จัดรูปแบบโค้ดด้วย `cargo fmt`
- [x] **Docs:** สร้าง `README.md` และ `TODO.md` เบื้องต้น

## งานที่รอการดำเนินการ (Future Tasks) ⏳
- [ ] เพิ่มคำสั่ง `new-template` สำหรับสร้างไฟล์เทมเพลตเริ่มต้น
- [ ] พัฒนาตัวเลือก `--format` ให้รองรับการบันทึกเป็น JSON/YAML นอกเหนือจาก Markdown
- [ ] เพิ่มการรองรับ Custom Helper ใน Template Engine
