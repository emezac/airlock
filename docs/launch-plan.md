# Launch plan

Where the project stands, what the hackathon requires, and what is left before submitting, in order. Written on 26 Sep 2026, when work paused until the launch.

## Dates

| What | When |
| --- | --- |
| Submission deadline | **30 Oct 2026, 10:00 PT** (11:00 in Mexico City, 17:00 UTC) |
| Judging | 1 to 15 Dec 2026. The demo must stay up and working all that time. |
| Winners | 11 Jan 2027 |

**Launch target: the week of 19 Oct**, and no later than 23 Oct. That leaves a week for a failed first build, Oracle capacity errors or a certificate problem. The Devpost entry can be saved early and edited until the deadline.

## What the rules ask, and where we stand

Read on 26 Sep 2026 from the [rules](https://nebiusglobalaihackathon.devpost.com/rules) and the [overview](https://nebiusglobalaihackathon.devpost.com/). Check them again before submitting.

| Requirement | Status |
| --- | --- |
| Run on "Nebius Token Factory or Nebius AI Cloud", with calls to the Token Factory inference API at run time | Done. Workers use Nemotron 3 Super, labels use Nano and diagnosis uses Ultra, all through Token Factory. |
| Use at least one NVIDIA open source model | Done: three Nemotron 3 models |
| Public repository with an open source license and a README with setup instructions | Done: github.com/emezac/airlock, Apache 2.0 |
| "A URL to a working demo, hosted application, or test build" | **Pending:** Oracle Cloud Always Free ([../deploy/oracle.md](../deploy/oracle.md)) |
| Video under 3 minutes, public on YouTube, with audio explaining how Nebius and NVIDIA models are used | **Pending** (outline below) |
| Project description and feedback on the tools used | **Pending** (Devpost form) |

**Track: Coding and Agentic Engineering.** The overview describes it as "agents that write, run, and test code in Token Factory Sandboxes". The Sandboxes integration is built and tested against a fake API, but our account is still waiting for beta access. Checks therefore run on the local runner (`NEBIUS_SANDBOX=false`). The submission has to say this plainly: what is integrated, what runs today, and the one switch that moves execution to Sandboxes.

**Judging criteria** (equal weight): technological implementation, design ("a complete, coherent product experience"), potential impact, and quality of the idea.

## Built and verified (26 Sep 2026)

- **Gate, workers and queue.** The push gate and hook, labels and risk, the batched merge queue, the agent workers and the swarm, failure diagnosis, the misbehaving agent, and the three tabs (Control room, Floor, Frameline). There are 116 tests.
- **Live runs with Nemotron on Token Factory.** Seven of ten backlog tasks are merged, one waits for review, one was rejected by a reviewer and one was given up. The details are in [workers.md](workers.md).
- **Production configuration.** It was run on a test machine without Docker: Solid Queue inside Puma, the recurring merge-queue sweep, the misbehaving agent's ten scenarios, and the swarm's dispatch through jobs. The Compose stack and the image have **not** been built yet.
- **Last commits:** 5906d96 (deployment), 21b36c6 (ignore `.nebius/`), and the commit that adds this plan.

## What is left, in order

### 1. Deploy on Oracle (about half a day, plus build time)

Follow [../deploy/oracle.md](../deploy/oracle.md): account and home region, VCN and ports, an A1 instance with 2 OCPUs and 12 GB, the iptables rules, `bootstrap.sh`, then `JOB_THREADS=4` and the Postgres memory settings.

If the first build fails, the most likely causes are native gems or rustup on aarch64. These are guesses: none of it has been tried on Arm. Keep the error and fix it in the repository, not only on the VM.

### 2. Record a real session on the deployed instance

The Floor's Replay plays sessions stored in the **production** database, so the swarm has to run once on the VM:

```sh
deploy/airlock rails airlock:demo:reset
deploy/airlock rails airlock:swarm AGENTS=agent-ada,agent-kai ONLY=FL-1,FL-5,FL-7,FL-10
deploy/airlock rails airlock:rogue
```

Then, on the Frameline tab, sign in with the reviewer token. Approve FL-10 after comparing the animatics, and reject another visual change with a reason.

- **Cost:** only the swarm spends tokens. The tasks listed took one attempt each in earlier runs, with Super writing about 4 000 to 8 000 tokens per turn (see [workers.md](workers.md)). The misbehaving agent only calls Nano, for labels, and Ultra, for one diagnosis.
- **Speed:** two agents on 2 cores build Rust slowly. The replay compresses the waits.

### 3. Decide what judges can do on the public site

On the public site, **nobody can start agents or spend tokens** today:
- the swarm and the misbehaving agent are rake tasks, run over SSH;
- the gate and agentkit's mount point are closed at Caddy;
- approving or rejecting needs the reviewer token.

Judges can see everything: the three tabs, the Floor's live view, Replay and Explain, and the renders.

Two small additions would make the site clearer. Both are **planned, not built**:

- **A banner** saying it is a public demo: what judges can do, and that the agents ran with Nemotron on Token Factory at the time shown.
- **Hands-on for judges (optional).** Put the reviewer token in the entry's testing instructions, so judges can approve or reject what waits for review. To show the gate at work, add a rate-limited "Run the misbehaving agent" button for signed-in reviewers. It is deterministic and costs almost no tokens.

After a judge approves or rejects, `airlock:demo:reset` followed by step 2 brings the demo back to a known state.

### 4. The video (under 3 minutes)

Record from the deployed site. Use Replay at 20× or 60× for the swarm, so real events play at a watchable speed.

| Time | Show | Say |
| --- | --- | --- |
| 0:00–0:20 | The Frameline tab: the brief | Agent swarms push faster than people can review. Airlock is the airlock between them and `main`. |
| 0:20–1:05 | Floor, Replay: agents think (Nemotron 3 Super on Token Factory), type, go to the lab and carry cards to the gate | Every agent works on its own branch and must bring evidence. The gate is deterministic: no model decides. |
| 1:05–1:35 | The misbehaving agent: eight pushes bounce off the gate, each with its reason. Forged evidence dies in the merge queue, and the doctor (Nemotron 3 Ultra) names the file | This is what the gate stops, and what it cannot see at push time. |
| 1:35–2:05 | Labels (Nemotron 3 Nano, three votes), the risk score, then the review desk and a side-by-side animatic, approved | Human attention goes where the risk is. |
| 2:05–2:35 | The merge queue batches, `main` is re-rendered, and the before and after appear on the Frameline tab | The result against the brief. |
| 2:35–2:55 | Floor, Explain: the nine components | Built on Nebius Token Factory with three Nemotron 3 models. Sandboxes are integrated, and turned on with one switch once access is granted. |

`docs/demo.md` has the full recording walkthrough. Never use the third-party audio from `taller_film/assets/audio` in the video.

### 5. The Devpost entry

Fill in:
- the project description (from README.md);
- the demo URL (the sslip.io or DuckDNS address);
- the repository URL;
- the YouTube link;
- the track (Coding and Agentic Engineering);
- which Nebius products and NVIDIA models are used, and where in the code;
- the testing instructions (what judges can do, and the reviewer token if step 3 is done);
- feedback on the tools used.

For the feedback, notes from the work so far:
- **Nemotron 3 Super.** It keeps the SEARCH/REPLACE format with reasoning on, and loses it with reasoning off.
- **Nemotron 3 Ultra.** Its diagnosis was fast and precise: about 1.5 s and 291 tokens.
- **Sandboxes.** Beta access was still pending near the deadline.

### 6. If Sandboxes access arrives

1. Set `NEBIUS_SANDBOX=true` and `NEBIUS_PROJECT_ID` in `deploy/.env`.
2. Set the policy's `[sandbox] image` to an image with Rust (see [../sandbox/README.md](../sandbox/README.md)).
3. Run `deploy/airlock up`.
4. Run `airlock:rogue SCENARIO=forged_evidence`. It should now be rejected at the evidence re-run, before the merge queue. The Floor's lab should say "Token Factory Sandbox".
5. Update [evidence.md](evidence.md) with what the real API did.

### 7. Optional, if time allows

- **A container runner.** Run each check in a throwaway `docker run --network none` container, with a prebuilt Rust image. The runner contract is the same. Isolation on the VM is the point.
- **An upstream fix in agentkit.** Its engine checks its tables while the app boots, and crashes if the database does not exist yet. Deployments work around it by having Postgres create the database.

## Risks

| Risk | Mitigation |
| --- | --- |
| Oracle has no Arm capacity in the home region | Try other availability domains, and retry over several days. Start early. |
| Oracle reclaims the instance as idle before or during judging | Postgres buffers keep memory above 20%. Check the memory metric, keep a boot-volume backup, and check the site on 30 Nov and during judging. |
| The first image build fails on aarch64 | Allow a week for fixing it. Keep the fixes in the repository. |
| The Let's Encrypt certificate fails for the sslip.io name | Use a DuckDNS name or your own domain (`AIRLOCK_HOST`). |
| Token Factory spend | The public site cannot spend tokens. Only swarm runs started over SSH do. |
| Sandboxes still unavailable | Say so plainly. The switch and the fake-API tests show the integration. |

## Picking the work up again

- **Local development:** [demo.md](demo.md) and [../control/README.md](../control/README.md).
- **Secrets:** they live in `.env` at the repository root and in `deploy/.env` on the VM. Neither is committed.
- **The Nebius service-account key** in `.nebius/` was created for a Nebius VM that is no longer planned. It is not uploaded anywhere, and it can be deleted.
