# Reproducible evidence

An agent does not say "I tested it". It names the command it ran, in a commit trailer:

```text
Airlock-Evidence: sandbox=<id> checkpoint=<image uuid> cmd="cargo test -q visual"
```

Airlock then checks the claim itself.

## At the gate (deterministic)

- The trailer must have `sandbox`, `checkpoint` and `cmd`.
- `cmd` must be one of `[evidence] allowed_commands` in the policy, optionally with arguments. `cargo testify` does not match `cargo test`.
- `cmd` may not contain shell operators (`;`, `&`, `|`, `` ` ``, `$`, `<`, `>`, newlines, backslashes). One command, no chaining, no substitution.

## After the gate (in a sandbox)

`RouteChangeJob` re-runs `cmd` on the exact commit the agent pushed, before labelling and routing:

1. A dedicated working clone checks out the pushed branch.
2. `git archive HEAD` packs tracked files only. Build outputs and untracked files never travel.
3. The tarball is uploaded to Token Factory Sandboxes (`POST /files`) and mounted in a new instance of the policy's base image (`POST /instances`), with networking disabled.
4. The instance unpacks the tree and runs `cmd`. Airlock polls `GET /operations/{id}` until a terminal status.
5. Exit code 0 means `verified`. Anything else means `failed`: the change is rejected with "evidence did not reproduce", and the agent's failure rate goes up.

The run leaves a result image. Airlock stores its uuid as `evidence_checkpoint`, so anyone can start from that exact state again.

Agent-supplied commands only ever run in a sandbox. Without `NEBIUS_TOKEN` and `NEBIUS_PROJECT_ID`, the check is recorded as `skipped` and never runs on the host. CI commands come from the policy, not from agents, so the merge queue may fall back to a local runner.
