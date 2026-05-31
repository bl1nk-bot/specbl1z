#!/data/data/com.termux/files/usr/bin/python3
# Fix MemoryTopic field type

# Read memory.rs
with open('/data/data/com.termux/files/home/specgen/core/src/memory.rs', 'r') as f:
    content = f.read()

# Fix line 125 - remove 'as i32' cast if ProtoMemoryEntry expects MemoryTopic
# The string_to_topic returns MemoryTopic, so we should use it directly
content = content.replace(
    'topic: string_to_topic(&row.get::<_, String>(3)?).unwrap_or(MemoryTopic::TopicLearn) as i32,',
    'topic: string_to_topic(&row.get::<_, String>(3)?).unwrap_or(MemoryTopic::TopicLearn).into(),'
)

# Write back
with open('/data/data/com.termux/files/home/specgen/core/src/memory.rs', 'w') as f:
    f.write(content)

print("Fixed memory.rs: MemoryTopic field type")
