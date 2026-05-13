import sqlite3
import re
import sys
import time

def bulk_import():
    try:
        with open('import_skills.sh', 'r', encoding='utf-8') as f:
            lines = f.readlines()
    except FileNotFoundError:
        print("Error: import_skills.sh not found.")
        return

    # Connect to db
    conn = sqlite3.connect('craft.db')
    cursor = conn.cursor()
    
    # Regex to extract key and value
    # Pattern: --key "..." --value "..."
    pattern = re.compile(r'--key\s+"(skill:[^"]+)"\s+--value\s+"(.*)"$', re.DOTALL)
    
    entries = []
    current_time = int(time.time())
    
    for line in lines:
        if line.startswith('$BIN memory write'):
            # The value might contain escaped quotes, but simple regex handles the outermost quotes
            match = re.search(r'--key\s+"(skill:[^"]+)"\s+--value\s+"(.*)"\s*$', line.strip())
            if match:
                key = match.group(1)
                value = match.group(2).replace('\\"', '"') # unescape quotes
                entries.append((
                    'global', 'preference', key, value, 'seeding_script', 1.0, current_time, current_time, 1
                ))
    
    print(f"Parsed {len(entries)} skills for bulk insert.")
    
    try:
        cursor.executemany("""
            INSERT OR REPLACE INTO memory_entries 
            (scope, category, key, value, source, confidence, created_at, updated_at, version) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        """, entries)
        conn.commit()
        print(f"✅ Successfully bulk-inserted {cursor.rowcount} memory entries!")
    except Exception as e:
        print(f"Database error: {e}")
    finally:
        conn.close()

if __name__ == '__main__':
    bulk_import()
