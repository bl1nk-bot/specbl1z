#!/usr/bin/env python3
"""
triage_security.py — GitHub Security Alert Triage (Slop-Aware v2.1)
Integrated with AI Slop Protocol and 100-point Weighting Standard.
"""

import json
import subprocess
import sys
import re
from pathlib import Path
from typing import Optional

# ─── Constants & Weights (Total 100) ──────────────────────────────────────────

BASE_IMPACT_WEIGHT = {"critical": 30, "high": 20, "medium": 10, "low": 5, "none": 0}

ALWAYS_DISMISS_PATHS = [r"vendor/", r"node_modules/", r"\.min\.js$", r"dist/", r"build/", r"target/"]
ALWAYS_DISMISS_FILENAMES = [r"\.example$", r"\.sample$", r"\.template$", r"\.fixture$"]
TEST_PATHS = [r"test/", r"tests/", r"__tests__/", r"spec/", r"\.test\.", r"\.spec\."]

# 10 Slop Patterns based on Slop Protocol
SLOP_PATTERNS = {
    "obvious_comments": r"(#|//)\s*(Create|Update|Delete|Get|Set)\s+\w+",
    "excessive_abstraction": r"(Interface|Factory|Strategy|Manager|Wrapper).*(Interface|Factory|Strategy|Manager|Wrapper)",
    "generic_naming": r"\b(data|item|result|obj|val|info|temp)\b",
    "jsdoc_filler": r"\*.*@param.*@returns",
    "unnecessary_guards": r"if\s*\(\w+\s*&&\s*\w+\.length\s*>\s*0\)",
    "todo_placeholder": r"TODO:.*(implement|fix|add)",
    "redundant_type": r":\s*(number|string|boolean)\s*=",
}

# ─── GitHub CLI Helpers ────────────────────────────────────────────────────────

def fetch_api(path: str) -> list:
    cmd = ["gh", "api", f"{path}?per_page=100&state=open"]
    res = subprocess.run(cmd, capture_output=True, text=True)
    return json.loads(res.stdout) if res.returncode == 0 else []

def fetch_repo_meta(owner: str, repo: str) -> dict:
    cmd = ["gh", "repo", "view", f"{owner}/{repo}", "--json", "visibility,primaryLanguage"]
    res = subprocess.run(cmd, capture_output=True, text=True)
    return json.loads(res.stdout) if res.returncode == 0 else {"visibility": "PRIVATE"}

# ─── Core Analysis Logic ──────────────────────────────────────────────────────

def check_reachability(pkg_name: str) -> bool:
    """Grep หาการเรียกใช้ package ในโปรเจกต์"""
    if not pkg_name: return False
    search_dir = "src" if Path("src").exists() else "."
    res = subprocess.run(["grep", "-r", "-l", pkg_name, search_dir], capture_output=True, text=True)
    return bool(res.stdout.strip())

def analyze_slop(path: str, summary: str = "") -> tuple[int, list[str]]:
    """วิเคราะห์ Slop Score (max 40) ตามหลักการ Slop Protocol"""
    score = 0
    findings = []
    
    # Check File Content (ถ้ามี)
    if Path(path).exists():
        content = Path(path).read_text(errors='ignore')
        for name, pattern in SLOP_PATTERNS.items():
            if len(re.findall(pattern, content, re.IGNORECASE)) > 2:
                score += 10
                findings.append(f"Slop Detected: {name} (+10)")
    
    # Check Metadata Slop
    meta_patterns = {"experimental": 10, "example": 15, "sample": 15, "test": 5}
    for word, weight in meta_patterns.items():
        if word in summary.lower() or word in path.lower():
            score += weight
            findings.append(f"Metadata Slop: {word} (+{weight})")
            
    return min(40, score), findings

# ─── Scoring Engine ───────────────────────────────────────────────────────────

