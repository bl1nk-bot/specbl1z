# gh-bl1nk-triage 📦

GitHub CLI extension สำหรับดึงรายงาน Security Alerts (Dependabot, CodeQL, Secret Scanning) ให้ออกมาเป็นรูปแบบ Markdown หรือ CSV ที่อ่านง่ายและพร้อมใช้งาน

## ✨ คุณสมบัติ
- ดึงข้อมูล Security Alerts ครบทั้ง 3 ประเภท
- รองรับการแสดงผลแบบ **Markdown Table** (สวยงามสำหรับ GitHub Issues/PRs)
- รองรับการแสดงผลแบบ **CSV** (สำหรับนำไปวิเคราะห์ต่อใน Excel/Sheets)
- จัดการปัญหาเรื่องสิทธิ์และ Line Endings อัตโนมัติ (รองรับ Linux, macOS และ Termux)

## 🚀 การติดตั้ง

### ข้อกำหนดเบื้องต้น
- ติดตั้ง [GitHub CLI (gh)](https://cli.github.com/)
- ล็อกอินด้วย `gh auth login`

### ขั้นตอนการติดตั้ง (Local)
1. ดาวน์โหลดหรือคัดลอกโฟลเดอร์นี้มาที่เครื่อง
2. รันสคริปต์เตรียมความพร้อม:
   ```bash
   chmod +x install.sh && ./install.sh
   ```
3. ติดตั้ง extension:
   ```bash
   gh extension install .
   ```

## 🛠 วิธีใช้งาน
ดึงรายงานเป็น Markdown (เริ่มต้น):
```bash
gh bl1nk-triage -r <owner>/<repo>
```

ดึงรายงานเป็น CSV:
```bash
gh bl1nk-triage -r <owner>/<repo> --csv
```

## 📄 ใบอนุญาต
MIT License
