# Deploying Airlock

Airlock runs on one VM with Docker Compose. There are three containers:

- **Postgres 16.** It holds the app's tables, the Solid Queue jobs and agentkit's audit log, all in one database.
- **The control service.** Puma serves the three tabs and runs Solid Queue in-process. The same container also holds the bare git repositories, with their pre-receive hook, and the toolchain the local runner needs (git, Rust, ffmpeg).
- **Caddy.** It sits in front and gets a certificate automatically.

Checks, merge-queue CI and renders run on the **local runner**, inside the app container. Token Factory Sandboxes are switched off with `NEBIUS_SANDBOX=false`, so evidence re-runs are recorded as skipped. Sandboxes turn on once access is granted: see [Sandboxes and the container runner](#sandboxes-and-the-container-runner).

Status: not deployed yet, and the image has not been built on a real machine. The launch plan is [../docs/launch-plan.md](../docs/launch-plan.md).

## Where

The public demo is planned on **Oracle Cloud Always Free**: an Arm VM with 2 OCPUs and 12 GB of memory. [oracle.md](oracle.md) covers it step by step, including the free tier's limits and idle reclamation.

Any Ubuntu VM with Docker works the same way. The Nebius AI Cloud steps below are kept for reference: the hackathon requires Token Factory or AI Cloud, and Airlock meets that through Token Factory, so a Nebius VM is not required.

## On Nebius AI Cloud (or any Ubuntu VM)

1. **Create a VM.** Use Ubuntu 22.04 or 24.04, with at least 4 vCPUs, 16 GB of RAM and a 100 GB disk. Several agents compiling Rust at once is the heavy part. The VM needs a public IP, and ports 22, 80 and 443 open.
2. **Install and start Airlock.** Either:
   - paste [`cloud-init.yaml`](cloud-init.yaml) as the user data when you create the VM; or
   - SSH in and run the bootstrap script:

   ```sh
   curl -fsSL https://raw.githubusercontent.com/emezac/airlock/main/deploy/bootstrap.sh -o bootstrap.sh
   bash bootstrap.sh
   ```

   Run this way, the script asks for the Token Factory key without echoing it. It then:
   - installs Docker;
   - adds swap on small machines;
   - clones the repository to `~/airlock`;
   - writes `deploy/.env` with fresh secrets;
   - builds the image and starts everything.
3. **Add the key if needed.** If you used cloud-init, or skipped the key, put `NEBIUS_TOKEN` and `NEBIUS_PROJECT_ID` in `~/airlock/deploy/.env`, then run `deploy/airlock up`.
4. **Open the service.** The address is `https://<ip-with-dashes>.sslip.io`. sslip.io resolves that name to the VM's IP, so there is no DNS to set up. The reviewer token is printed once at the end of the bootstrap, and it is also stored in `deploy/.env`.

The first build takes several minutes: it compiles the gems, installs Rust and downloads Frameline's crates.

## Running the demo there

On the first start, the demo repository (Frameline) is created with its hook. After that, from `~/airlock`:

```sh
deploy/airlock rails airlock:swarm AGENTS=agent-ada,agent-kai,agent-lin
deploy/airlock rails airlock:rogue
deploy/airlock rails airlock:demo:reset      # forget the changes and runs, keep the repository
deploy/airlock logs
```

The swarm runs as jobs, so closing the SSH session does not stop it. The command only reports progress.

## Configuration

`deploy/.env` holds the configuration. [`env.example`](env.example) documents each variable.

| Variable | Default | Purpose |
| --- | --- | --- |
| `AIRLOCK_HOST` | — | The public name for Caddy's certificate |
| `NEBIUS_SANDBOX` | `false` | Use Token Factory Sandboxes for every run |
| `AIRLOCK_SEED_DEMO` | `1` | Create the demo repository on the first start |
| `JOB_THREADS` | `8` | Solid Queue worker threads: one per parallel agent, plus routing, the merge queue and renders |
| `PG_SHARED_BUFFERS`, `PG_EFFECTIVE_CACHE_SIZE` | `256MB`, `1GB` | Postgres memory; `3GB` and `6GB` on a 12 GB VM |
| `AIRLOCK_FORCE_SSL` | `true` | `false` serves plain HTTP. Use it only for a private network or a first smoke test without Caddy. |

A recurring task (`config/recurring.yml`) runs `MergeQueueSweepJob` every minute. That keeps the merge queue moving after a restart.

The gate (`/gate/*`) and agentkit's mount point (`/agentkit`, whose console is disabled anyway) are not published: Caddy answers 404. The pre-receive hook calls the gate on `127.0.0.1` inside the app container.

All state lives in two volumes: `pg` holds the database, and `data` holds the repositories, working clones and renders. `docker compose down` keeps both, and `down -v` deletes them.

## Security of the local runner

`cargo test` runs code that agents wrote: tests and build scripts. On the local runner, that code runs inside the app container as an unprivileged user, and it gets a minimal environment: `PATH`, `HOME` and the Rust paths. It never sees `NEBIUS_TOKEN`, the database URL or the hook secret, and a test checks this.

That is not isolation. The code can still reach the network and read files the app user can read. Use a VM dedicated to Airlock, and treat the local runner as a stopgap until one of the runners below is in place.

## Sandboxes and the container runner

- **Token Factory Sandboxes.** When access is granted, set `NEBIUS_SANDBOX=true` and run `deploy/airlock up`. Agents' checks, evidence re-runs, merge-queue CI and renders then run in sandboxes, and nothing agent-written runs on the VM.
- **A container runner** (planned) runs each check in a throwaway `docker run --network none` container with a prebuilt Rust image. It implements the same runner contract, so nothing else changes.

## Notes

- The database is created by the Postgres container (`POSTGRES_DB`), not by `db:prepare`. agentkit's engine checks its tables while the app boots, and that check fails if the database does not exist yet.
- Image and asset URLs are cached for a year, because they name a full commit id or a digest.
