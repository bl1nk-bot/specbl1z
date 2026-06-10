# Daytona vs Modal — Comparison Reference

## Quick Comparison

| Feature | Daytona | Modal |
|---------|---------|-------|
| Sandbox type | Full container (Docker) | Serverless container |
| Startup time | <90ms | <1s |
| CPU cost | $0.05/hr | $0.002/hr |
| GPU available | No | Yes (T4, A100, H100) |
| Max runtime | Unlimited | 1hr (extendable) |
| Persistence | Snapshots | Volumes |
| SDK languages | TS, Python, Go, Ruby, Java | Python, TS |
| OpenCode plugin | Yes (.opencode/) | Yes (.opencode/) |
| Rust SDK | Yes (sandbox/) | Yes (sandbox/) |
| Termux support | Yes | Yes |
| Webhook | Yes | Yes |
| API auth | Bearer token | Token ID + Secret |

## When to use Daytona
- Full container environment needed (install any tool)
- Unlimited session duration
- Native OpenCode integration
- Team/organization management features
- Snapshots for state persistence across sessions

## When to use Modal
- GPU compute needed (ML training, inference)
- Cost-sensitive CPU workloads (25x cheaper than Daytona)
- Serverless model — only pay for active compute time
- Python-first workflows
- Large-scale parallel execution

## Cost Comparison (1 hour session)

| Workload | Daytona | Modal |
|----------|---------|-------|
| CPU build | $0.05 | $0.002 |
| GPU T4 | N/A | $0.70 |
| GPU A100 | N/A | $1.10 |
| Month (720hr CPU) | $36 | $1.44 |

## API Differences

Daytona:
```
POST https://app.daytona.io/api/sandbox
Authorization: Bearer <key>
```

Modal:
```
POST https://api.modal.com/v1/sandbox
Authorization: Basic base64(token_id:token_secret)
```
