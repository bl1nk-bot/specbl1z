# Fix CodeSense::new calls to pass database reference

with open('/data/data/com.termux/files/home/specgen/cli/src/main.rs', 'r') as f:
    content = f.read()

# Fix line 765 - add database reference
# The database variable is defined from the Commands parameter
# Need to check where Commands::Index accepts database as a parameter

# Replace: let sense = specgen_core::sense::CodeSense::new(&root)?;
# With something like: let sense = specgen_core::sense::CodeSense::new(&db, &root)?;

# Let's check what Commands::Index looks like in the enum definition
import re

# Find the Commands::Index definition in the file
match = re.search(r'Index\s*\{[^}]+\}', content)
if match:
    index_cmd = match.group(0)
    print(f"Found Commands::Index: {index_cmd}")

# For now, let's just add a placeholder database variable
# The pattern should be similar to how MemoryStore::new is called above
content = content.replace(
    'let sense = specgen_core::sense::CodeSense::new(&root)?;',
    '''let sense = specgen_core::sense::CodeSense::new(&database, &root)?;'''
)

with open('/data/data/com.termux/files/home/specgen/cli/src/main.rs', 'w') as f:
    f.write(content)

print("Fixed CodeSense::new calls - added database reference")
