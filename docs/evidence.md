# Reproducible evidence

An agent does not say "I tested it". It names the command it ran, in a commit trailer:

```text
Airlock-Evidence: sandbox=<id> checkpoint=<image uuid> cmd="cargo test --release"
```

Airlock then checks the claim itself.

## At the gate (deterministic)

- The trailer must have `sandbox`, `checkpoint` and `cmd`.
- `cmd` must be one of `[evidence] allowed_commands` in the policy, optionally with arguments. `cargo testify` does not match `cargo test`.
- `cmd` may not contain shell operators (`;`, `&`, `|`, `` ` ``, `$`, `<`, `>`, newlines, backslashes). One command, no chaining, no substitution.

A trailer that is well formed but false passes the gate: at push time nothing can tell. That is what the re-run is for.

## After the gate (in a sandbox)

`RouteChangeJob` re-runs `cmd` on the exact commit the agent pushed, before labelling and routing:

1. A dedicated working clone checks out the pushed branch.
2. `git archive HEAD` packs tracked files only. Build outputs and untracked files never travel.
3. The tarball is uploaded to Token Factory Sandboxes (`POST /files`) and mounted in a new instance of the policy's image (`POST /instances`), with networking disabled.
4. The instance unpacks the tree and runs `cmd`. Airlock polls `GET /operations/{id}` until the run reaches a terminal status.
5. Exit code 0 means `verified`. Anything else means `failed`: the change is rejected with "evidence did not reproduce", and the agent's failure rate goes up.

The run leaves a result image. Airlock stores its uuid as `evidence_checkpoint`, so anyone can start again from that exact state.

## The same runner for everything that executes

Agents' checks, evidence re-runs, merge-queue CI and renders all go through a runner with one contract: run a command on a committed tree and return the result.

- **`Runners::Sandbox`** is the production runner.
- **`Runners::Local`** is for development. Each tree builds in its own `target/`. A shared build cache once let one agent's check run another agent's test binaries, so its evidence proved nothing (see [workers.md](workers.md)).
- **Generated files** (regenerated golden images, rendered boards and animatics) come back from a sandbox as a base64 tarball between markers in the output. Only regular files under the allowed paths are extracted: no links, no absolute paths, no `..`.

## Current state

The client, both runners, the file collection and the evidence verifier are built and tested against a fake Sandboxes API. Our Token Factory account is waiting for Sandboxes beta access, so we run with `NEBIUS_SANDBOX=false`. That keeps the token for the Nemotron models and switches every execution to the local runner. Sandboxes are used only when all three switches are on: the policy's `[sandbox] enabled`, `NEBIUS_SANDBOX` (unset means on), and credentials (`Airlock::Sandboxes.status` says which one is off). Today:

- evidence re-runs are recorded as `skipped` ("no sandbox configured"), never run on the host;
- the agents' checks, the merge queue's CI and renders use the local runner;
- the Floor's sandbox lab says "local runner" and why.

## What forged evidence looks like

The scripted misbehaving agent has a `forged_evidence` scenario: it breaks spec validation and commits it with a trailer claiming `cargo test --release` passed.

- **With sandboxes enabled,** the re-run fails and the change is rejected before anyone looks at it.
- **Without sandboxes,** the merge queue still builds and tests the change. It marks the change failed, and the doctor explains why. In a live run Nemotron 3 Ultra named `src/spec.rs` and explained that a caption limit of 8 made the example specs invalid.

Either way, the forged change never reaches `main`. The difference is how much work and attention it wastes first.

The command an agent cites is only ever re-run in a sandbox; without one, the check is recorded as skipped. The commands that run locally in development come from the backlog and the policy, not from the agent.
