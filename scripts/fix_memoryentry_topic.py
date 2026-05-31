# Fix MemoryEntry missing topic field in main.rs

with open('/data/data/com.termux/files/home/specgen/cli/src/main.rs', 'r') as f:
    lines = f.readlines()

# Find the MemoryEntry initialization and add topic field after category
for i, line in enumerate(lines):
    if 'category: category_enum as i32,' in line and i > 1180:
        # Insert topic field after category
        lines.insert(i + 1, '                    topic: 0,')
        break

with open('/data/data/com.termux/files/home/specgen/cli/src/main.rs', 'w') as f:
    f.writelines(lines)

print("Added topic field to MemoryEntry in main.rs")
