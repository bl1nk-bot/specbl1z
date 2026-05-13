pub fn detect_blockers(labels: &Vec<String>) -> bool {
    for label in labels {
        let label = label.to_lowercase();
        if label.contains("blocking") || label.contains("conflict") || label.contains("rev:expert") {
            return true;
        }
    }
    false
}
