# Architecture

Four layers. None of them keeps the source of truth inside an agent or a disposable VM.

| Layer | In Airlock | Runs on | Rule |
| --- | --- | --- | --- |
| Truth | Git repository with a protected `main` | Nebius AI Cloud VM (Enroute optional) | Never lives inside an agent |
| Execution | One sandbox per agent and task | Token Factory Sandboxes | Disposable; checkpoints are evidence |
| Control | Push gate, risk scoring, batched merge queue, audit | Airlock service (`control/`) | Deterministic code; models only advise |
| Policy | Rules and thresholds in `airlock.toml` | The target repository | Written by humans |

## Path of a change

1. The orchestrator assigns a task to an agent in a sandbox.
2. The agent reproduces, fixes, tests and checkpoints, then pushes with an `Airlock-Evidence` trailer.
3. `hooks/pre-receive` calls `POST /gate/push`. Rule violations are rejected with a reason.
4. Accepted changes get a risk score (Nemotron 3 Nano plus deterministic features).
5. Low risk joins the batched merge queue; high risk goes to a human with a Nemotron 3 Super summary.
6. A red batch is bisected; Nemotron 3 Ultra diagnoses the failing change.
7. A random sample of auto-merged changes is audited; the defect rate is published with a Wilson interval.
