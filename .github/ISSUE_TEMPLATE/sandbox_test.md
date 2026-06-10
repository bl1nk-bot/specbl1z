---
name: Sandbox Test
about: Test Daytona or Modal sandbox connection
title: "[SANDBOX] "
labels: ["sandbox", "testing"]
assignees: []
---

## Type
- [ ] Daytona
- [ ] Modal
- [ ] Cross-platform

## Test Steps
1. Set credentials (DAYTONA_API_KEY / MODAL_TOKEN_ID+SECRET)
2. Run: `cargo run -p specgen-sandbox -- spin --provider <type> --repo https://github.com/bl1nk-bot/specbl1z.git --branch <branch>`
3. Run: `cargo run -p specgen-sandbox -- exec --id <id> --cmd "make all"`
4. Verify exit code = 0
5. Run: `cargo run -p specgen-sandbox -- sync --id <id>`
6. Run: `cargo run -p specgen-sandbox -- delete --id <id>`

## Expected
- [ ] Sandbox created in < 2 min
- [ ] All 57 tests pass
- [ ] Sync completes
- [ ] Sandbox deleted

## Results
```
paste output here
```

## Related
- Linear: BNK-
