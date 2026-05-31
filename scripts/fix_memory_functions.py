#!/data/data/com.termux/files/usr/bin/python3
# Fix all Memory-related function calls

# Read memory.rs
with open('/data/data/com.termux/files/home/specgen/core/src/memory.rs', 'r') as f:
    content = f.read()

# Fix 1: Use string_to_scope instead of string_to_memory_scope (for the i32 result)
# line 123 expects i32 - string_to_scope returns MemoryScope, which needs .into()
content = content.replace(
    'scope: string_to_memory_scope(&row.get::<_, String>(1)?).unwrap_or(MemoryScope::ScopeGlobal) as i32,',
    'scope: string_to_scope(&row.get::<_, String>(1)?).unwrap_or(MemoryScope::ScopeGlobal).into(),'
)

# Fix 2: Use string_to_category instead of string_to_memory_category
# line 124 expects i32 - string_to_category returns MemoryCategory, which needs .into()
content = content.replace(
    'category: string_to_memory_category(&row.get::<_, String>(2)?).unwrap_or(MemoryCategory::CategoryFact) as i32,',
    'category: string_to_category(&row.get::<_, String>(2)?).unwrap_or(MemoryCategory::CategoryFact).into(),'
)

# Fix 3: scope_to_string is private, we need to export or use a public function
# But currently there's string_to_scope which is already defined
# Let's just remove the scope_to_string call and use string_to_scope with error handling

# The error message said "function scope_to_string is private" - this might be in tests
# Let's make the helper functions public so they can be used from tests

content = content.replace(
    'fn scope_to_string(scope: i32) -> Result<String> {',
    'pub fn scope_to_string(scope: i32) -> Result<String> {'
)

content = content.replace(
    'fn current_timestamp() -> Result<i64> {',
    'pub fn current_timestamp() -> Result<i64> {'
)

# Write back
with open('/data/data/com.termux/files/home/specgen/core/src/memory.rs', 'w') as f:
    f.write(content)

print("Fixed memory.rs: Helper function visibility and function names")
