# 📝 Roadmap: Security Triage & Anti-Slop

## [x] Milestone 0: Core Foundation
- [x] Collector: gh-bl1nk-triage (v0.1.1)
- [x] Standard: SECURITY_TRIAGE_CRITERIA.md (100-pt Scale)

## [ ] Phase 1: API-based Triage (Active) 🚀
*เน้นความง่าย รวดเร็ว และทำงานผ่านระบบ Metadata*
- [x] สร้างโครงสร้างสคริปต์ประมวลผลเบื้องต้น
- [ ] ปรับปรุงการวิเคราะห์ Slop จาก API Metadata (Alert titles, summaries, labels)
- [ ] เชื่อมต่อ Kilo API สำหรับการตัดสินใจก้ำกึ่ง (Score 30-79) โดยส่งเฉพาะ Metadata

## [ ] Phase 2: Local Code Analysis
*เน้นความแม่นยำ (Precision) โดยการสแกนโค้ดจริง*
- [ ] Implement Reachability Check (Grep/Import Analysis)
- [ ] Implement Slop Protocol 10 ข้อ โดยการอ่านไฟล์ Source Code
- [ ] จัดเก็บ "Pattern Memory" เพื่อลดการวิเคราะห์ซ้ำ

## [ ] Phase 3: Workflow & Sandbox Integration
*เน้นความปลอดภัยและการทำงานอัตโนมัติในระดับ CI/CD*
- [ ] สร้าง GitHub Action Workflow สำหรับรัน Triage อัตโนมัติใน PR
- [ ] ทดสอบรัน Exploit/PoC ใน Sandbox เพื่อยืนยันช่องโหว่ (ถ้าจำเป็น)
- [ ] ระบบ Auto-dismiss/Auto-open Issue สมบูรณ์แบบ
