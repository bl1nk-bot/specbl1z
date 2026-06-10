//! specgen-sandbox — OpenCode SDK in Rust
//!
//! Daytona + Modal sandbox management.
//! Runs on Termux/Android via CLI. Uses Open Bridge JSON I/O.
//!
//! ## Commands
//!
//! ```sh
//! specgen sandbox spin  --provider daytona --repo <url> [--branch main]
//! specgen sandbox exec  --id <sandbox> --cmd "make all"
//! specgen sandbox sync  --id <sandbox> [--branch main]
//! specgen sandbox delete --id <sandbox>
//! specgen sandbox compare --repo <url> --daytona-webhook <url> --modal-webhook <url>
//! ```

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;

// ---- types ----

#[derive(Debug, Serialize, Deserialize)]
pub struct SandboxResult {
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sandbox_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_per_hour: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct DaytonaResponse {
    id: String,
    name: String,
    #[serde(default)]
    exit_code: i32,
    #[serde(default)]
    output: String,
    #[serde(default)]
    success: bool,
    #[serde(default)]
    sandbox_id: String,
    #[serde(default)]
    sandbox_name: String,
    #[serde(default)]
    workspace_path: String,
    #[serde(default)]
    cost_per_hour: f64,
}

// ---- provider traits ----

pub trait SandboxProvider {
    fn name(&self) -> &str;
    fn base_url(&self) -> &str;
    fn auth_header(&self) -> Result<String>;
    fn cost_per_hour(&self) -> f64;
}

// ---- Daytona ----

pub struct Daytona {
    api_key: String,
    api_url: String,
}

impl Daytona {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            api_key: env::var("DAYTONA_API_KEY").context("DAYTONA_API_KEY not set")?,
            api_url: env::var("DAYTONA_API_URL")
                .unwrap_or_else(|_| "https://api.daytona.io".into()),
        })
    }
}

impl SandboxProvider for Daytona {
    fn name(&self) -> &str {
        "daytona"
    }
    fn base_url(&self) -> &str {
        &self.api_url
    }
    fn auth_header(&self) -> Result<String> {
        Ok(format!("Bearer {}", self.api_key))
    }
    fn cost_per_hour(&self) -> f64 {
        0.05
    }
}

// ---- Modal ----

pub struct Modal {
    token_id: String,
    token_secret: String,
}

impl Modal {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            token_id: env::var("MODAL_TOKEN_ID").context("MODAL_TOKEN_ID not set")?,
            token_secret: env::var("MODAL_TOKEN_SECRET").context("MODAL_TOKEN_SECRET not set")?,
        })
    }
}

impl SandboxProvider for Modal {
    fn name(&self) -> &str {
        "modal"
    }
    fn base_url(&self) -> &str {
        "https://api.modal.com/v1"
    }
    fn auth_header(&self) -> Result<String> {
        use base64::{engine::general_purpose::STANDARD, Engine};
        Ok(format!(
            "Basic {}",
            STANDARD.encode(format!("{}:{}", self.token_id, self.token_secret))
        ))
    }
    fn cost_per_hour(&self) -> f64 {
        0.002
    }
}

// ---- sandbox operations ----

fn api_post<T: SandboxProvider>(
    provider: &T,
    path: &str,
    body: &serde_json::Value,
) -> Result<DaytonaResponse> {
    let url = format!("{}{}", provider.base_url(), path);
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(&url)
        .header("Authorization", provider.auth_header()?)
        .header("Content-Type", "application/json")
        .json(body)
        .send()
        .with_context(|| format!("POST {} failed", url))?;

    let data: DaytonaResponse = resp.json().context("parse response")?;
    Ok(data)
}

fn api_delete<T: SandboxProvider>(provider: &T, path: &str) -> Result<()> {
    let url = format!("{}{}", provider.base_url(), path);
    let client = reqwest::blocking::Client::new();
    client
        .delete(&url)
        .header("Authorization", provider.auth_header()?)
        .send()
        .with_context(|| format!("DELETE {} failed", url))?;
    Ok(())
}

