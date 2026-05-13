# craft-local-db

ฐานข้อมูล SQLite ที่มีโครงสร้างเหมือน Craft สำหรับใช้ในท้องถิ่น  
รองรับการนำเข้าไฟล์ Markdown (รวม frontmatter)

## การใช้งาน

### 1. สร้างฐานข้อมูลเปล่า
```bash
cargo run -- init
```

### 2. นำเข้า Markdown
```bash
cargo run -- import path/to/file.md
```

## โครงสร้างฐานข้อมูล
- `folders` — โฟลเดอร์
- `documents` — เอกสาร
- `blocks` — เนื้อหาแบบ tree (หัวข้อ, ข้อความ, โค้ด, quote, list)
- `tasks` — งาน
- `collections` + `collection_properties` + `collection_items` — ฐานข้อมูลภายในเอกสาร
- `comments` — ความคิดเห็น

## Schema
ดู `schema.sql` สำหรับรายละเอียดทั้งหมด
