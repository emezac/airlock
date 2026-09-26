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

## Results of the run

| Task | What it asked for | Outcome | Attempts |
|---|---|---|---|
| FL-1 | A `sway` motion, with a test and docs | Merged automatically | 1 |
| FL-2 | A new example spec | Accepted; held for human review (new golden image) | 1 |
| FL-5 | Per-scene lines in `frameline check` | Merged automatically | 1 |
| FL-6 | "Make the boards nicer" (vague on purpose) | Accepted; held for human review (changed golden images) | 5 |
| FL-3 | `frameline board --scene N` | Gave up; nothing pushed | 6 |
| FL-4 | Reject duplicate scene names | Gave up; nothing pushed | 6 |

FL-4 is a limitation of the model on this task: `HashMap::new()` without a
type, and rustc points at the line that uses the value rather than the
declaration. FL-3 ran before the shared context existed. In both cases the
worker stopped at its budget and pushed nothing, which is the behaviour we
want: a task that cannot pass does not become a branch that someone has to
clean up.

## Running a worker

```sh
bin/rails airlock:work TASK=FL-1 AGENT=agent-kai
```

The task needs `NEBIUS_TOKEN` for the model, plus `AIRLOCK_GIT_ROOT` (or
`AIRLOCK_PUSH_URL`), `AIRLOCK_URL` and `AIRLOCK_HOOK_SECRET` so that the push
goes through the gate. With `NEBIUS_PROJECT_ID` set and the sandbox enabled in
the policy, the check runs in a Token Factory Sandbox. Otherwise it runs
locally, which is for development only.
