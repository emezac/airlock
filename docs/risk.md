# Risk scoring, human attention and batching

Everything here lives in `control/app/services/airlock/`, and each piece is small enough to check by hand.

## 1. Labels (`labels.rb`, `labeler.rb`)

Every change gets labels from closed vocabularies. A value outside its vocabulary is rejected, whoever proposes it.

| Category | Values | Owner |
| --- | --- | --- |
| `area` | from `[labels.area]` in the policy, plus `multiple`, `other` | rule (longest path prefix) |
| `blast_radius` | `single_area`, `multi_area`, `core` | rule |
| `test_signal` | `tests_added`, `tests_modified`, `no_tests`, `tests_weakened` | rule |
| `dependency` | `none`, `added_or_changed` | rule |
| `change_type` | `fix`, `feature`, `refactor`, `deps`, `config`, `test`, `docs`, `revert` | Nemotron 3 Nano |
| `goal_clarity` | `clear`, `partial`, `vague` | Nemotron 3 Nano |
| `safety_flag` | `none`, `touches_money`, `touches_auth`, `touches_data`, `weakens_safeguard` | Nemotron 3 Nano |

Each label records its source: `rule`, `model` or `override`. The effective label per category is **override > rule > model**. Humans correct labels through `POST /reviews/:id/label`; the value is validated, every correction is appended to `label_overrides` and the agentkit audit log, and the change is re-routed with the new labels.

**Why rules own four categories.** In a test on Token Factory (25 Sep 2026, temperature 0, thinking off), asking Nano for all categories at once gave different answers across runs for 3 of 5 changes, called a change with a test file `no_tests`, and missed a weakened test. Anything computable from paths and diffs is now a rule.

**Why the model votes.** With only the three semantic categories and tighter definitions, 5 runs per change agreed 100 % on `change_type` and 96 % on `goal_clarity` and `safety_flag`, and "relax the visual threshold" became `weakens_safeguard`. Airlock asks 3 times and keeps the majority; the agreeing share (1/3, 2/3, 3/3) is the label's confidence. Each call costs about 150 tokens.

**Routing is a table, not a judgement.** `[routing] human_when` lists `category:value` pairs that always go to a human, for example `safety_flag:touches_money`, `test_signal:tests_weakened`, `goal_clarity:vague`, `dependency:added_or_changed`.

## 2. Risk score (`risk.rb`)

A noisy-OR of independent signals, each `x_i` in [0, 1] with weight `w_i`:

```text
risk = 1 - prod_i (1 - w_i * x_i)
```

| Signal | Weight | x |
| --- | --- | --- |
| `dependency = added_or_changed` | 0.5 | 1 |
| `blast_radius` | 0.3 | `core` 1, `multi_area` 0.5 |
| `test_signal` | 0.6 | `tests_weakened` 1, `no_tests` 0.25 |
| `goal_clarity` | 0.3 | `vague` 1, `partial` 0.5 |
| `safety_flag` other than `none` | 0.5 | 1 |
| Breadth | 0.3 | files changed / 20, capped at 1 |
| Volume | 0.3 | lines added / 400, capped at 1 |
| Agent failure rate | 1.0 | exponentially forgotten rate, alpha = 0.1 |

Every signal can only raise the risk and none saturates it alone. Because label signals read the effective labels, a human correction changes the score deterministically. Unknown labels contribute nothing.

## 3. Human attention (`attention.rb`)

Reviews are a server with capacity mu per hour; changes arrive at lambda per hour; the share with risk above the threshold t goes to humans:

```text
rho = lambda * P(risk > t) / mu        waiting time ~ 1 / (mu - lambda_h)
```

Airlock takes the largest share humans can absorb, `s = target * mu / lambda` (target 0.7), and sets t to the `1 - s` quantile of recent risk scores, never below 0.35. More load raises the bar; it never lowers it. mu starts from a prior (6 reviews per hour) and switches to measured review times after 5 reviews.

## 4. Batch size (`batch_math.rb`)

With each change failing independently with probability f (q = 1 - f), the expected CI runs of test-and-bisect on n changes are exactly:

```text
R(1) = 1
R(n) = 1 + R(a) + R(b) - 2 q^n,   a = floor(n/2), b = n - a
```

A failing batch splits in halves; each half runs knowing its parent failed, and `E[R(half) | parent failed] = (R(half) - q^n) / (1 - q^n)`. Multiplying by `P(parent failed) = 1 - q^n` gives the recursion. It matches a 20,000-batch Monte Carlo simulation within 0.01.

| Failure rate f | Best batch k | CI runs per change | Saving vs one by one |
| --- | --- | --- | --- |
| 0.05 | 8 | 0.40 | 60 % |
| 0.10 | 4 | 0.61 | 39 % |
| 0.15 | 4 | 0.77 | 23 % |
| 0.25 | 2 | 0.94 | 6 % |

After bisection, the surviving changes are tested together once more, because two changes can pass alone and fail together.

## 5. Running the queue safely (`merge_queue.rb`)

The math above assumes the queue runs one batch at a time on an up-to-date `main`. Four rules keep that true:

- **One batch per repository.** A Postgres advisory lock guards each repository. The lock and the unlock bypass Rails' query cache: jobs run with the cache on, and a cached "unlock" never ran, which once left a connection holding the lock and the queue stalled.
- **Check what landed.** After pushing a green batch (through the same gate as everyone else), the queue fetches `main` and checks that it contains every change it merged.
- **Recover from a dead process.** A batch still marked `running` while the queue holds the lock was left by a process that died. Its changes go back to the queue, or are marked merged if `main` already contains them.
- **Never wait for the next push.** `MergeQueueSweepJob` wakes the queue for every repository with approved or queued changes when the server boots. In production it should also run as a recurring task.

## 6. After a failure (`diagnosis.rb`)

A change that fails in the queue gets a diagnosis, with the same split as the labels.

The rules decide the category from what the queue recorded:

| Category | How the rules decide it |
| --- | --- |
| `merge_conflict` | git could not merge the branch |
| `interaction` | the change passed in one CI run and failed only when combined with others |
| `compile_error` | `error[E…]` or `could not compile` in the output |
| `visual_regression` | the visual test reports changed pixels |
| `test_failure` | a panic or a failed test result |
| `infrastructure` | a timeout, or the machine killed the job |

Nemotron 3 Ultra adds three things:

- the culprit files, kept only if the change touched them;
- a short explanation;
- the next step: `agent_retry`, `human` or `drop`.

It may name the category only when no rule matched. A person's rejection is recorded the same way, as `rejected_by_reviewer` with their reason.

The swarm's planner reads the result:

- `agent_retry` tasks are offered again, with the diagnosis or the reviewer's reason in the agent's instructions;
- `human` and `drop` tasks wait for a person.
