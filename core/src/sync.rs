use anyhow::{Result, anyhow};
use std::process::Command;
use std::path::Path;
use reqwest::blocking::Client;
use serde_json::{json, Value};

pub struct SyncManager;

impl SyncManager {
    /// Prepares a git patch for all uncommitted changes.
    pub fn prepare_patch(root: &Path) -> Result<String> {
        let output = Command::new("git")
            .arg("diff")
            .arg("HEAD")
            .current_dir(root)
            .output()?;

        if !output.status.success() {
            return Err(anyhow!("Git diff failed: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Applies a patch back to the filesystem.
    pub fn apply_patch(root: &Path, patch_content: &str) -> Result<()> {
        use std::io::Write;
        let patch_path = root.join("remote_sync.patch");
        let mut file = std::fs::File::create(&patch_path)?;
        file.write_all(patch_content.as_bytes())?;

        let output = Command::new("git")
            .arg("apply")
            .arg("remote_sync.patch")
            .current_dir(root)
            .output()?;

        std::fs::remove_file(patch_path)?;

        if !output.status.success() {
            return Err(anyhow!("Failed to apply patch: {}", String::from_utf8_lossy(&output.stderr)));
        }

        Ok(())
    }

    /// Sends a git patch to a remote endpoint (KiloCode Gateway / Modal).
    /// Endpoint URL can be overridden via parameter or KILOCODE_URL env var.
    pub fn send_patch(
        patch_content: &str,
        endpoint_override: Option<&str>,
    ) -> Result<Value> {
        let endpoint = endpoint_override
            .map(|s| s.to_string())
            .or_else(|| std::env::var("KILOCODE_URL").ok())
            .unwrap_or_else(|| "http://localhost:8000/sync".to_string());

        if patch_content.is_empty() {
            return Err(anyhow!("No changes to sync - patch is empty"));
        }

        let client = Client::new();
        let payload = json!({
            "patch": patch_content,
            "source": "specgen-cli",
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        let response = client
            .post(&endpoint)
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .map_err(|e| anyhow!("HTTP request failed: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().unwrap_or_default();
            return Err(anyhow!("Server returned {}: {}", status, text));
        }

        let result: Value = response
            .json()
            .map_err(|e| anyhow!("Failed to parse JSON response: {}", e))?;

        Ok(result)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_prepare_patch_empty_repo() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        // Initialize a git repo
        std::process::Command::new("git")
            .arg("init")
            .current_dir(root)
            .output()
            .unwrap();

        // Configure git identity
        std::process::Command::new("git")
            .arg("config")
            .arg("user.email")
            .arg("test@example.com")
            .current_dir(root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .arg("config")
            .arg("user.name")
            .arg("Test")
            .current_dir(root)
            .output()
            .unwrap();

        // Create an initial commit so HEAD exists
        fs::write(root.join("README.md"), "# Test").unwrap();
        std::process::Command::new("git")
            .arg("add")
            .arg("README.md")
            .current_dir(root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("initial")
            .current_dir(root)
            .output()
            .unwrap();

        // No uncommitted changes => empty patch
        let result = SyncManager::prepare_patch(root).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn test_prepare_patch_with_modifications() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        // Initialize repo
        std::process::Command::new("git")
            .arg("init")
            .current_dir(root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .arg("config")
            .arg("user.email")
            .arg("test@example.com")
            .current_dir(root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .arg("config")
            .arg("user.name")
            .arg("Test")
            .current_dir(root)
            .output()
            .unwrap();

        // Initial commit
        fs::write(root.join("file.txt"), "original").unwrap();
        std::process::Command::new("git")
            .arg("add")
            .arg("file.txt")
            .current_dir(root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("initial")
            .current_dir(root)
            .output()
            .unwrap();

        // Make a modification
        fs::write(root.join("file.txt"), "modified").unwrap();

        let patch = SyncManager::prepare_patch(root).unwrap();
        assert!(!patch.is_empty());
        assert!(patch.contains("modified"));
    }

    #[test]
    fn test_apply_patch() {
        let temp_dir = tempdir().unwrap();
        let root = temp_dir.path();

        // Initialize repo
        std::process::Command::new("git")
            .arg("init")
            .current_dir(root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .arg("config")
            .arg("user.email")
            .arg("test@example.com")
            .current_dir(root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .arg("config")
            .arg("user.name")
            .arg("Test")
            .current_dir(root)
            .output()
            .unwrap();

        // Initial file
        fs::write(root.join("test.txt"), "line1\n").unwrap();
        std::process::Command::new("git")
            .arg("add")
            .arg("test.txt")
            .current_dir(root)
            .output()
            .unwrap();
        std::process::Command::new("git")
            .arg("commit")
            .arg("-m")
            .arg("initial")
            .current_dir(root)
            .output()
            .unwrap();

        // Create a patch that modifies the file
        let mut patch = String::new();
        patch.push_str("--- a/test.txt\n");
        patch.push_str("+++ b/test.txt\n");
        patch.push_str("@@ -1 +1 @@\n");
        patch.push_str("-line1\n");
        patch.push_str("+line2\n");

        // Apply the patch
        SyncManager::apply_patch(root, &patch).unwrap();

        // Verify the file was modified
        let content = fs::read_to_string(root.join("test.txt")).unwrap();
        assert_eq!(content, "line2\n");
    }
}
