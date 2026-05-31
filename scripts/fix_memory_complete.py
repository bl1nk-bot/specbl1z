#!/data/data/com.termux/files/usr/bin/python3
# Fix memory.rs thoroughly - preserve original functions

# Read memory.rs
with open('/data/data/com.termux/files/home/specgen/core/src/memory.rs', 'r') as f:
    content = f.read()

# Fix 1: Export helper functions (make them public)
content = content.replace(
    'fn string_to_memory_scope(s: &str) -> Option<MemoryScope> {',
    'pub fn string_to_memory_scope(s: &str) -> Option<MemoryScope> {'
)

content = content.replace(
    'fn string_to_memory_category(s: &str) -> Option<MemoryCategory> {',
    'pub fn string_to_memory_category(s: &str) -> Option<MemoryCategory> {'
)

content = content.replace(
    'fn scope_to_string(scope: i32) -> Result<String> {',
    'pub fn scope_to_string(scope: i32) -> Result<String> {'
)

content = content.replace(
    'fn current_timestamp() -> Result<i64> {',
    'pub fn current_timestamp() -> Result<i64> {'
)

# Fix 2: Fix topic field type (remove 'as i32' cast if unnecessary)
# Looking at line 125 in original:
# topic: string_to_topic(&row.get::<_, String>(3)?).unwrap_or(MemoryTopic::TopicLearn) as i32,

# The string_to_topic returns MemoryTopic enum, which might be directly compatible
# with the ProtoMemoryEntry's topic field, or may need conversion to i32
# Let's try just using .into() if MemoryTopic implements Into<i32> or similar

# First, let's remove the 'as i32' cast and try .into()
content = content.replace(
    'topic: string_to_topic(&row.get::<_, String>(3)?).unwrap_or(MemoryTopic::TopicLearn) as i32,',
    'topic: string_to_topic(&row.get::<_, String>(3)?).unwrap_or(MemoryTopic::TopicLearn).into(),'
)

# Write back
with open('/data/data/com.termux/files/home/specgen/core/src/memory.rs', 'w') as f:
    f.write(content)

print("Fixed memory.rs: Helper visibility and topic field")
