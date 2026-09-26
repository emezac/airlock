# Token Factory Sandboxes

Everything Airlock executes goes through a runner. In production that runner is Token Factory Sandboxes:

- agents' checks;
- evidence re-runs;
- merge-queue CI;
- renders.

The code lives in the control service:

| File | Role |
| --- | --- |
| `control/app/services/airlock/sandboxes/client.rb` | REST client: upload a file, start an instance, poll the operation, decode output |
| `control/app/services/airlock/runners/sandbox.rb` | Runs a command on `git archive HEAD` of a tree, networking off, and brings generated files back |
| `control/app/services/airlock/collector.rb` | Extracts those files safely (regular files under allowed paths only) |
| `control/app/services/airlock/evidence_verifier.rb` | Re-runs an agent's cited command on the exact pushed commit |

## Status

- **Built:** the client and the runner, tested against a fake Sandboxes API.
- **Waiting:** our account's beta access (`GET /whoami` still reports every permission off). Until it arrives, `.env` sets `NEBIUS_SANDBOX=false`: evidence re-runs are recorded as skipped, and checks, CI and renders use the local runner. When access arrives, set it to `true`; nothing else changes.
- **Next, once access arrives:**
  1. Check `GET /whoami`.
  2. Run one command end to end.
  3. Build a prebuilt image for Frameline: Rust stable, the crate's dependencies vendored (instances have no network), ffmpeg for animatics. Then set `[sandbox] image` in the policy to that image, instead of `ubuntu:latest`.

Instances run with networking disabled, so the image must contain everything a build needs.
