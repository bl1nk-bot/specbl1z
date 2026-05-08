# Specgen

เครื่องมือสำหรับสร้างและตรวจสอบ Workflow Template (รองรับ JSON, Markdown+XML และ TOML)

## โครงสร้างโปรเจกต์

- `cli/`: ตัวโปรแกรมหลักสำหรับใช้งานผ่าน Command Line
- `core/`: ไลบรารีหลักสำหรับการ Parsing, Validation และ Rendering
- `schema/`: เก็บ JSON Schema สำหรับตรวจสอบความถูกต้องของเทมเพลต
- `templates/`: โฟลเดอร์เก็บเทมเพลตตัวอย่าง
- `tests/`: Integration Tests สำหรับตรวจสอบการทำงานร่วมกัน

## การใช้งาน (Usage)

หลังจากติดตั้งแล้ว สามารถใช้คำสั่ง `specgen` ได้โดยตรง:

### 1. ดูรายการเทมเพลต
```bash
specgen list-templates
```

### 2. สร้างเอกสารจากเทมเพลต (Generate)
```bash
specgen generate <TEMPLATE_ID> --var "Key=Value" --out output.md
```

### 3. ตรวจสอบความถูกต้องของเทมเพลต (Validate)
```bash
specgen validate templates/spec-workflow.md
```

## การติดตั้งสำหรับผู้ใช้ใหม่ (Installation)

```bash
# ติดตั้งผ่าน Cargo
cargo install --path cli
```

## การพัฒนา (Development)

- **รันการทดสอบ:** `cargo test`
- **ตรวจสอบโค้ด:** `cargo check`
- **จัดรูปแบบโค้ด:** `cargo fmt`