pub fn spin<T: SandboxProvider>(
    provider: &T,
    repo_url: &str,
    branch: &str,
    name: &str,
) -> Result<SandboxResult> {
    let data = api_post(
        provider,
        "/sandbox",
        &serde_json::json!({
            "image": "antigravity-opencode-rust",
            "name": name
        }),
    )?;

    api_post(
        provider,
        &format!("/sandbox/{}/clone", data.id),
        &serde_json::json!({
            "repoUrl": repo_url,
            "branch": branch,
            "path": "/workspace"
        }),
    )?;

    Ok(SandboxResult {
        ok: true,
        sandbox_id: Some(data.id),
        workspace: Some("/workspace".into()),
        cost_per_hour: Some(provider.cost_per_hour()),
        output: None,
        error: None,
    })
}

pub fn exec<T: SandboxProvider>(
    provider: &T,
    sandbox_id: &str,
    command: &str,
) -> Result<SandboxResult> {
    let data = api_post(
        provider,
        &format!("/sandbox/{}/exec", sandbox_id),
        &serde_json::json!({"command": command}),
    )?;

    Ok(SandboxResult {
        ok: data.exit_code == 0,
        sandbox_id: Some(sandbox_id.into()),
        workspace: None,
        cost_per_hour: None,
        output: Some(data.output),
        error: if data.exit_code != 0 {
            Some(format!("exit code {}", data.exit_code))
        } else {
            None
        },
    })
}

pub fn sync<T: SandboxProvider>(
    provider: &T,
    sandbox_id: &str,
    branch: &str,
    message: &str,
) -> Result<SandboxResult> {
    let cmds = [
        "git add .",
        let sanitized = message.replace('\"', "\\\"");
        &format!("git commit -m \"{}\"", sanitized),
        &format!("git push origin {}", branch),
    ];

    for cmd in &cmds {
        let r = exec(provider, sandbox_id, cmd)?;
        if !r.ok {
            return Ok(SandboxResult {
                ok: false,
                sandbox_id: Some(sandbox_id.into()),
                workspace: None,
                cost_per_hour: None,
                output: r.output,
                error: Some(format!("command failed: {}", cmd)),
            });
        }
    }

    Ok(SandboxResult {
        ok: true,
        sandbox_id: Some(sandbox_id.into()),
        workspace: None,
        cost_per_hour: None,
        output: None,
        error: None,
    })
}

pub fn delete<T: SandboxProvider>(provider: &T, sandbox_id: &str) -> Result<SandboxResult> {
    // Daytona uses /sandbox/{id}, Modal uses /sandbox/{id}
    api_delete(provider, &format!("/sandbox/{}", sandbox_id))?;
    Ok(SandboxResult {
        ok: true,
        sandbox_id: Some(sandbox_id.into()),
        workspace: None,
        cost_per_hour: None,
        output: None,
        error: None,
    })
}

pub fn compare(
    repo_url: &str,
    daytona_webhook: &str,
    modal_webhook: &str,
    hours: f64,
) -> Result<serde_json::Value> {
    let client = reqwest::blocking::Client::new();

    let spin = |url: &str| -> Result<DaytonaResponse> {
        let resp = client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "repo_url": repo_url,
                "branch": "main",
                "sandbox_name": "price-check"
            }))
            .send()
            .with_context(|| format!("webhook {} failed", url))?;
        resp.json().context("parse webhook response")
    };

    let d = spin(daytona_webhook)?;
    let m = spin(modal_webhook)?;

    let dc = d.cost_per_hour.max(0.05) * hours;
    let mc = m.cost_per_hour.max(0.002) * hours;
    let cheaper = if dc < mc { "daytona" } else { "modal" };

    Ok(serde_json::json!({
        "daytona": { "sandbox_id": d.sandbox_id, "cost_per_hour": d.cost_per_hour.max(0.05), "total": dc },
        "modal":   { "sandbox_id": m.sandbox_id, "cost_per_hour": m.cost_per_hour.max(0.002), "total": mc },
        "cheaper": cheaper,
        "savings": (dc - mc).abs()
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sandbox_result_json() {
        let r = SandboxResult {
            ok: true,
            sandbox_id: Some("abc123".into()),
            workspace: Some("/workspace".into()),
            cost_per_hour: Some(0.05),
            output: None,
            error: None,
        };
        let json = serde_json::to_string(&r).unwrap();
        assert!(json.contains("abc123"));
        assert!(json.contains("0.05"));
    }
}
