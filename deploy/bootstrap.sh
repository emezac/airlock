#!/usr/bin/env bash
# Sets up Airlock on a fresh Ubuntu 22.04/24.04 VM (e.g. Nebius AI Cloud) and
# starts it. Run as a user with sudo:
#
#   curl -fsSL https://raw.githubusercontent.com/emezac/airlock/main/deploy/bootstrap.sh | bash
#
# or from a clone: deploy/bootstrap.sh. Safe to run again: it keeps an
# existing deploy/.env and rebuilds.
#
# NEBIUS_TOKEN and NEBIUS_PROJECT_ID can be exported beforehand; otherwise the
# script asks (the token is not echoed). Leave them empty to start without
# models and add them to deploy/.env later.
set -euo pipefail

REPO_URL=${AIRLOCK_REPO_URL:-https://github.com/emezac/airlock.git}
DIR=${AIRLOCK_DIR:-$HOME/airlock}

say() { printf '\n== %s\n' "$*"; }

if ! command -v docker >/dev/null || ! docker compose version >/dev/null 2>&1; then
  say "Installing Docker"
  curl -fsSL https://get.docker.com | sudo sh
  sudo usermod -aG docker "$USER" || true
fi
DOCKER="docker"; docker info >/dev/null 2>&1 || DOCKER="sudo docker"

# Rust builds of several agents at once need memory; add swap on small VMs.
if [ "$(swapon --noheadings | wc -l)" -eq 0 ] && [ "$(awk '/MemTotal/ {print $2}' /proc/meminfo)" -lt 16000000 ]; then
  say "Adding 8 GB of swap"
  sudo fallocate -l 8G /swapfile && sudo chmod 600 /swapfile && sudo mkswap /swapfile >/dev/null && sudo swapon /swapfile
  grep -q '^/swapfile' /etc/fstab || echo '/swapfile none swap sw 0 0' | sudo tee -a /etc/fstab >/dev/null
fi

if [ ! -d "$DIR/.git" ]; then
  say "Cloning $REPO_URL"
  git clone "$REPO_URL" "$DIR"
else
  git -C "$DIR" pull --ff-only || true
fi
cd "$DIR"

ENV_FILE=deploy/.env
if [ ! -f "$ENV_FILE" ]; then
  say "Writing $ENV_FILE"
  ip=${AIRLOCK_PUBLIC_IP:-$(curl -fsS https://api.ipify.org || true)}
  host=${AIRLOCK_HOST:-${ip//./-}.sslip.io}
  token=${NEBIUS_TOKEN:-}
  project=${NEBIUS_PROJECT_ID:-}
  if [ -z "$token" ] && [ -t 0 ]; then read -rsp "Nebius Token Factory key (empty to skip): " token; echo; fi
  if [ -z "$project" ] && [ -t 0 ]; then read -rp "Nebius project id (empty to skip): " project; fi
  review=$(openssl rand -hex 12)
  umask 077
  cat > "$ENV_FILE" <<ENV
AIRLOCK_HOST=$host
CONTROL_DATABASE_PASSWORD=$(openssl rand -hex 24)
SECRET_KEY_BASE=$(openssl rand -hex 64)
AGENTKIT_AUDIT_KEY=$(openssl rand -hex 32)
AIRLOCK_HOOK_SECRET=$(openssl rand -hex 24)
AIRLOCK_REVIEW_TOKEN=$review
NEBIUS_TOKEN=$token
NEBIUS_PROJECT_ID=$project
NEBIUS_SANDBOX=false
AIRLOCK_SEED_DEMO=1
JOB_THREADS=8
ENV
  echo "Reviewer token: $review  (also in $ENV_FILE)"
fi

say "Building and starting (the first build compiles Ruby gems and installs Rust)"
$DOCKER compose -f deploy/compose.yml --env-file "$ENV_FILE" up -d --build

host=$(grep '^AIRLOCK_HOST=' "$ENV_FILE" | cut -d= -f2)
say "Waiting for the service"
for _ in $(seq 1 90); do
  if $DOCKER compose -f deploy/compose.yml --env-file "$ENV_FILE" exec -T app curl -fsS http://127.0.0.1:3000/up >/dev/null 2>&1; then
    echo "Airlock is up: https://$host  (Control room, Floor, Frameline)"
    echo "Run the swarm:  deploy/airlock rails airlock:swarm AGENTS=agent-ada,agent-kai,agent-lin"
    exit 0
  fi
  sleep 5
done
echo "The service did not answer yet; see: $DOCKER compose -f deploy/compose.yml logs app" >&2
exit 1
