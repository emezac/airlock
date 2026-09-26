# Agent workers

A worker is one agent taking one backlog task from idea to a pushed branch.
It lives in the control service (`Airlock::Worker`), next to the gate that
will judge its work, and it is deliberately narrow: the model proposes edits,
and everything else is done by code that the model cannot change.

## The loop

1. Clone the target repository and branch from `main` as
   `agents/<agent>/<task>`.
2. Send Nemotron Super the task, the files listed as its context and the rules
   of the edit format.
3. Apply the reply. Edits are all-or-nothing and must stay inside the tree: no
   absolute paths, no `..`, no `.git`, no symlinks.
4. Run the task's check (for Frameline, `cargo test --release`) in the runner:
   a Token Factory Sandbox in production, a local process in development.
5. On failure, send the model a digest of the output and the current contents
   of the files it touched, then go back to step 3. The budget is six attempts.
6. On success, squash to one commit whose tree is exactly the one that passed,
   with `Airlock-Task` and `Airlock-Evidence` trailers, and push it as the
   agent.
7. If the gate refuses the push, send its reasons back to the model like a
   failed check.

The model never runs a command and never touches git. The check command comes
from the backlog and must be on the policy's evidence allowlist.

## Edit format

Replies use SEARCH/REPLACE blocks, with a summary line and one flag:

```
SUMMARY: Adds a sway motion and a test for it.
UPDATE_GOLDENS: no

src/spec.rs
<<<<<<< SEARCH
pub const MOTIONS: &[&str] = &["still", "bob", "drift", "rise"];
=======
pub const MOTIONS: &[&str] = &["still", "bob", "drift", "rise", "sway"];
>>>>>>> REPLACE
```

A SEARCH must match the file exactly once, and an empty SEARCH creates a new
file. When a SEARCH does not match, the worker names the first line that
differs and shows what the file has on that line.

`UPDATE_GOLDENS: yes` asks the worker to regenerate Frameline's golden images
with a command fixed by Airlock. In a sandbox, the images come back as a
tarball; only regular files under `tests/golden/` are accepted. Golden images
are a human-review path in the policy, so every visual change is seen by a
person.

## What the first live runs taught us

We ran the six Frameline backlog tasks with Nemotron Super on Token Factory.
Each finding below changed the design.

**JSON is the wrong envelope for code.** The first version asked for a JSON
object of edits. Rust source is full of quotes and escapes, and the model
produced JSON that parsed but did not mean what it intended: a closing quote in
the wrong place turned one edit into two keys, and a missing `replace` would
have deleted code. With SEARCH/REPLACE blocks nothing is escaped, and the next
task (FL-1) passed on the first attempt.

**Reasoning keeps the format.** With `enable_thinking: false`, Super wrote the
first block correctly and lost the `=======` divider in later ones. We sent the
same two requests with reasoning on, and both replies parsed. Workers now call
Super with reasoning, at the cost of more output tokens (about 4 000 to 8 000
per turn on these tasks).

**A shared build cache lets one agent vouch for another.** To save compile
time, parallel workers first shared one `CARGO_TARGET_DIR`. Cargo gave the test
binaries of the same crate the same name in every tree, so one agent's check
ran another agent's tests. Nothing wrong reached `main`, because the merge
queue tests the merged tree again, but that evidence was worthless. Local runs
now always build inside their own tree. This is the case for running evidence
in a clean sandbox: a result is only evidence if nothing else could have
produced it.

**Show errors, not the tail.** The runner kept the last 40 lines of output,
which for Rust were often compiler warnings, so the model never saw the errors.
The worker now sends a digest: compiler errors first, then the test failures
section with the panic messages, then the tail only if neither is present.

**Give every task the crate map.** FL-3 failed because the model did not know
that the binary imports the library as `taller_film::`; it tried `crate::`,
then `super::`, then `super::super::`. The backlog now lists `src/lib.rs` as
shared context for every task.

## Results

Across the runs so far, every backlog task was attempted at least once with
Nemotron Super (reasoning on, SEARCH/REPLACE blocks):

| Task | What it asked for | Outcome | Attempts |
|---|---|---|---|
| FL-1 | A `sway` motion, with a test and docs | Merged automatically | 1 |
| FL-2 | A new example spec | Held for human review (new golden image) | 1 |
| FL-4 | Reject duplicate scene names | Merged automatically in one run; gave up in another | 2 / 6 |
| FL-5 | Per-scene lines in `frameline check` | Merged automatically | 1 |
| FL-6 | "Make the boards nicer" (vague on purpose) | Held for human review (changed golden images) | 5 |
| FL-7 | `frameline check --json` | Merged automatically | 1 |
| FL-8 | A `warm_dusk` style (touches `src/core/`) | Held for human review (risk above threshold) | 1 |
| FL-9 | Smoke-test every element kind | Merged automatically | 1 |
| FL-3 | `frameline board --scene N` | Gave up twice; nothing pushed | 6 / 6 |

The same task can pass in one run and fail in the next: FL-4 passed on its
second attempt in one run (reasoning off) and, in a later run with reasoning
on, spent five attempts on a type inference error at the wrong line. FL-3 fails on the boundary between the
binary and the library (`crate::` against `taller_film::`), even with
`src/lib.rs` in its context; the second failure cost about 140 000 tokens.
When a worker stops at its budget it pushes nothing, so a task that cannot
pass never becomes a branch someone has to clean up.

The labels also caught something the agent did not say: FL-7 asked for a test,
and its card shows `tests: no_tests`. The change was small and low risk, so it
merged, but the gap is visible on the panel.

## The swarm

`SwarmFlow` (agentkit Flow) runs several agents at once:

1. `plan` spreads the pending tasks over the agents, round-robin. A task whose
   branch is already queued, merging, merged or waiting for review is skipped.
2. `map :work` gives each agent its share as one branch. With the ActiveJob
   dispatcher every branch is a job, so the agents work in parallel; inside a
   branch an agent does its tasks in order, in its own clone.
3. `join` waits for every branch, whether it succeeded or not.
4. `report` sums the outcomes and the tokens.

Every task is an `AgentRun` row, updated as the worker goes (waiting for the
model, running the check, the gate's answer). The panel shows the latest swarm
above the columns: one card per agent, its current task and its last step.

```sh
bin/rails airlock:swarm AGENTS=agent-ada,agent-kai,agent-lin
```

Two more lessons came from running the swarm:

**A process can die in the middle of a batch.** Restarting the service while
the merge queue was testing left a batch marked `running` and its change
marked `merging`, so neither the queue nor the planner would touch it again.
The queue now recovers under its repository lock: any batch still `running`
there was left by a dead process, so its changes go back to the queue, or are
marked merged if `main` already contains them. That is how FL-4 finally
merged.

**Polling needs to bypass the query cache.** The swarm's progress display
looked frozen while agents kept pushing: the task repeated the same query
every few seconds, and Rails answered it from the query cache. The rows were
correct in the database. The loop now reads uncached.

## Running a worker

```sh
bin/rails airlock:work TASK=FL-1 AGENT=agent-kai
```

The task needs `NEBIUS_TOKEN` for the model, plus `AIRLOCK_GIT_ROOT` (or
`AIRLOCK_PUSH_URL`), `AIRLOCK_URL` and `AIRLOCK_HOOK_SECRET` so that the push
goes through the gate. With `NEBIUS_PROJECT_ID` set and the sandbox enabled in
the policy, the check runs in a Token Factory Sandbox. Otherwise it runs
locally, which is for development only.
