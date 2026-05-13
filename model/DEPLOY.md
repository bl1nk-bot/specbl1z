# 🚀 Deploy OpenCode บน Modal

## ข้อกำหนดก่อน Deploy

1. **ติดตั้ง Modal CLI:**
   ```bash
   pip install modal
   ```

2. **Login Modal:**
   ```bash
   modal setup
   ```

3. **Secrets ที่ต้องมีใน Modal Dashboard:**
   - `opencode-secret` - API keys สำหรับ OpenCode
   - `github-secret` - GitHub token (ถ้าใช้)

4. **Volume:**
   - `ollama-models` - จะถูกสร้างอัตโนมัติถ้ายังไม่มี

## คำสั่ง Deploy

### 🔥 Deploy ถาวร (แนะนำสำหรับ Production)

```bash
modal deploy opencode.py
```

**ผลลัพธ์:**
- URL คงที่ (เช่น: `https://your-username--opencode-free-cloud-backend-server.modal.run`)
- App จะรันต่อเนื่องจนกว่าคุณจะลบ
- URL จะแสดงใน terminal หลัง deploy สำเร็จ

### 🛠 Serve แบบ Long-running (สำหรับ Development)

```bash
modal serve opencode.py
```

**ผลลัพธ์:**
- URL ชั่วคราว (จะเปลี่ยนเมื่อ restart)
- เหมาะสำหรับทดสอบและ develop
- กด `Ctrl+C` เพื่อหยุด

### 🐚 ทดสอบใน Shell

```bash
modal shell --image=get_image() opencode.py
```

## การใช้งาน API

หลัง deploy สำเร็จ คุณสามารถเรียกใช้ API ได้ที่:

```bash
# ตัวอย่าง: เรียกใช้งาน endpoint
curl -X POST https://YOUR-APP-URL.modal.run/v1/chat/completions \
  -H "Content-Type: application/json" \
  -d '{
    "model": "llama2",
    "messages": [{"role": "user", "content": "Hello"}]
  }'
```

## Configuration

แก้ไขค่าใน `opencode.py`:

```python
PORT = 4096              # Port ที่ backend รัน
USE_KILOCODE = False     # True = ใช้ KiloCode, False = ใช้ OpenCode
```

### GPU & Resources

```python
@app.function(
    gpu="A10G",          # เปลี่ยนเป็น: T4, A100, หรือ None (CPU only)
    cpu=6,               # จำนวน CPU cores
    memory=16384,        # RAM ใน MB (16GB)
    timeout=7200,        # Timeout ในวินาที (2 ชั่วโมง)
)
```

## Troubleshooting

### ❌ Deploy ล้มเหลว

```bash
# ดู logs
modal app logs opencode-free-cloud-backend

# ตรวจสอบ secrets
modal secret list
```

### 🐌 Server ช้าตอนเริ่ม

- `keep_warm=True` จะรักษา instance ให้พร้อมใช้งาน
- ถ้า cold start ใช้เวลานาน รอ 5-10 วินาที

### 🔌 Connection Refused

- ตรวจสอบว่า backend เริ่มทำงานเสร็จแล้ว (ดู logs)
- ตรวจสอบว่า port ตรงกัน (PORT = 4096)

## Custom Domain (Optional)

ถ้าต้องการผูก domain ของตัวเอง:

1. เพิ่ม domain ใน Modal Dashboard
2. ตั้งค่า DNS records
3. แก้ไข app ให้ใช้ custom domain

```python
# ใน modal.App()
app = modal.App("opencode-free-cloud-backend")
# เพิ่ม custom domain ผ่าน Dashboard หรือ CLI
```

## การอัปเดต

```bash
# Deploy ใหม่ทับของเดิม
modal deploy opencode.py

# หรือ deploy เป็น app ใหม่
modal deploy --name opencode-v2 opencode.py
```

## หยุด/ลบ App

```bash
# หยุด app
modal app stop opencode-free-cloud-backend

# ลบ app ถาวร
modal app delete opencode-free-cloud-backend
```

## Links

- [Modal Documentation](https://modal.com/docs)
- [OpenCode](https://opencode.ai)
- [KiloCode](https://kilocode.ai)
