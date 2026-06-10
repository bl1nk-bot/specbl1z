/// Daytona plugin for OpenCode
/// Uses actual OpenCode plugin API (not definePattern)
/// Reference: https://www.daytona.io/docs

import { z } from "zod"
import type { PluginInput, Hooks } from "../../../packages/plugin/src/index"

export default async function daytonaPlugin(
  input: PluginInput,
): Promise<Hooks> {
  const { $ } = input

  return {
    tool: {
      spin: {
        description: "Create Daytona sandbox and clone repo",
        args: z.object({
          repoUrl: z.string().describe("GitHub repo URL"),
          branch: z.string().default("main"),
          name: z.string().default("specgen-session"),
        }),
        async execute(args, ctx) {
          const key = process.env.DAYTONA_API_KEY
          if (!key) return "DAYTONA_API_KEY not set"

          const base = process.env.DAYTONA_API_URL ?? "https://app.daytona.io/api"
          const headers = {
            Authorization: `Bearer ${key}`,
            "Content-Type": "application/json",
          }

          const create = await fetch(`${base}/sandbox`, {
            method: "POST",
            headers,
            body: JSON.stringify({ name: args.name }),
          })
          const sandbox = await create.json()

          await fetch(`${base}/sandbox/${sandbox.id}/clone`, {
            method: "POST",
            headers,
            body: JSON.stringify({
              repoUrl: args.repoUrl,
              branch: args.branch,
              path: "/workspace",
            }),
          })

          return {
            output: `Sandbox ${sandbox.id} ready at /workspace`,
            metadata: {
              sandboxId: sandbox.id,
              name: args.name,
              workspace: "/workspace",
              repoUrl: args.repoUrl,
              branch: args.branch,
            },
          }
        },
      },

      exec: {
        description: "Run command inside Daytona sandbox",
        args: z.object({
          sandboxId: z.string(),
          command: z.string(),
        }),
        async execute(args, ctx) {
          const key = process.env.DAYTONA_API_KEY
          if (!key) return "DAYTONA_API_KEY not set"

          const base = process.env.DAYTONA_API_URL ?? "https://app.daytona.io/api"
          const r = await fetch(
            `${base}/sandbox/${args.sandboxId}/exec`,
            {
              method: "POST",
              headers: {
                Authorization: `Bearer ${key}`,
                "Content-Type": "application/json",
              },
              body: JSON.stringify({ command: args.command }),
            },
          )
          const out = await r.json()
          return {
            output: out.output ?? out.result ?? "",
            metadata: { exitCode: out.exitCode ?? out.exit_code },
          }
        },
      },

      sync: {
        description: "Git push sandbox changes to GitHub",
        args: z.object({
          sandboxId: z.string(),
          branch: z.string().default("main"),
          message: z.string().default("agent: sync from OpenCode"),
        }),
        async execute(args, ctx) {
          const key = process.env.DAYTONA_API_KEY
          if (!key) return "DAYTONA_API_KEY not set"

          const base =
            process.env.DAYTONA_API_URL ?? "https://app.daytona.io/api"
          const exec = async (cmd: string) => {
            const r = await fetch(
              `${base}/sandbox/${args.sandboxId}/exec`,
              {
                method: "POST",
                headers: {
                  Authorization: `Bearer ${key}`,
                  "Content-Type": "application/json",
                },
                body: JSON.stringify({ command: cmd }),
              },
            )
            return r.json()
          }

          for (const cmd of [
            "git add .",
            `git commit -m "${args.message}"`,
            `git push origin ${args.branch}`,
          ]) {
            const out = await exec(cmd)
            if ((out.exitCode ?? out.exit_code) !== 0) {
              return { output: `Failed: ${cmd}`, metadata: { ok: false } }
            }
          }

          return {
            output: `Synced to ${args.branch}`,
            metadata: { ok: true, branch: args.branch },
          }
        },
      },

      delete: {
        description: "Delete Daytona sandbox",
        args: z.object({ sandboxId: z.string() }),
        async execute(args, ctx) {
          const key = process.env.DAYTONA_API_KEY
          if (!key) return "DAYTONA_API_KEY not set"

          const base =
            process.env.DAYTONA_API_URL ?? "https://app.daytona.io/api"
          await fetch(`${base}/sandbox/${args.sandboxId}`, {
            method: "DELETE",
            headers: { Authorization: `Bearer ${key}` },
          })

          return { output: `Sandbox ${args.sandboxId} deleted` }
        },
      },
    },
  }
}
