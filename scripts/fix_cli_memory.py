#!/data/data/com.termux/files/usr/bin/python3
# Fix CLI memory store initialization

# Read main.rs
with open('/data/data/com.termux/files/home/specgen/cli/src/main.rs', 'r') as f:
    content = f.read()

# Fix line 1111 - change `&database` to proper Database initialization
content = content.replace(
    'let store = specgen_core::memory::MemoryStore::new(&database)?;',
    '''let db = specgen_core::db::Database::new(&database)?;
                let store = specgen_core::memory::MemoryStore::new(&db);'''
)

# Fix line 1152 - same fix
content = content.replace(
    'let store = specgen_core::memory::MemoryStore::new(&database)?;',
    '''let db = specgen_core::db::Database::new(&database)?;
                let store = specgen_core::memory::MemoryStore::new(&db);'''
)

# Write back
with open('/data/data/com.termux/files/home/specgen/cli/src/main.rs', 'w') as f:
    f.write(content)

print("Fixed CLI main.rs: MemoryStore initialization")
