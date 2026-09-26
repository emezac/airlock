# Rules for agents working on Airlock

- Read the affected code before you name a cause. Mark untested explanations as hypotheses.
- Reproduce a bug before you change code. Repeat the same steps after the change.
- Exercise the real interface (HTTP API, git push, CLI). A passing build does not prove behavior.
- Keep changes in scope. Do not refactor unrelated code.
- Never commit secrets, API keys or tokens. Configuration reads them from the environment.
- The gate is deterministic code. Model output may inform a decision; it never makes one.
- When the same review comment appears twice, turn it into a test, a lint rule or a policy rule.
- Documentation describes what exists. Planned or unbuilt work is labelled as such, never described as a feature.
- Finish with: what changed, which checks you ran, where the evidence is, and what is still unverified.
