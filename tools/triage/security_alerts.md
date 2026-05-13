# Security Alerts Report
> **Repo:** bl1nk-bot/agent-library
> **วันที่:** 2026-04-22 15:22
> **รวมทั้งหมด:** 110 alerts (Dependabot: 72 | CodeQL: 8 | Secret: 30)

---

## 📦 Dependabot Alerts (72)

> แจ้งเมื่อ **library/package** ที่ใช้อยู่มีช่องโหว่ด้านความปลอดภัย

| # | แจ้งเรื่อง | Package | Version | Severity | Scope | สรุปปัญหา |
|---|-----------|---------|---------|----------|-------|-----------|
| 102 | 🟡 MEDIUM | `hono` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | hono Improperly Handles JSX Attribute Names Allows HTML Injection in hono/jsx SSR (CVE: N/A) |
| 101 | 🟡 MEDIUM | `dompurify` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | DOMPurify's ADD_TAGS function form bypasses FORBID_TAGS due to short-circuit evaluation (CVE: N/A) |
| 99 | 🟡 MEDIUM | `next-intl` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | next-intl has an open redirect vulnerability (CVE: CVE-2026-40299) |
| 98 | 🟠 HIGH | `next` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | Next.js has a Denial of Service with Server Components (CVE: N/A) |
| 97 | 🟡 MEDIUM | `fast-xml-parser` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Entity Expansion Limits Bypassed When Set to Zero Due to JavaScript Falsy Evaluation in fast-xml-parser (CVE: CVE-2026-33349) |
| 96 | 🟡 MEDIUM | `hono` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Hono: Non-breaking space prefix bypass in cookie name handling in getCookie() (CVE: CVE-2026-39410) |
| 95 | 🟡 MEDIUM | `hono` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Hono has incorrect IP matching in ipRestriction() for IPv4-mapped IPv6 addresses (CVE: CVE-2026-39409) |
| 94 | 🟡 MEDIUM | `hono` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Hono missing validation of cookie name on write path in setCookie() (CVE: N/A) |
| 93 | 🟡 MEDIUM | `hono` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Hono: Path traversal in toSSG() allows writing files outside the output directory (CVE: CVE-2026-39408) |
| 92 | 🟡 MEDIUM | `hono` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Hono: Middleware bypass via repeated slashes in serveStatic (CVE: CVE-2026-39407) |
| 91 | 🟡 MEDIUM | `@hono/node-server` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | @hono/node-server: Middleware bypass via repeated slashes in serveStatic (CVE: CVE-2026-39406) |
| 89 | 🟠 HIGH | `defu` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | defu: Prototype pollution via `__proto__` key in defaults argument (CVE: CVE-2026-35209) |
| 88 | 🟡 MEDIUM | `dompurify` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | DOMPurify ADD_ATTR predicate skips URI validation (CVE: N/A) |
| 87 | 🟡 MEDIUM | `dompurify` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | DOMPurify USE_PROFILES prototype pollution allows event handlers (CVE: N/A) |
| 86 | 🟡 MEDIUM | `path-to-regexp` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | path-to-regexp vulnerable to Regular Expression Denial of Service via multiple wildcards (CVE: CVE-2026-4923) |
| 85 | 🟠 HIGH | `path-to-regexp` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | path-to-regexp vulnerable to Denial of Service via sequential optional groups (CVE: CVE-2026-4926) |
| 84 | 🟡 MEDIUM | `dompurify` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | DOMPurify is vulnerable to mutation-XSS via Re-Contextualization  (CVE: N/A) |
| 83 | 🟡 MEDIUM | `serialize-javascript` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Serialize JavaScript has CPU Exhaustion Denial of Service via crafted array-like objects (CVE: CVE-2026-34043) |
| 81 | 🟡 MEDIUM | `brace-expansion` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | brace-expansion: Zero-step sequence causes process hang and memory exhaustion (CVE: CVE-2026-33750) |
| 80 | 🟡 MEDIUM | `picomatch` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Picomatch: Method Injection in POSIX Character Classes causes incorrect Glob Matching (CVE: CVE-2026-33672) |
| 79 | 🟡 MEDIUM | `picomatch` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Picomatch: Method Injection in POSIX Character Classes causes incorrect Glob Matching (CVE: CVE-2026-33672) |
| 78 | 🟠 HIGH | `picomatch` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | Picomatch has a ReDoS vulnerability via extglob quantifiers (CVE: CVE-2026-33671) |
| 77 | 🟠 HIGH | `picomatch` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | Picomatch has a ReDoS vulnerability via extglob quantifiers (CVE: CVE-2026-33671) |
| 76 | 🟡 MEDIUM | `yaml` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | yaml is vulnerable to Stack Overflow via deeply nested YAML collections (CVE: CVE-2026-33532) |
| 75 | 🟠 HIGH | `fast-xml-parser` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | fast-xml-parser affected by numeric entity expansion bypassing all entity expansion limits (incomplete fix for CVE-2026-26278) (CVE: CVE-2026-33036) |
| 74 | 🟠 HIGH | `effect` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | Effect `AsyncLocalStorage` context lost/contaminated inside Effect fibers under concurrent load with RPC (CVE: CVE-2026-32887) |
| 73 | 🟡 MEDIUM | `next` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Next.js: Unbounded next/image disk cache growth can exhaust storage (CVE: CVE-2026-27980) |
| 72 | 🟠 HIGH | `flatted` | `pnpm-lock.yaml` | 🟠 HIGH | 🔧 dev-only | Prototype Pollution via parse() in NodeJS flatted (CVE: CVE-2026-33228) |
| 71 | 🟡 MEDIUM | `next` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Next.js: HTTP request smuggling in rewrites (CVE: CVE-2026-29057) |
| 70 | 🟡 MEDIUM | `next` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Next.js: Unbounded postponed resume buffering can lead to DoS (CVE: CVE-2026-27979) |
| 69 | 🟡 MEDIUM | `next` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Next.js: null origin can bypass Server Actions CSRF checks (CVE: CVE-2026-27978) |
| 66 | 🟡 MEDIUM | `hono` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Hono vulnerable to Prototype Pollution possible through __proto__ key allowed in parseBody({ dot: true }) (CVE: N/A) |
| 65 | 🟠 HIGH | `express-rate-limit` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | express-rate-limit: IPv4-mapped IPv6 addresses bypass per-client rate limiting on servers with dual-stack network (CVE: CVE-2026-30827) |
| 64 | 🟡 MEDIUM | `dompurify` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | DOMPurify contains a Cross-site Scripting vulnerability (CVE: CVE-2026-0540) |
| 63 | 🟠 HIGH | `@hono/node-server` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | @hono/node-server has authorization bypass for protected static paths via encoded slashes in Serve Static Middleware (CVE: CVE-2026-29087) |
| 62 | 🟡 MEDIUM | `hono` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Hono Vulnerable to Cookie Attribute Injection via Unsanitized domain and path in setCookie() (CVE: CVE-2026-29086) |
| 61 | 🟡 MEDIUM | `hono` | `pnpm-lock.yaml` | 🟡 MEDIUM | 🚀 prod | Hono Vulnerable to SSE Control Field Injection via CR/LF in writeSSE() (CVE: CVE-2026-29085) |
| 60 | 🟠 HIGH | `hono` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | Hono vulnerable to arbitrary file access via serveStatic vulnerability  (CVE: CVE-2026-29045) |
| 58 | 🟠 HIGH | `serialize-javascript` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | Serialize JavaScript is Vulnerable to RCE via RegExp.flags and Date.prototype.toISOString() (CVE: N/A) |
| 57 | 🟠 HIGH | `minimatch` | `pnpm-lock.yaml` | 🟠 HIGH | 🔧 dev-only | minimatch has ReDoS: matchOne() combinatorial backtracking via multiple non-adjacent GLOBSTAR segments (CVE: CVE-2026-27903) |
| 56 | 🟠 HIGH | `minimatch` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | minimatch has ReDoS: matchOne() combinatorial backtracking via multiple non-adjacent GLOBSTAR segments (CVE: CVE-2026-27903) |
| 54 | 🟠 HIGH | `minimatch` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | minimatch ReDoS: nested *() extglobs generate catastrophically backtracking regular expressions (CVE: CVE-2026-27904) |
| 53 | 🟠 HIGH | `rollup` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | Rollup 4 has Arbitrary File Write via Path Traversal (CVE: CVE-2026-27606) |
| 52 | 🟠 HIGH | `hono` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | Hono is Vulnerable to Authentication Bypass by IP Spoofing in AWS Lambda ALB conninfo (CVE: CVE-2026-27700) |
| 50 | 🟠 HIGH | `minimatch` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | minimatch has a ReDoS via repeated wildcards with non-matching literal in pattern (CVE: CVE-2026-26996) |
| 47 | 🔴 CRITICAL | `form-data` | `pnpm-lock.yaml` | 🔴 CRITICAL | 🔧 dev-only | form-data uses unsafe random function in form-data for choosing boundary (CVE: CVE-2025-7783) |
| 43 | 🔴 CRITICAL | `minimist` | `pnpm-lock.yaml` | 🔴 CRITICAL | 🔧 dev-only | Prototype Pollution in minimist (CVE: CVE-2021-44906) |
| 41 | 🟠 HIGH | `trim-newlines` | `pnpm-lock.yaml` | 🟠 HIGH | 🚀 prod | Uncontrolled Resource Consumption in trim-newlines (CVE: CVE-2021-33623) |
| 37 | 🟡 MEDIUM | `hono` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | hono Improperly Handles JSX Attribute Names Allows HTML Injection in hono/jsx SSR (CVE: N/A) |
| 36 | 🟡 MEDIUM | `dompurify` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | DOMPurify's ADD_TAGS function form bypasses FORBID_TAGS due to short-circuit evaluation (CVE: N/A) |
| 34 | 🟡 MEDIUM | `next-intl` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | next-intl has an open redirect vulnerability (CVE: CVE-2026-40299) |
| 33 | 🟠 HIGH | `next` | `package-lock.json` | 🟠 HIGH | 🚀 prod | Next.js has a Denial of Service with Server Components (CVE: N/A) |
| 32 | 🟡 MEDIUM | `hono` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | Hono: Non-breaking space prefix bypass in cookie name handling in getCookie() (CVE: CVE-2026-39410) |
| 31 | 🟡 MEDIUM | `hono` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | Hono has incorrect IP matching in ipRestriction() for IPv4-mapped IPv6 addresses (CVE: CVE-2026-39409) |
| 30 | 🟡 MEDIUM | `hono` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | Hono missing validation of cookie name on write path in setCookie() (CVE: N/A) |
| 29 | 🟡 MEDIUM | `hono` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | Hono: Path traversal in toSSG() allows writing files outside the output directory (CVE: CVE-2026-39408) |
| 28 | 🟡 MEDIUM | `hono` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | Hono: Middleware bypass via repeated slashes in serveStatic (CVE: CVE-2026-39407) |
| 27 | 🟡 MEDIUM | `@hono/node-server` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | @hono/node-server: Middleware bypass via repeated slashes in serveStatic (CVE: CVE-2026-39406) |
| 25 | 🟠 HIGH | `defu` | `package-lock.json` | 🟠 HIGH | 🔧 dev-only | defu: Prototype pollution via `__proto__` key in defaults argument (CVE: CVE-2026-35209) |
| 24 | 🟡 MEDIUM | `dompurify` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | DOMPurify ADD_ATTR predicate skips URI validation (CVE: N/A) |
| 23 | 🟡 MEDIUM | `dompurify` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | DOMPurify USE_PROFILES prototype pollution allows event handlers (CVE: N/A) |
| 22 | 🟡 MEDIUM | `path-to-regexp` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | path-to-regexp vulnerable to Regular Expression Denial of Service via multiple wildcards (CVE: CVE-2026-4923) |
| 21 | 🟠 HIGH | `path-to-regexp` | `package-lock.json` | 🟠 HIGH | 🚀 prod | path-to-regexp vulnerable to Denial of Service via sequential optional groups (CVE: CVE-2026-4926) |
| 20 | 🟡 MEDIUM | `dompurify` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | DOMPurify is vulnerable to mutation-XSS via Re-Contextualization  (CVE: N/A) |
| 17 | 🟡 MEDIUM | `brace-expansion` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | brace-expansion: Zero-step sequence causes process hang and memory exhaustion (CVE: CVE-2026-33750) |
| 15 | 🟡 MEDIUM | `picomatch` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | Picomatch: Method Injection in POSIX Character Classes causes incorrect Glob Matching (CVE: CVE-2026-33672) |
| 13 | 🟠 HIGH | `picomatch` | `package-lock.json` | 🟠 HIGH | 🚀 prod | Picomatch has a ReDoS vulnerability via extglob quantifiers (CVE: CVE-2026-33671) |
| 12 | 🟠 HIGH | `effect` | `package-lock.json` | 🟠 HIGH | 🔧 dev-only | Effect `AsyncLocalStorage` context lost/contaminated inside Effect fibers under concurrent load with RPC (CVE: CVE-2026-32887) |
| 11 | 🟡 MEDIUM | `dompurify` | `package-lock.json` | 🟡 MEDIUM | 🚀 prod | DOMPurify contains a Cross-site Scripting vulnerability (CVE: CVE-2026-0540) |
| 9 | 🔴 CRITICAL | `form-data` | `package-lock.json` | 🔴 CRITICAL | 🔧 dev-only | form-data uses unsafe random function in form-data for choosing boundary (CVE: CVE-2025-7783) |
| 6 | 🔴 CRITICAL | `minimist` | `package-lock.json` | 🔴 CRITICAL | 🔧 dev-only | Prototype Pollution in minimist (CVE: CVE-2021-44906) |
| 4 | 🟠 HIGH | `trim-newlines` | `package-lock.json` | 🟠 HIGH | 🚀 prod | Uncontrolled Resource Consumption in trim-newlines (CVE: CVE-2021-33623) |