def score_alert(a_type: str, alert: dict, repo_meta: dict) -> dict:
    score = 0
    breakdown = []
    
    # 1. Base Impact (30)
    severity = alert.get("security_advisory", {}).get("severity") or \
               alert.get("rule", {}).get("severity") or "medium"
    base = BASE_IMPACT_WEIGHT.get(severity.lower(), 10)
    score += base
    breakdown.append(f"Base Impact ({severity}): +{base}")

    # 2. Reachability & Slop (40)
    path = alert.get("most_recent_instance", {}).get("location", {}).get("path") or \
           alert.get("locations", [{}])[0].get("details", {}).get("path") or "unknown"
    summary = alert.get("security_advisory", {}).get("summary") or \
              alert.get("rule", {}).get("description") or ""
    pkg = alert.get("dependency", {}).get("package", {}).get("name") or ""
    
    slop_val, slop_findings = analyze_slop(path, summary)
    breakdown.extend(slop_findings)

    # 2.1 Reachability Check (Grep)
    is_reachable = check_reachability(pkg)
    if is_reachable:
        score += 40
        breakdown.append("Directly Reachable in src/: +40")
    else:
        # ถ้าไม่เจอการเรียกใช้ แต่เป็นไฟล์ใน src ก็ยังให้คะแนนพื้นฐาน
        if "src/" in path:
            score += 20
            breakdown.append("In src/ but no direct import found: +20")
        else:
            score -= slop_val # ถ้าเป็นที่อื่น (test/example) ให้หักล้างด้วยคะแนน Slop
            breakdown.append(f"Non-critical path slop deduction: -{slop_val}")

    # 3. Context (20)
    vis = repo_meta.get("visibility", "PRIVATE")
    vis_score = 20 if vis == "PUBLIC" else 10
    score += vis_score
    breakdown.append(f"Context ({vis}): +{vis_score}")

    # 4. Trust (10)
    if "malware" in str(alert).lower():
        score += 10
        breakdown.append("Trust: Malware classification detected (+10)")

    # Final Adjustment & Action
    score = max(0, min(100, score))
    action = "open_issue" if score >= 80 else "send_to_ai" if score >= 35 else "dismiss"

    return {
        "score": score,
        "action": action,
        "breakdown": breakdown,
        "path": path,
        "type": a_type
    }

# ─── Main Logic ───────────────────────────────────────────────────────────────

def triage(owner: str, repo: str, output_file: str = "triage_output.json"):
    print(f"🔍 Triaging {owner}/{repo}...")
    repo_meta = fetch_repo_meta(owner, repo)
    results = {"critical": [], "needs_ai": [], "slop_dismissed": []}

    endpoints = [
        ("dependabot/alerts", "Dependabot"),
        ("code-scanning/alerts", "CodeQL"),
        ("secret-scanning/alerts", "Secret")
    ]

    for endpoint, a_type in endpoints:
        alerts = fetch_api(f"/repos/{owner}/{repo}/{endpoint}")
        print(f"  → Found {len(alerts)} {a_type} alerts")
        
        for a in alerts:
            path = a.get("most_recent_instance", {}).get("location", {}).get("path") or \
                   a.get("locations", [{}])[0].get("details", {}).get("path") or ""
            
            # Hard Ignore for non-source paths
            if any(re.search(p, path) for p in ALWAYS_DISMISS_PATHS): continue
            
            scored = score_alert(a_type, a, repo_meta)
            item = {
                "number": a.get("number"),
                "type": a_type,
                "score": scored["score"],
                "path": scored["path"],
                "reason": scored["breakdown"],
                "url": a.get("html_url")
            }
            
            if scored["action"] == "open_issue": results["critical"].append(item)
            elif scored["action"] == "dismiss": results["slop_dismissed"].append(item)
            else: results["needs_ai"].append(item)

    Path(output_file).write_text(json.dumps(results, indent=2, ensure_ascii=False))
    print(f"\n✅ Triage Complete! (Output: {output_file})")
    print(f"🔥 Critical: {len(results['critical'])} | 🤖 AI: {len(results['needs_ai'])} | 🧹 Slop: {len(results['slop_dismissed'])}")

if __name__ == "__main__":
    if len(sys.argv) < 3:
        print("Usage: python triage_security.py <owner> <repo>")
        sys.exit(1)
    triage(sys.argv[1], sys.argv[2])
