#!/usr/bin/env python3

import os
import json
import subprocess
import re
import sys
from pathlib import Path
import numpy as np

# --- Configuration ---
HOME = str(Path.home())
OLLAMA_URL = "http://localhost:11434/api/embeddings"
EMBEDDING_MODEL = "qwen3-embedding:0.6b" 
SIMILARITY_THRESHOLD = 0.92  # 0.0 to 1.0

def find_skill_files():
    """Find all SKILL.md files in Termux."""
    print("🔍 Searching for SKILL.md files across Termux...")
    cmd = ["find", HOME, "-name", "SKILL.md", "-o", "-name", "skill.md"]
    result = subprocess.run(cmd, capture_output=True, text=True)
    paths = result.stdout.splitlines()
    filtered = []
    for p in paths:
        if any(x in p for x in ["node_modules", "target", ".git/", ".pytest_cache", "Downloads/tmp"]):
            continue
        filtered.append(p)
    return filtered

def extract_skill_info(path):
    """Extract name and description from SKILL.md."""
    try:
        with open(path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # Regex for YAML name: field or # Title
        name_match = re.search(r"^name:\s*(.*)$", content, re.M | re.I)
        if not name_match:
            name_match = re.search(r"^#\s*(.*)$", content, re.M)
            
        desc_match = re.search(r"^description:\s*(.*)$", content, re.M | re.I)
        if not desc_match:
            # Fallback: find first paragraph after title
            lines = content.splitlines()
            for i, line in enumerate(lines):
                if line.startswith("#"):
                    for j in range(i+1, len(lines)):
                        clean_line = lines[j].strip()
                        if clean_line and not clean_line.startswith("<"):
                            description = clean_line
                            break
                    break
        else:
            description = desc_match.group(1).strip()
        
        name = name_match.group(1).strip() if name_match else Path(path).parent.name
        
        if not desc_match and 'description' not in locals():
            description = "No description found"

        return {
            "name": name,
            "description": description,
            "path": path
        }
    except Exception as e:
        return None

def get_embedding(text):
    """Get embedding from Ollama."""
    import requests
    try:
        resp = requests.post(OLLAMA_URL, json={
            "model": EMBEDDING_MODEL,
            "prompt": text
        }, timeout=10)
        if resp.status_code == 200:
            return resp.json().get("embedding")
    except Exception as e:
        return None

def cosine_similarity(v1, v2):
    v1 = np.array(v1)
    v2 = np.array(v2)
    return np.dot(v1, v2) / (np.linalg.norm(v1) * np.linalg.norm(v2))

def main():
    files = find_skill_files()
    print(f"Found {len(files)} potential skill files.")
    
    raw_skills = []
    for f in files:
        info = extract_skill_info(f)
        if info and len(info['description']) > 10: # Ignore stub skills
            raw_skills.append(info)
            
    print(f"Processing {len(raw_skills)} valid skills...")

    # Step 1: Get Embeddings
    embedded_skills = []
    for s in raw_skills:
        print(f"  → Vectorizing: {s['name']}...", end="\r")
        emb = get_embedding(s['description'])
        if emb:
            s['embedding'] = emb
            embedded_skills.append(s)
        else:
            # Fallback to name-only deduplication if Ollama is down
            pass

    if not embedded_skills:
        print("\n⚠️ Ollama not reachable or no embeddings generated. Falling back to basic deduplication.")
        # Simple name-based fallback
        final_skills = {}
        duplicates_report = []
        for s in raw_skills:
            key = s['name'].lower()
            if key not in final_skills:
                final_skills[key] = s
            else:
                duplicates_report.append({
                    "name": s['name'],
                    "kept_path": final_skills[key]['path'],
                    "duplicate_path": s['path'],
                    "reason": "Name collision"
                })
        unique_list = list(final_skills.values())
    else:
        print(f"\n🧠 Performing semantic deduplication on {len(embedded_skills)} skills...")
        unique_list = []
        duplicates_report = []
        for s in embedded_skills:
            is_duplicate = False
            for existing in unique_list:
                sim = cosine_similarity(s['embedding'], existing['embedding'])
                if sim > SIMILARITY_THRESHOLD:
                    is_duplicate = True
                    duplicates_report.append({
                        "name": s['name'],
                        "kept_path": existing['path'],
                        "duplicate_path": s['path'],
                        "similarity": float(sim)
                    })
                    # Keep the one with the longer description
                    if len(s['description']) > len(existing['description']):
                        # Swap paths in report if we update the kept one
                        duplicates_report[-1]["kept_path"] = s['path']
                        duplicates_report[-1]["duplicate_path"] = existing['path']
                        existing.update(s)
                    break
            if not is_duplicate:
                unique_list.append(s)

    print(f"✅ Filtered down to {len(unique_list)} unique skills.")
    
    # Generate Cleanup Report
    report_file = "duplicates_report.json"
    with open(report_file, 'w') as f:
        json.dump(duplicates_report, f, indent=2)
    print(f"📊 Cleanup report generated: {report_file}")
    print(f"   (Found {len(duplicates_report)} potential duplicates for you to review)")
    
    # Generate Import Commands
    import_script = "import_skills.sh"
    with open(import_script, 'w') as f:
        f.write("#!/bin/bash\n\n")
        f.write("# Specgen Skill Import Script\n")
        f.write("BIN=\"./target/debug/specgen\"\n\n")
        for s in unique_list:
            clean_desc = s['description'].replace('"', '\\"').replace('$', '\\$')
            f.write(f'$BIN memory write --scope global --category preference --key "skill:{s["name"]}" --value "{clean_desc}"\n')
            
    os.chmod(import_script, 0o755)
    print(f"🚀 Generated: {import_script}")
    print("Run './import_skills.sh' to seed your database.")

if __name__ == "__main__":
    main()
