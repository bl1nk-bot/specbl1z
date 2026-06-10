# OpenCode SDK & Daytona SDK — Research Reference
# Fetched: 2025-06-10

## OpenCode Plugin API (actual)

### Plugin format
```typescript
// plugin is a FUNCTION, not definePlugin
export type Plugin = (input: PluginInput, options?: PluginOptions) => Promise<Hooks>

export type PluginModule = {
  id?: string
  server: Plugin    // <-- export this
}
```

### PluginInput
```typescript
export type PluginInput = {
  client: ReturnType<typeof createOpencodeClient>
  project: Project
  directory: string
  worktree: string
  serverUrl: URL
  $: BunShell
  experimental_workspace: {
    register(type: string, adapter: WorkspaceAdapter): void
  }
}
```

### Hooks (what plugin returns)
```typescript
export interface Hooks {
  dispose?: () => Promise<void>
  event?: (input: { event: Event }) => Promise<void>
  config?: (input: Config) => Promise<void>
  tool?: {
    [key: string]: ToolDefinition  // <-- tools go here
  }
  auth?: AuthHook
  provider?: ProviderHook
  "chat.message"?: (...)
  "chat.params"?: (...)
}
```

### Tool Definition
```typescript
import { z } from "zod"

export function tool<Args extends z.ZodRawShape>(input: {
  description: string
  args: Args                       // zod schema
  execute(args: z.infer<z.ZodObject<Args>>, context: ToolContext): Promise<ToolResult>
}) { return input }

tool.schema = z
export type ToolDefinition = ReturnType<typeof tool>
```

### ToolContext
```typescript
export type ToolContext = {
  sessionID: string
  messageID: string
  agent: string
  directory: string
  worktree: string
  abort: AbortSignal
  metadata(input: { title?: string; metadata?: { [key: string]: any } }): void
  ask(input: AskInput): Promise<void>
}
```

### ToolResult
```typescript
export type ToolResult =
  | string
  | {
      title?: string
      output: string
      metadata?: { [key: string]: any }
      attachments?: ToolAttachment[]
    }
```

### WorkspaceAdapter (for sandbox providers)
```typescript
export type WorkspaceAdapter = {
  name: string
  description: string
  configure(config: WorkspaceInfo): WorkspaceInfo | Promise<WorkspaceInfo>
  create(config: WorkspaceInfo, env: Record<string, string | undefined>, from?: WorkspaceInfo): Promise<void>
  remove(config: WorkspaceInfo): Promise<void>
  target(config: WorkspaceInfo): WorkspaceTarget | Promise<WorkspaceTarget>
}
```

### Package location
- Internal to OpenCode monorepo: `packages/plugin/` and `packages/sdk/`
- NOT published to npm as `@opencode-ai/sdk`
- Import from monorepo directly: `"@opencode-ai/sdk"` (internal alias)

## Daytona SDK

### TypeScript SDK
```
npm install @daytonaio/sdk
```
```typescript
import { Daytona } from "@daytonaio/sdk"
const daytona = new Daytona({ apiKey: '...' })
const sandbox = await daytona.create({ language: 'typescript' })
const response = await sandbox.process.codeRun('console.log("hi")')
await sandbox.delete()
```

### REST API
```
POST https://app.daytona.io/api/sandbox
Authorization: Bearer <key>
Content-Type: application/json
```

### Python SDK
```
pip install daytona
from daytona import Daytona, DaytonaConfig
```

### Go SDK
```
go get github.com/daytonaio/daytona/libs/sdk-go
```

### Ruby SDK
```
gem install daytona
```

### Java SDK
```
io.daytona:sdk-java:0.1.0
```

### OpenCode + Daytona integration
- Daytona has OpenCode guide: https://www.daytona.io/docs/guides/opencode/opencode-web-agent
- Daytona agent skills: `npx skills add https://github.com/daytona/skills --skill daytona`
- Daytona MCP server available

## Key differences from our current .opencode/plugins/

Our code used:                      Actual API:
  definePlugin({...})               export default async function(input, options)
  @opencode-ai/sdk                  Internal monorepo import
  tools: [tool({...})]              tool: { [key: string]: ToolDefinition }
  ctx.env.DAYTONA_API_KEY           Process env or PluginInput
  @opencode-ai/plugin               packages/plugin/src/index.ts
