# Airlock

**A git gateway for agent swarms: nothing reaches `main` without reproducible evidence, and only risky changes reach a human.**

Airlock is our entry for the Nebius x NVIDIA Global AI Hackathon (Coding and Agentic Engineering track).

> Status: scaffold. The components below are being built during the hackathon submission period.

## The problem

When ten or a hundred coding agents work in parallel, human review becomes the bottleneck. The pull-request queue grows without bound, or reviews degrade into approvals without reading. If an agent cannot verify its own work, you are still the verification system.

## How Airlock works

1. **Evidence before trust.** Each agent works in a Nebius Token Factory Sandbox, reproduces the problem, runs the real flow and attaches evidence to its commit as a trailer that points to a sandbox checkpoint.
2. **A deterministic gate, not an LLM gatekeeper.** A `pre-receive` hook calls the Airlock service, which applies rules written as code: allowed paths per agent, required evidence, secrets, tests. Models advise; code decides.
3. **Human attention as a budgeted resource.** Airlock scores the risk of each change and adapts the threshold so the human review queue stays stable (utilization ρ ≤ 0.7). Everything else merges in batches, with sampled audits.

## Layout

| Path | Contents |
| --- | --- |
| `control/` | Airlock service: gate API, risk scoring, merge queue, dashboard (Rails + agentkit-rails) |
| `hooks/` | Git server hooks; `pre-receive` calls the gate |
| `sandbox/` | OCI image and client for Token Factory Sandboxes |
| `demo/frameline/` | Frameline, the demo target: storyboards and animatics from YAML, with visual regression tests |
| `docs/` | Architecture and design notes |
| `airlock.toml.example` | Example policy |

The demo target, **Frameline** (a storyboard and animatic maker built on the `taller_film` Rust renderer), is kept here as source. For a run it is pushed as its own repository behind the gate, and agents change it only through Airlock. See [demo/frameline/README.md](demo/frameline/README.md).

## Models and services

- NVIDIA Nemotron 3 Super: worker agents and human-facing summaries
- NVIDIA Nemotron 3 Nano: risk classification on every push
- NVIDIA Nemotron 3 Ultra: diagnosis of failed merge batches
- NVIDIA Nemotron-Nano-V2-12b and Cosmos3-Super-Reasoner: visual verification of rendered storyboards
- Nebius Token Factory (OpenAI-compatible API) and Token Factory Sandboxes

## Setup

The push gate is ready. See [control/README.md](control/README.md) to run the service and install `hooks/pre-receive` on a bare repository.

```sh
cd control && bundle install && bin/rails db:create db:migrate && bin/rails test
```

The merge queue, labels, risk scoring and human-attention control are described in [docs/risk.md](docs/risk.md); evidence verification in Token Factory Sandboxes in [docs/evidence.md](docs/evidence.md); agent workers and what the first live runs with Nemotron Super taught us in [docs/workers.md](docs/workers.md); how to run the demo, the misbehaving agent and failure diagnosis in [docs/demo.md](docs/demo.md). The live panel is served at `/` (HTML) and `/dashboard.json`; it is read-only and shows no push contents or secrets.

## License

Apache License 2.0. See [LICENSE](LICENSE).
