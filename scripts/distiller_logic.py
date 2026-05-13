import json
import re

def analyze_skill(content: str) -> str:
    """
    Analyze SKILL.md content to extract metadata, tags, and heuristic scores.
    Returns a JSON string to be consumed by Rust.
    """
    name = "Unknown"
    description = ""
    tags = set()
    score = 10
    
    # 1. Frontmatter parsing
    name_match = re.search(r"(?i)^name:\s*(.+)$", content, re.MULTILINE)
    if name_match:
        name = name_match.group(1).strip()
    else:
        # Fallback to first H1
        h1_match = re.search(r"^#\s+(.+)$", content, re.MULTILINE)
        if h1_match:
            name = h1_match.group(1).strip()
            
    desc_match = re.search(r"(?i)^description:\s*(.+)$", content, re.MULTILINE)
    if desc_match:
        description = desc_match.group(1).strip()

    # 2. Tagging via Keyword matching
    content_lower = content.lower()
    
    keyword_map = {
        "frontend": ["react", "vue", "svelte", "frontend", "ui/ux", "tailwind"],
        "backend": ["express", "fastapi", "django", "backend", "sql", "database"],
        "rust": ["rust", "cargo", "fn main", "tokio"],
        "python": ["python", "def ", "import ", "pip ", "pytest"],
        "typescript": ["typescript", "npm ", "npx ", "interface ", "type "],
        "workflow": ["workflow", "step 1", "process", "guide"],
        "mcp": ["mcp ", "model context protocol", "server"],
        "firebase": ["firebase", "firestore", "genkit"]
    }
    
    for tag, keywords in keyword_map.items():
        if any(kw in content_lower for kw in keywords):
            tags.add(tag)
            
    # 3. Slop & Heuristics Detection
    # Penalty for "As an AI"
    if "as an ai" in content_lower or "i cannot" in content_lower:
        score -= 3
        tags.add("ai_slop_suspect")
        
    # Check code ratio
    code_blocks = re.findall(r"```.*?```", content, re.DOTALL)
    code_length = sum(len(b) for b in code_blocks)
    total_length = len(content)
    
    if total_length > 2000 and code_length < 100:
        # Long document, almost no code - might be generic text
        score -= 2
        tags.add("text_heavy")
        
    if total_length < 150:
        score -= 5 # Too short to be a useful skill
        tags.add("stub")
        
    # Ensure score bounds
    score = max(1, min(10, score))
    
    result = {
        "name": name,
        "description": description,
        "tags": list(tags),
        "quality_score": score,
        "is_slop": score < 5,
        "word_count": len(content.split())
    }
    
    return json.dumps(result)
