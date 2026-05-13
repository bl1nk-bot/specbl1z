# Changelog

ประวัติการเปลี่ยนแปลงของโปรเจกต์ gh-bl1nk-triage

## [0.1.1] - 2026-04-22
### Added
- เพิ่มไฟล์ `.gitattributes` เพื่อควบคุม Line Endings (LF)
- เพิ่ม `install.sh` สำหรับเตรียมความพร้อมก่อนติดตั้ง
- เพิ่ม `README.md` และ `CHANGELOG.md`

### Fixed
- แก้ไขปัญหา "Permission denied" ใน Termux โดยการปรับปรุงตำแหน่งการติดตั้งและสิทธิ์ไฟล์
- แก้ไข Arithmetic Syntax Error ในการคำนวณรวมจำนวน Security Alerts
- ปรับปรุงการจัดการค่าว่างและ Newline จาก GitHub API

## [0.1.0] - 2026-04-22
### Added
- เริ่มต้นโปรเจกต์ (Initial Release)
- ฟีเจอร์หลัก: ดึง Security Alerts เป็น Markdown และ CSV
