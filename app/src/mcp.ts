import { Server } from "@modelcontextprotocol/sdk/server/index.js";
import { StdioServerTransport } from "@modelcontextprotocol/sdk/server/stdio.js";
import {
  CallToolRequestSchema,
  ListToolsRequestSchema,
} from "@modelcontextprotocol/sdk/types.js";
import { sql } from "./db.js";
import { exec } from "child_process";
import { promisify } from "util";
import path from "path";
import { fileURLToPath } from "url";
import fs from "fs/promises";
import { CRAFT_TOOLS } from "./craft_tools.js";

const execAsync = promisify(exec);
const __dirname = path.dirname(fileURLToPath(import.meta.url));
const TRIAGE_SCRIPT_PATH = path.resolve(__dirname, "../../tools/triage/gh-bl1nk-triage");

/**
 * MCP Server for bl1nk coding standards and security triage.
 */
const server = new Server(
  {
    name: "bl1nk-mcp",
    version: "1.2.0",
  },
  {
    capabilities: {
      tools: {},
    },
  }
);

server.setRequestHandler(ListToolsRequestSchema, async () => {
  return {
    tools: [
      {
        name: "get_standards",
        description: "Get all coding standards and rules from the database",
        inputSchema: {
          type: "object",
          properties: {},
        },
      },
      {
        name: "triage_security",
        description: "Run security triage (Dependabot, CodeQL, Secret Scanning) on a GitHub repository",
        inputSchema: {
          type: "object",
          properties: {
            repo: {
              type: "string",
              description: "The GitHub repository in OWNER/REPO format",
            },
          },
          required: ["repo"],
        },
      },
      ...CRAFT_TOOLS,
    ],
  };
});

server.setRequestHandler(CallToolRequestSchema, async (request) => {
  const { name, arguments: args } = request.params;
  const projectRoot = path.resolve(__dirname, "../../");

  // --- Core / Legacy Tools ---
  if (name === "get_standards") {
    try {
      const { stdout, stderr } = await execAsync(`cargo run -p specgen --quiet -- rule list --format json`, { cwd: projectRoot });
      if (stderr && !stdout) return { content: [{ type: "text", text: `Error: ${stderr}` }], isError: true };
      const rules = JSON.parse(stdout);
      return { content: [{ type: "text", text: JSON.stringify(rules, null, 2) }] };
    } catch (error: any) {
      return { content: [{ type: "text", text: `Failed to fetch rules: ${error.message}` }], isError: true };
    }
  }

  if (name === "triage_security") {
    const { repo } = args as { repo: string };
    try {
      const { stdout, stderr } = await execAsync(`bash "${TRIAGE_SCRIPT_PATH}" -r ${repo}`);
      if (stderr && !stdout) return { content: [{ type: "text", text: `Error: ${stderr}` }], isError: true };
      return { content: [{ type: "text", text: stdout }] };
    } catch (error: any) {
      return { content: [{ type: "text", text: `Execution failed: ${error.message}` }], isError: true };
    }
  }

  // --- Generic Craft Tool Handler (Map to Specgen CLI) ---
  const isCraftTool = CRAFT_TOOLS.some(t => t.name === name);
  if (isCraftTool) {
    try {
      // Map tool name to CLI: e.g. folders_list -> db folders_list
      // Passing arguments as a JSON string to the CLI
      const jsonArgs = JSON.stringify(args);
      const command = `cargo run -p specgen --quiet -- db ${name} --args '${jsonArgs.replace(/'/g, "'\\''")}'`;
      
      const { stdout, stderr } = await execAsync(command, { cwd: projectRoot });
      
      if (stderr && !stdout) {
        return {
          content: [{ type: "text", text: `CLI Error: ${stderr}` }],
          isError: true,
        };
      }

      return {
        content: [{ type: "text", text: stdout }],
      };
    } catch (error: any) {
      return {
        content: [{ type: "text", text: `CLI Execution failed: ${error.message}\nNote: Many Craft tools require implementing corresponding Rust logic in 'specgen db'.` }],
        isError: true,
      };
    }
  }

  throw new Error(`Tool not found: ${name}`);
});

async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error("bl1nk MCP Server running on stdio");
}

main().catch((error) => {
  console.error("Fatal error in main():", error);
  process.exit(1);
});
