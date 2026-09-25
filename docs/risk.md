# Risk scoring, human attention and batching

All three pieces live in `control/app/services/airlock/`. Each one is small enough to check by hand.

## 1. Risk score (`risk.rb`)

A noisy-OR of independent signals, each `x_i` in [0, 1] with weight `w_i`:

```text
risk = 1 - prod_i (1 - w_i * x_i)
```

| Signal | Weight | x |
| --- | --- | --- |
| Dependency manifest changed (`Cargo.toml`, `Gemfile`, ...) | 0.5 | 0 or 1 |
| Core code changed (`src/core/`) | 0.3 | 0 or 1 |
| Breadth | 0.3 | files changed / 20, capped at 1 |
| Volume | 0.3 | lines added / 400, capped at 1 |
| No test file touched | 0.15 | 0 or 1 |
| Agent failure rate | 1.0 | exponentially forgotten rate, alpha = 0.1 |
| Nemotron 3 Nano block probability | 0.4 | model output |

Every signal can only raise the risk and none saturates it alone. The gate's sensitive paths still go to a human whatever the score.

### Calibrating Nemotron 3 Nano

Asking Nano "is this risky?" with a confidence flagged every test change as risky at 0.90 to 0.95, including a one-file caption fix with a test. That signal carries no information. A rubric with anchored bands and a separate clarity score gave graded answers on the same changes (Token Factory, 25 Sep 2026, temperature 0, thinking off, about 1 s and 210 tokens per call):

| Change | Block probability | Clarity |
| --- | --- | --- |
| Fix caption overflow, with a test | 0.2 | 0.8 |
| Add pastel theme preset | 0.3 | 0.5 |
| Charge credits and add a payment crate | 0.6 | 0.7 |
| "Make it more cinematic" in core code | 0.8 | 0.2 |
| Relax the visual test threshold | 0.3 | 0.5 |

The last row is a miss: weakening a test should score high. Rules catch deleted tests and skip markers; a rule for test-configuration files is still to do. The model advises; it never decides alone.

A clarity below 0.4 sends the change to a human, because a vague goal cannot be verified.

## 2. Human attention (`attention.rb`)

Reviews are a server with capacity mu per hour; changes arrive at lambda per hour; the share with risk above the threshold t goes to humans:

```text
rho = lambda * P(risk > t) / mu        waiting time ~ 1 / (mu - lambda_h)
```

Airlock takes the largest share humans can absorb, `s = target * mu / lambda` (target 0.7), and sets t to the `1 - s` quantile of recent risk scores, never below 0.35. More load raises the bar; it never lowers it. mu starts from a prior (6 reviews per hour) and switches to measured review times after 5 reviews.

## 3. Batch size (`batch_math.rb`)

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
