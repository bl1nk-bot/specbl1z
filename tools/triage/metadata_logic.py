    # Metadata Slop Detection (Phase 1)
    meta_slop_patterns = {
        "experimental_rule": r"experimental|beta|preview",
        "example_context": r"example|sample|fixture|demo",
        "generic_alert": r"potential\s+\w+\s+issue|generic\s+pattern",
        "low_confidence": r"low\s+confidence|statistical\s+analysis"
    }
    for name, pattern in meta_slop_patterns.items():
        if re.search(pattern, summary, re.IGNORECASE):
            score += 15
            findings.append(f"Metadata Slop: {name} (+15)")
