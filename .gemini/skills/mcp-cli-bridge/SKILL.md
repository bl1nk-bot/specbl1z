---
name: mcp-cli-bridge
description: How to implement a robust MCP server that dispatches domain-specific tool calls to a compiled CLI (e.g., Rust/Go). Use this when building MCP integrations for existing CLI tools, or when platform constraints (like Termux/Android) prevent direct usage of native Node.js database drivers. This skill teaches the 'Generic Dispatcher' pattern to keep the bridge layer thin and performant.
---

# MCP-CLI Bridge Pattern

This pattern allows you to expose complex logic from a compiled CLI to AI agents via MCP without duplicating code in the bridge layer.

## Implementation Steps

### 1. Define Tool Schemas Externally
Store tool definitions in a dedicated file (e.g., `app/src/domain_tools.ts`) to keep the main server file clean.
- Exclude standard agent capabilities (file I/O, shell execution) to avoid tool overlap and agent confusion.
- Focus only on domain-specific operations (e.g., `document_create`, `rule_list`).

### 2. Update the CLI for Generic Dispatch
The CLI must be able to handle dynamic tool names and JSON-encoded arguments.
- Add a `db` or `mcp` subcommand group.
- Implement a `Generic` handler that accepts a tool name and an `--args` string containing JSON.

**Rust (Clap) example:**
```rust
#[derive(Subcommand)]
enum DbCommands {
    /// Generic handler for MCP tool dispatch
    #[command(external_subcommand)]
    Generic(Vec<String>),
}
```

### 3. Implement the MCP Handler
In `mcp.ts`, implement a catch-all handler that forwards requests to the CLI.

```typescript
server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;
  
  // Forward to CLI
  const projectRoot = path.resolve(__dirname, "../../");
  const argsJson = JSON.stringify(args);
  const command = `cargo run -p specgen --quiet -- db ${name} --args '${argsJson}'`;
  
  try {
    const { stdout, stderr } = await execAsync(command, { cwd: projectRoot });
    if (stderr && !stdout) return { content: [{ type: "text", text: stderr }], isError: true };
    return { content: [{ type: "text", text: stdout }] };
  } catch (error: any) {
    return { content: [{ type: "text", text: error.message }], isError: true };
  }
});
```

## Best Practices & Pitfalls

### 🛡️ Failure Shields
- **Version Parity**: Ensure the CLI and MCP server are part of the same monorepo or keep their schemas synced.
- **Zero-Overlap**: Never implement tools that standard agents already have (e.g., `read_file`). This creates "Tool Choice Paralysis" in the agent.
- **Silent Mode**: Always use quiet flags (e.g., `--quiet`) when executing the CLI to prevent non-JSON output (like build logs) from corrupting the response.
- **Schema Validation**: Let the CLI handle the heavy validation; the MCP server should act as a pass-through.

### 🧪 Verification Checklist
- [ ] Tool schemas are registered and visible to the agent.
- [ ] CLI handles dynamic tool names without crashing.
- [ ] JSON arguments are correctly escaped when passed to the shell.
- [ ] Standard agent tools (file/shell) are NOT duplicated.