---

## 🔍 CodeQL / Code Scanning Alerts (8)

> แจ้งเมื่อพบ **รูปแบบโค้ดที่อาจเป็นช่องโหว่** เช่น injection, path traversal

| # | Rule | แจ้งเรื่อง | Severity | ไฟล์ | บรรทัด |
|---|------|-----------|----------|------|--------|
| 8 | `js/identity-replacement` | Replacement of a substring with itself | ⚪ UNKNOWN | `src/lib/plugins/media-generators/fal.ts` | 304 |
| 7 | `js/incomplete-sanitization` | Incomplete string escaping or encoding | ⚪ UNKNOWN | `src/lib/webhook.ts` | 256 |
| 6 | `js/incomplete-sanitization` | Incomplete string escaping or encoding | ⚪ UNKNOWN | `src/components/ide/utils.ts` | 12 |
| 5 | `js/incomplete-sanitization` | Incomplete string escaping or encoding | ⚪ UNKNOWN | `src/components/api/improve-prompt-demo.tsx` | 216 |
| 4 | `js/incomplete-sanitization` | Incomplete string escaping or encoding | ⚪ UNKNOWN | `src/__tests__/lib/webhook.test.ts` | 55 |
| 3 | `js/incomplete-multi-character-sanitization` | Incomplete multi-character sanitization | ⚪ UNKNOWN | `src/lib/similarity.ts` | 13 |
| 2 | `actions/missing-workflow-permissions` | Workflow does not contain permissions | ⚪ UNKNOWN | `.github/workflows/security.yml` | 11 |
| 1 | `actions/missing-workflow-permissions` | Workflow does not contain permissions | ⚪ UNKNOWN | `.github/workflows/project-automation.yml` | 11 |

---

## 🔑 Secret Scanning Alerts (30)

> แจ้งเมื่อพบ **API key, token, หรือ credential** ที่ถูก commit ขึ้น repo

| # | ประเภท Secret | ไฟล์ที่พบ | Commit |
|---|--------------|----------|--------|

---

## สรุป Pattern

| แหล่ง | จำนวน | ความหมาย |
|-------|-------|----------|
| 📦 Dependabot | 72 | library มีช่องโหว่ |
| 🔍 CodeQL | 8 | โค้ดมีรูปแบบอันตราย |
| 🔑 Secret | 30 | credential หลุดใน repo |
