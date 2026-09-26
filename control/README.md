# Airlock control service

Rails 8 service built on [agentkit-rails](https://github.com/emezac/agentkit-rails-0.2). It runs:

- the push gate that `../hooks/pre-receive` calls;
- labelling, risk and routing;
- the merge queue;
- the agent workers and the swarm;
- failure diagnosis and renders;
- the three tabs (Control room, Floor, Frameline).

Decisions are recorded in its tables and in agentkit's signed, hash-chained audit log. [../docs/architecture.md](../docs/architecture.md) explains how the pieces fit.

## Requirements

- Ruby 3.2 or later (developed with 3.3.6)
- PostgreSQL 14 or later
- git, and for Frameline: Rust (cargo) and ffmpeg on the runner

## Setup

```sh
cd control
bundle install
bin/rails db:prepare
bin/rails test
AIRLOCK_HOOK_SECRET=change-me bin/rails server -p 3000
```

To run the demo end to end, see [../docs/demo.md](../docs/demo.md). To deploy on a VM, see [../deploy/README.md](../deploy/README.md).

In production, jobs run on Solid Queue in the primary database, inside Puma (`SOLID_QUEUE_IN_PUMA=1`).

## Environment

| Variable | Purpose |
| --- | --- |
| `AIRLOCK_HOOK_SECRET` | Shared secret between the git hook and the gate. Required; without it every call is refused. |
| `AIRLOCK_URL` | The gate's URL, exported to the hook. Workers and the merge queue push through it too. |
| `AIRLOCK_GIT_ROOT` | Directory holding the bare repositories (`<repo>.git`). |
| `AIRLOCK_PUSH_URL` | Optional: where workers push, instead of `AIRLOCK_GIT_ROOT/<repo>.git`. |
| `AIRLOCK_WORK_ROOT` | Working clones (merge queue, workers, evidence, renders) and stored renders. Default: `tmp/workspaces`. |
| `AIRLOCK_POLICY` | Path to the policy. Default: `config/airlock/airlock.toml`. |
| `AIRLOCK_REVIEW_TOKEN` | Token for reviewers: bearer token for the reviews API, and the sign-in token on the Frameline tab. |
| `AIRLOCK_BRIEF`, `AIRLOCK_BACKLOG` | The brief and the backlog. Default: `../demo/brief.yml` and `../demo/backlog.yml`. |
| `NEBIUS_TOKEN` | Nebius Token Factory API key, for the Nemotron models and for Sandboxes. |
| `NEBIUS_PROJECT_ID` | Nebius project id, sent as the `Project` header to Sandboxes. Without it, evidence re-runs are recorded as skipped and CI runs locally. |
| `NEBIUS_SANDBOX` | Switch for Token Factory Sandboxes: `false` keeps the token for the models but runs everything on the local runner and skips evidence re-runs. Unset means on. |
| `NEBIUS_API_BASE`, `NEBIUS_SANDBOXES_BASE` | Override the Token Factory endpoints. |
| `AIRLOCK_RELAY_OUT`, `AIRLOCK_RELAY_IN` | Development only: relay model calls through files to a machine that holds the token (see [../docs/workers.md](../docs/workers.md)). |
| `AGENTKIT_AUDIT_KEY` | Audit signing key. Required in production. |
| `DATABASE_URL` | Production database (one database: app tables, Solid Queue, audit log). |
| `SOLID_QUEUE_IN_PUMA` | Run Solid Queue inside Puma (the deployment does). |
| `JOB_THREADS`, `DB_POOL` | Solid Queue worker threads (default 8) and database pool (default 20). |
| `AIRLOCK_FORCE_SSL` | `false` to serve plain HTTP in production (behind no TLS proxy). Default `true`. |
| `AIRLOCK_SEED_DEMO` | `1` makes the container create the demo repository on first start. |
| `SECRET_KEY_BASE` | Rails secret. Required in production. |

## Install the hook on a bare repository

`bin/rails airlock:demo:seed` does this for the demo repository. By hand:

```sh
cp ../hooks/pre-receive /srv/git/frameline.git/hooks/pre-receive
# The git server must export these to the hook:
#   AIRLOCK_URL=http://127.0.0.1:3000  AIRLOCK_HOOK_SECRET=change-me
# and set REMOTE_USER to the authenticated pusher (agent-<name> for agents).
```

The hook fails closed: if the gate cannot be reached, the push is refused.

## Gate rules (`app/services/airlock/gate.rb`)

1. Protected refs (`main`) can only be updated by the merge queue identity.
2. Agents (`agent-*`) can only push to `refs/heads/agents/<their name>/*`.
3. Agents cannot touch forbidden paths.
4. No added secrets.
5. No deleted test files or new skip markers.
6. Every agent commit carries `Airlock-Evidence: sandbox=<id> checkpoint=<id> cmd="<command>"`. The command must be on the policy's allowlist and have no shell operators. See [../docs/evidence.md](../docs/evidence.md).
7. Sensitive paths are accepted but routed to human review.

The gate is deterministic: the same push report always gets the same decision, and no model is consulted.

A push that rewrites a branch (not a fast-forward) is judged like a new branch: its commits are the ones not reachable from any other branch, and its files are diffed against the merge-base with `main`.

## Endpoints

| Endpoint | Purpose |
| --- | --- |
| `POST /gate/push` | The hook's report in, the decision out (`X-Airlock-Hook-Secret` required) |
| `GET /` and `GET /dashboard` | Control room (`.json` for the same snapshot) |
| `GET /floor` | The Floor; `GET /floor/state`, `/floor/events?after=&from=&to=`, `/floor/sessions` feed it |
| `GET /frameline` | Brief, tasks, boards, renders and the visual review (`.json` too) |
| `GET /frameline/golden/:sha/:name` | A golden image from git, by full commit id |
| `GET /frameline/renders/:sha/:name` | A rendered board (`.png`) or animatic (`.mp4`) of a commit |
| `GET /frameline/reviewer/new`, `POST /frameline/reviewer`, `DELETE /frameline/reviewer` | Reviewer sign-in (name and review token), rate limited |
| `POST /frameline/reviews/:id/approve`, `/reject` | Decisions from the browser (signed-in reviewer, CSRF token) |
| `GET /reviews`, `POST /reviews/:id/approve`, `/reject`, `/label` | Reviews API (`Authorization: Bearer $AIRLOCK_REVIEW_TOKEN`) |

## Tasks

| Task | What it does |
| --- | --- |
| `bin/rails airlock:demo:seed` | Creates the demo repository from `../demo/frameline` and installs the hook |
| `bin/rails airlock:demo:reset [FORCE=1]` | Forgets the demo repository's changes, batches, decisions and runs; `FORCE=1` also deletes the repository (refuses in production) |
| `bin/rails airlock:work TASK=FL-1 AGENT=agent-kai` | One agent, one backlog task |
| `bin/rails airlock:swarm AGENTS=agent-ada,agent-kai [ONLY=FL-3,FL-4]` | The swarm over the backlog, one parallel job per agent |
| `bin/rails airlock:rogue [SCENARIO=all\|name] [AGENT=agent-rex]` | The scripted misbehaving agent (see [../docs/demo.md](../docs/demo.md)) |

## Jobs

| Job | When it runs |
| --- | --- |
| `RouteChangeJob` | After an accepted push: evidence re-run, labels, risk, routing |
| `MergeQueueJob` | When a change is queued or approved; one batch at a time per repository |
| `MergeQueueSweepJob` | When Puma boots, and every minute in production (`config/recurring.yml`) |
| `DiagnoseFailureJob` | For each change that fails in the merge queue |
| `RenderJob` | After a green batch, and for a visual change sent to review |

## Tests

```sh
bin/rails test     # 116 tests
```

Tests use real git repositories, and some run the real hook against a local Puma server. They never call Token Factory: model clients are replaced by scripted ones.
