import { definePlugin, tool } from "@opencode-ai/sdk";
import { z } from "zod";

export default definePlugin({
  name: "webhook",
  description: "Spin sandboxes via webhook (Daytona or Modal)",

  tools: [
    tool({
      name: "spin",
      description: "Call webhook to spin a sandbox",
      input: z.object({
        webhookUrl: z.string().describe("Webhook URL"),
        repoUrl: z.string(),
        branch: z.string().default("main"),
        name: z.string().default("specgen-session"),
      }),
      async execute({ webhookUrl, repoUrl, branch, name }, ctx) {
        const token = ctx.env.WEBHOOK_AUTH_TOKEN;
        const headers: Record<string, string> = { "Content-Type": "application/json" };
        if (token) headers["Authorization"] = `Bearer ${token}`;

        const r = await fetch(webhookUrl, {
          method: "POST",
          headers,
          body: JSON.stringify({ repo_url: repoUrl, branch, sandbox_name: name }),
        });
        const data = await r.json();
        return {
          ok: data.success ?? r.ok,
          sandboxId: data.sandbox_id,
          name: data.sandbox_name,
          workspace: data.workspace_path,
          costPerHour: data.cost_per_hour,
        };
      },
    }),

    tool({
      name: "compare",
      description: "Spin both Daytona + Modal sandboxes and compare cost",
      input: z.object({
        daytonaWebhook: z.string(),
        modalWebhook: z.string(),
        repoUrl: z.string(),
        hours: z.number().default(1),
      }),
      async execute({ daytonaWebhook, modalWebhook, repoUrl, hours }, ctx) {
        const token = ctx.env.WEBHOOK_AUTH_TOKEN;
        const headers: Record<string, string> = { "Content-Type": "application/json" };
        if (token) headers["Authorization"] = `Bearer ${token}`;

        const spin = async (url: string) => {
          const r = await fetch(url, {
            method: "POST",
            headers,
            body: JSON.stringify({ repo_url: repoUrl, branch: "main", sandbox_name: "price-check" }),
          });
          return r.json();
        };

        const [daytona, modal] = await Promise.all([spin(daytonaWebhook), spin(modalWebhook)]);

        const dc = (daytona.cost_per_hour ?? 0.05) * hours;
        const mc = (modal.cost_per_hour ?? 0.002) * hours;
        const cheaper = dc < mc ? "daytona" : "modal";

        return {
          daytona: { sandboxId: daytona.sandbox_id, costPerHour: daytona.cost_per_hour ?? 0.05, total: dc },
          modal: { sandboxId: modal.sandbox_id, costPerHour: modal.cost_per_hour ?? 0.002, total: mc },
          cheaper,
          savings: Math.abs(dc - mc),
        };
      },
    }),
  ],
});
