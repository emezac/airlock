# Deploying on Oracle Cloud Always Free

This is the plan for the public demo: one Arm VM on Oracle Cloud's Always Free tier, set up with the files in this folder.

**Status (26 Sep 2026): not deployed yet.**
- The Compose stack was not built or run on a real machine: Docker Hub was blocked on the machine where it was written.
- The production configuration was run on a test machine without Docker: the gate, Solid Queue, the merge queue, the swarm dispatch and all three tabs.
- The launch checklist is [../docs/launch-plan.md](../docs/launch-plan.md).

Oracle's limits and policies below were read on 26 Sep 2026 from [Always Free Resources](https://docs.oracle.com/en-us/iaas/Content/FreeTier/freetier_topic-Always_Free_Resources.htm). Check them again before launching.

## Why Oracle, and why not a Nebius VM

The hackathon rules ask that the project "run on either Nebius Token Factory or Nebius AI Cloud". Airlock calls Token Factory at run time: Nemotron 3 Super for the agents, Nano for the labels and Ultra for the diagnosis. That meets the requirement.

The rules also ask for "a URL to a working demo, hosted application, or test build", but not where it is hosted. A Nebius AI Cloud VM of the right size was too expensive for us, and Nebius Builder Program credits cover Token Factory, not AI Cloud. So the web service runs on Oracle's free tier, and the models stay on Token Factory.

## What the free tier gives, and what Airlock needs

| Resource | Always Free (per tenancy) | Airlock |
| --- | --- | --- |
| Arm compute (VM.Standard.A1.Flex) | "2 OCPUs and 12 GB of memory" in total | One VM: 2 OCPUs, 12 GB |
| Block storage | 200 GB in total; each boot volume is at least 47 GB | Boot volume: 100 GB |
| Outbound data | 10 TB per month | A few GB |
| Region | Only in your **home region** | Choose the home region at sign-up; it cannot be changed |

**What 2 OCPUs are enough for.** They are enough for the public, read-only demo: the tabs, the Floor's replay and the Frameline results. A live swarm compiles Rust once per agent, so run at most two agents at a time on this VM, and expect each check to take minutes.

**Architecture.** A1 is Arm (aarch64). Every piece of the image exists for arm64:
- the `ruby:3.3.6-slim`, `postgres:16` and `caddy:2` base images;
- rustup;
- the gems: `Gemfile.lock` lists `aarch64-linux` among its platforms.

## Idle reclamation (read before the judging period)

Oracle may reclaim an instance it considers idle. The rule is that, over 7 days, all three of these are true:

- the 95th-percentile CPU utilization is under 20%;
- network utilization is under 20%;
- memory utilization is under 20% (A1 shapes only).

Judging runs from **1 to 15 December 2026**, so the demo is likely to sit idle for weeks before then.

- **Memory.** The rule needs all three conditions, so staying above 20% memory (2.4 GB of 12 GB) should be enough to not count as idle. This is our reading of the rule; Oracle does not define the measurement. Give Postgres a larger `shared_buffers` (see step 7), and check Monitoring → Metrics for the instance in the weeks before judging.
- **Backup plan.** Keep a boot-volume backup (five are included free). If the instance is reclaimed, restoring it and running `deploy/airlock up` brings the demo back.
- **Pay As You Go.** Upgrading the account to Pay As You Go needs a card but keeps Always Free resources free, and is reported to make capacity easier to get. The docs do not say whether idle reclamation still applies after the upgrade, so do not count on it.

## Step by step

### 1. Account

