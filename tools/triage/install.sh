#!/usr/bin/env bash
# สคริปต์สำหรับเตรียมความพร้อมก่อนติดตั้ง extension

echo "🔧 การเตรียมความพร้อมสำหรับ gh-bl1nk-triage..."

# แก้ไข Line Endings (ลบ \r ออก)
sed -i 's/\r$//' gh-bl1nk-triage 2>/dev/null

# เพิ่มสิทธิ์การรัน
chmod +x gh-bl1nk-triage

echo "✅ พร้อมสำหรับการติดตั้งแล้ว!"
echo "รันคำสั่งนี้เพื่อติดตั้ง: gh extension install ."
