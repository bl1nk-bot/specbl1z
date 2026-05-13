# Plan: Add Craft MCP Tools

## Objective
Update the TypeScript MCP server (`app/src/mcp.ts`) to register and handle a large list of Craft-related tools provided by the user.

## Key Files & Context
- `app/src/mcp.ts`: The main MCP server implementation.
- `app/src/craft_tools.ts`: New file to hold the large JSON schema.
- Tools to add cover: Folders, Documents, Search, Blocks, Collections, Tasks, Comments, Images, Connection Info, and generic utilities (`ask_user`, `run_shell_command`, `read_file`, `write_file`).

## Implementation Steps
1.  **Extract Tool Definitions:** Store the provided JSON schemas in a separate file `app/src/craft_tools.ts` exporting them as a constant array.
2.  **Update `ListToolsRequestSchema`:** Import the tool definitions from `craft_tools.ts` and merge them with the existing tools (`get_standards`, `triage_security`) in `mcp.ts`.
3.  **Update `CallToolRequestSchema`:** Add a dispatch mechanism in the request handler. As per the user's decision, we will map these MCP tool calls to execute the `specgen` CLI via `execAsync`.
    *   *Implementation Strategy:* Since not all CLI commands for these tools exist yet (e.g., `specgen db folders_list`), we will create a generic wrapper in `mcp.ts` that tries to call `cargo run -p specgen -- <tool_name> <arguments_as_json>` or similar. If the CLI command doesn't exist, it will naturally return an error, which the MCP server will relay. This sets up the plumbing, allowing us to implement the Rust CLI side iteratively later.
    *   For the generic utilities (`run_shell_command`, `read_file`, `write_file`), we can implement them directly in Node.js within `mcp.ts` to save CLI overhead.

## Verification & Testing
- Ensure the TypeScript code compiles.
- Run the MCP server to verify it starts without errors.
- Test `listTools` to ensure all new tools are exposed.