import { definePlugin, tool } from "@opencode-ai/sdk";
import { z } from "zod";

export default definePlugin({
  name: "modal",
  description: "Spin Modal.com sandbox, run agents, sync back to GitHub",

  tools: [
    tool({
      name: "spin",
      description: "Create Modal.com sandbox + clone repo",
      input: z.object({
        repoUrl: z.string().describe("GitHub repo URL"),
        branch: z.string().default("main"),
        name: z.string().default("specgen-session"),
      }),
      async execute({ repoUrl, branch, name }, ctx) {
        const tokenId = ctx.env.MODAL_TOKEN_ID;
        const tokenSecret = ctx.env.MODAL_TOKEN_SECRET;

        // Modal uses their Python SDK or REST API
        // This uses a simplified REST approach
        const r = await fetch("https://api.modal.com/v1/sandbox", {
          method: "POST",
          headers: {
            "Authorization": `Basic ${btoa(`${tokenId}:${tokenSecret}`)}`,
            "Content-Type": "application/json",
          },
          body: JSON.stringify({
            image: "debian-slim-python3.11-rust",
            name,
            setup: [
              "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y",
              "git clone ${repoUrl} /workspace",
              "cd /workspace && git checkout ${branch}",
            ],
          }),
        });
        const sandbox = await r.json();

        return {
          sandboxId: sandbox.id,
          name: sandbox.name,
          workspace: "/workspace",
          repoUrl,
          branch,
          costPerHour: 0.002,
        };
      },
    }),

    tool({
      name: "exec",
      description: "Run command inside Modal sandbox",
      input: z.object({
        sandboxId: z.string(),
        command: z.string(),
      }),
      async execute({ sandboxId, command }, ctx) {
        const tokenId = ctx.env.MODAL_TOKEN_ID;
        const tokenSecret = ctx.env.MODAL_TOKEN_SECRET;

        const r = await fetch(`https://api.modal.com/v1/sandbox/${sandboxId}/exec`, {
          method: "POST",
          headers: {
            "Authorization": `Basic ${btoa(`${tokenId}:${tokenSecret}`)}`,
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ command }),
        });
        const out = await r.json();
        return { ok: out.exitCode === 0, exitCode: out.exitCode, output: out.stdout };
      },
    }),

    tool({
      name: "sync",
      description: "Git push sandbox changes back to GitHub",
      input: z.object({
        sandboxId: z.string(),
        branch: z.string().default("main"),
        message: z.string().default("agent: sync from OpenCode sandbox"),
      }),
      async execute({ sandboxId, branch, message }, ctx) {
        const tokenId = ctx.env.MODAL_TOKEN_ID;
        const tokenSecret = ctx.env.MODAL_TOKEN_SECRET;

        for (const cmd of [
          "git add .",
          `git commit -m "${message}"`,
          `git push origin ${branch}`,
        ]) {
          await fetch(`https://api.modal.com/v1/sandbox/${sandboxId}/exec`, {
            method: "POST",
            headers: {
              "Authorization": `Basic ${btoa(`${tokenId}:${tokenSecret}`)}`,
              "Content-Type": "application/json",
            },
            body: JSON.stringify({ command: cmd }),
          });
        }
        return { ok: true, branch, message };
      },
    }),

    tool({
      name: "delete",
      description: "Delete Modal sandbox",
      input: z.object({ sandboxId: z.string() }),
      async execute({ sandboxId }, ctx) {
        const tokenId = ctx.env.MODAL_TOKEN_ID;
        const tokenSecret = ctx.env.MODAL_TOKEN_SECRET;

        await fetch(`https://api.modal.com/v1/sandbox/${sandboxId}`, {
          method: "DELETE",
          headers: {
            "Authorization": `Basic ${btoa(`${tokenId}:${tokenSecret}`)}`,
          },
        });
        return { ok: true, sandboxId };
      },
    }),
  ],
});