1. Sign up at [oracle.com/cloud/free](https://www.oracle.com/cloud/free/). Oracle asks for a card to verify identity; Always Free resources are not charged.
2. **Pick the home region with care.** Always Free compute and block volumes exist only there. A region near the judges or near you is fine, but Arm capacity varies by region.

### 2. Network

In the console: Networking → Virtual cloud networks → **Start VCN Wizard** → "Create VCN with Internet Connectivity". The defaults are fine. This creates a public subnet with an internet gateway.

Then open the ports in the **public subnet's security list** (Default Security List → Add Ingress Rules):

| Source CIDR | Protocol | Destination port | Why |
| --- | --- | --- | --- |
| 0.0.0.0/0 | TCP | 22 | SSH (already there by default) |
| 0.0.0.0/0 | TCP | 80 | Let's Encrypt HTTP challenge and redirect |
| 0.0.0.0/0 | TCP | 443 | The demo |

### 3. SSH key

On your Mac, if you do not have one yet:

```sh
ssh-keygen -t ed25519 -f ~/.ssh/oracle_airlock -C airlock
cat ~/.ssh/oracle_airlock.pub     # paste this in the next step
```

### 4. The instance

Compute → Instances → **Create instance**:

- **Name:** `airlock`
- **Image:** Canonical Ubuntu 24.04. Pick the aarch64 build; the console shows only images compatible with the shape.
- **Shape:** Ampere → `VM.Standard.A1.Flex`, **2 OCPUs, 12 GB**
- **Networking:** the public subnet from step 2, with **Assign a public IPv4 address** checked
- **SSH keys:** paste the public key from step 3
- **Boot volume:** custom size **100 GB**. That leaves 100 GB of the free 200 GB for backups or a second volume.
- **Advanced options → Initialization script:** leave it empty. We run the bootstrap by hand in step 6, so the Token Factory key never goes into user data.

**"Out of host capacity"** means no free Arm capacity in that availability domain right now. Try another availability domain in the same region, or try again later. Oracle's own advice is to "wait a while, then try to create the instance again".

**Reserve the IP.** A public IP from the wizard is ephemeral: it changes if the instance is recreated. To keep the demo URL stable, use a reserved public IP: Networking → IP management → Reserved public IPs, then attach it to the instance's VNIC. Reserved IPs are free while attached.

### 5. The instance firewall

Oracle's Ubuntu images come with iptables rules that reject all inbound traffic except SSH, even when the security list allows it. See [Enabling network traffic to Ubuntu images in OCI](https://blogs.oracle.com/developers/enabling-network-traffic-to-ubuntu-images-in-oracle-cloud-infrastructure).

Docker publishes ports through its own chains, so Caddy may work without this step. Open 80 and 443 anyway, so that the host's rules match the security list:

```sh
ssh -i ~/.ssh/oracle_airlock ubuntu@<public-ip>

sudo iptables -I INPUT 6 -m state --state NEW -p tcp --dport 80 -j ACCEPT
sudo iptables -I INPUT 6 -m state --state NEW -p tcp --dport 443 -j ACCEPT
sudo netfilter-persistent save
```

### 6. Airlock

Still on the VM:

```sh
curl -fsSL https://raw.githubusercontent.com/emezac/airlock/main/deploy/bootstrap.sh -o bootstrap.sh
bash bootstrap.sh
```

The script:

1. installs Docker;
2. adds 8 GB of swap (the VM has less than 16 GB of memory);
3. clones the repository to `~/airlock`;
4. writes `deploy/.env` with fresh secrets and `AIRLOCK_HOST=<ip-with-dashes>.sslip.io`;
5. builds the image and starts Postgres, the service and Caddy;
6. creates the Frameline demo repository.

It asks for the Token Factory key and the project id without echoing them. The **reviewer token** is printed once, and it is also stored in `deploy/.env`.

The first build takes a while on 2 Arm cores: native gems, Rust and the crate download. Expect 15 to 30 minutes (an estimate; not measured on this shape).

**For this VM,** set `JOB_THREADS=4` in `deploy/.env` (2 cores: fewer parallel jobs), together with step 7, then run `deploy/airlock up`.

### 7. Postgres memory (idle reclamation)

In `deploy/.env` on the VM:

```sh
PG_SHARED_BUFFERS=3GB
PG_EFFECTIVE_CACHE_SIZE=6GB
```

Then run `deploy/airlock up`. With 12 GB of memory this is a normal setting, and it keeps resident memory above the 20% idle threshold once the buffers are in use.

This has not been measured. After a day, check the instance's memory metric in the console.

### 8. Check it

- `https://<ip-with-dashes>.sslip.io` shows the Control room, the Floor and the Frameline tab.
- `deploy/airlock logs` shows no errors. Caddy's logs (`deploy/airlock logs caddy`) show a certificate obtained.
- `curl -s -o /dev/null -w '%{http_code}' https://<host>/gate/push` returns `404`: the gate is not public.

**If the certificate fails** (for example, Let's Encrypt rate limits on a shared name), use a free DuckDNS name or your own domain:
1. Point it at the IP.
2. Set `AIRLOCK_HOST` in `deploy/.env`.
3. Run `deploy/airlock up`.

## Operating it

| Task | Command (from `~/airlock` on the VM) |
| --- | --- |
| Logs | `deploy/airlock logs` (`logs caddy`, `logs db`) |
| Update to the latest `main` | `git pull && deploy/airlock up` |
| Run the swarm (costs tokens) | `deploy/airlock rails airlock:swarm AGENTS=agent-ada,agent-kai` |
| The misbehaving agent (no tokens) | `deploy/airlock rails airlock:rogue` |
| Start the demo over | `deploy/airlock rails airlock:demo:reset` |
| Stop / start | `deploy/airlock down` / `deploy/airlock up` |
| Database backup | `docker compose -f deploy/compose.yml --env-file deploy/.env exec -T db pg_dump -U control control_production > backup.sql` |

For a whole-machine backup, use the console: Block Storage → Boot volumes → the instance's volume → Create manual backup. It is free up to five.

## Costs to watch

On Always Free, nothing here should be billed:
- one A1 instance within 2 OCPUs and 12 GB;
- a 100 GB boot volume;
- one reserved IP, attached;
- traffic well under 10 TB.

After signing up, check Billing → Cost analysis once to confirm it shows zero.

Token Factory is billed separately, per token. The public demo should not spend tokens (see "Public demo mode" in [../docs/launch-plan.md](../docs/launch-plan.md)). Only the swarm runs that you start yourself do.
