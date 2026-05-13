# SECURITY_TRIAGE_CRITERIA.md

เกณฑ์ตัดสิน GitHub Security Alerts — ใช้ประกอบกับ `triage_security.py` และ Kilo API prompt

---

## หลักการทำงาน

```
Alert เข้ามา
    ↓
[TOOL] gh cli ดึง alert metadata
    ↓
[SCRIPT] triage_security.py จัดหมวด + score เบื้องต้น
    ↓
[AI Prompt] Kilo API รับ structured input → ตัดสิน action
    ↓
[TOOL] gh cli เปิด Issue / dismiss / flag
```

AI prompt รับเฉพาะ structured input ที่ script เตรียมไว้ ไม่อ่าน raw codebase

---

## กลุ่ม A: แหล่งที่มา (Alert Source)

| ID | Source | เครื่องมือดึงข้อมูล |
|----|--------|-------------------|
| A1 | Dependabot Alert (CVE) | `gh api /repos/{owner}/{repo}/dependabot/alerts` |
| A2 | Dependabot Malware | `gh api /repos/{owner}/{repo}/dependabot/alerts?ecosystem=malware` |
| A3 | CodeQL / Code Scanning | `gh api /repos/{owner}/{repo}/code-scanning/alerts` |
| A4 | Secret Scanning | `gh api /repos/{owner}/{repo}/secret-scanning/alerts` |

---

## กลุ่ม B: Reachability Checks

script ทำก่อน AI — ถ้า script ตัดได้ชัดเจน ไม่ต้องส่ง AI

### B1 — Dependabot (CVE)
```
ตรวจ: package อยู่ใน dependencies หรือ devDependencies?
- devDependencies + CVE ไม่ใช่ runtime exploit → score -2 (ลดความเร่งด่วน)
- package ถูก import ใน src/ จริงไหม → grep -r "require\|import" src/
- vulnerable function ถูกเรียกจริงไหม → ดู CVE advisory ว่า function ชื่ออะไร แล้ว grep
```

### B2 — Dependabot Malware
```
ตรวจ: package ชื่อนี้อยู่ใน lock file จริงไหม?
- ถ้าใช่ → Critical ทันที ไม่ต้องให้ AI ตัดสิน → เปิด Issue + block PR
- ถ้าไม่มีใน lock file → false positive → dismiss
```

### B3 — CodeQL
```
ตรวจ: alert location อยู่ใน file ที่ถูก compile/bundle จริงไหม?
- ถ้าอยู่ใน test/ หรือ __tests__/ → score -2
- ถ้าอยู่ใน vendor/ หรือ node_modules/ → dismiss ทันที (false positive)
- ถ้า rule_id มี "experimental" → score -1
```

### B4 — Secret Scanning
```
ตรวจ: secret อยู่ใน file ประเภทไหน?
- *.example / *.sample / *.template → dismiss ทันที
- test/ หรือ __tests__/ → score -1 + flag for review
- .env.local / .env.development → medium
- src/ หรือ committed โดยตรงใน history → Critical ทันที
```

---

## กลุ่ม C: Context ของ Codebase

script ดึงจาก repo metadata ก่อนส่ง AI

| ID | ตรวจอะไร | วิธีดึง | ผลต่อ score |
|----|---------|---------|------------|
| C1 | repo visibility | `gh repo view --json visibility` | private → -1 |
| C2 | has network exposure | มี Dockerfile / server entry point ไหม | ถ้าไม่มี → -1 |
| C3 | dep อยู่ใน prod หรือ dev | ดูจาก package.json / Cargo.toml section | dev only → -2 |
| C4 | language runtime | `gh repo view --json primaryLanguage` | ใช้ประกอบ B checks |
| C5 | repo มี CI/CD test ไหม | ดูว่ามี .github/workflows/ | ถ้ามี test cover → -1 |

---

## กลุ่ม D: Severity Matrix

### D1 — Base Severity (จาก GitHub)
| Level | CVSS | Action เริ่มต้น |
|-------|------|----------------|
| Critical | 9.0–10.0 | เปิด Issue ทันที |
| High | 7.0–8.9 | เปิด Issue |
| Medium | 4.0–6.9 | Flag for review |
| Low | 0.1–3.9 | Snooze 30 วัน |
| None | 0 | Dismiss |

### D2 — Score Adjustment (จาก B + C checks)

```
final_score = base_impact + reachability + context + trust

ถ้า final_score ≥ 80   → เปิด Issue (Critical/High)
ถ้า final_score 40–79   → Flag → AI review
ถ้า final_score 20–39   → Snooze
ถ้า final_score < 20   → Dismiss + reason
```

### D3 — False Positive Rate โดยประมาณ
| Source | FP Rate | นัยยะ |
|--------|---------|-------|
| Dependabot CVE | ~20% | ส่วนใหญ่ reachability issue |
| Dependabot Malware | ~5% | เชื่อได้สูง |
| CodeQL default | ~40% | ต้อง reachability check เสมอ |
| CodeQL experimental | ~65% | score -1 อัตโนมัติ |
| Secret Scanning | ~30% | ส่วนใหญ่ test/example file |

---

## กลุ่ม E: Action Output

### E1 — เปิด Issue
```markdown
title: [SECURITY] {alert.rule_id} in {alert.location.path}
labels: security, {severity}
body:
- Alert source + link
- CVE/rule ID
- Location ที่พบ
- Reachability verdict (จาก script)
- แนะนำว่าต้องทำอะไร (จาก AI)
- ลิงก์ไป advisory
```

### E2 — Dismiss
```
reason: "false_positive" | "used_in_tests" | "not_reachable" | "wont_fix"
comment: บอกเหตุผลที่ script/AI ตัดสิน
```

### E3 — Flag (Human Review)
```
เปิด Issue ด้วย label: security, needs-review
ไม่ใส่ severity จนกว่าคนจะ review
```

### E4 — Snooze
```
dismiss ชั่วคราว 30 วัน
เพิ่ม comment: "snoozed — low severity, no prod exposure"
```

---

## AI Prompt Input Format

script ส่งให้ AI ในรูปแบบนี้เท่านั้น — ไม่ส่ง raw code

```json
{
  "alert_source": "A1|A2|A3|A4",
  "base_severity": "critical|high|medium|low",
  "location_type": "src|test|vendor|config|example",
  "dep_scope": "prod|dev|none",
  "repo_visibility": "public|private",
  "has_network_exposure": true,
  "reachability_grep_hit": true,
  "rule_id": "...",
  "cve_id": "...",
  "advisory_summary": "...(max 200 chars)...",
  "current_score": 6
}
```

AI ตอบกลับในรูปแบบ:
```json
{
  "action": "open_issue|dismiss|flag|snooze",
  "final_score": 6,
  "reason": "...",
  "issue_recommendation": "..."
}
```

---

## สิ่งที่ Script ทำ — สิ่งที่ AI ทำ

| งาน | Script | AI |
|-----|--------|----|
| ดึง alert metadata | ✅ | ❌ |
| grep reachability | ✅ | ❌ |
| คำนวณ score | ✅ | ❌ |
| ตัดสิน Malware B2 | ✅ ทันที | ❌ |
| ตัดสิน vendor/example | ✅ ทันที | ❌ |
| วิเคราะห์ advisory context | ❌ | ✅ |
| เขียน issue body | ❌ | ✅ |
| เขียน dismiss reason | ❌ | ✅ |
| กรณีกำกวม score 4–6 | ❌ | ✅ |
