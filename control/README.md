# Airlock control service

Rails 8 service with [agentkit-rails](https://github.com/emezac/agentkit-rails-0.2). It exposes the push gate that `../hooks/pre-receive` calls, records every decision, and audits it in agentkit's hash-chained log.

## Requirements

- Ruby 3.2 or later (developed with 3.3.6)
- PostgreSQL 14 or later

## Setup

```sh
cd control
bundle install
bin/rails db:create db:migrate
AIRLOCK_HOOK_SECRET=change-me bin/rails server -p 3000
```

## Environment

| Variable | Purpose |
| --- | --- |
| `AIRLOCK_HOOK_SECRET` | Shared secret between the git hook and the gate. Required; without it every call is refused. |
| `AIRLOCK_POLICY` | Path to the policy file. Default: `config/airlock/airlock.toml`. |
| `NEBIUS_TOKEN` | Nebius Token Factory API key for Nemotron models. |
| `AGENTKIT_AUDIT_KEY` | Audit signing key. Required in production. |
| `SECRET_KEY_BASE` | Rails secret. Required in production. |

## Install the hook on a bare repository

```sh
cp ../hooks/pre-receive /srv/git/frameline.git/hooks/pre-receive
# The git server must export these to the hook:
#   AIRLOCK_URL=http://127.0.0.1:3000  AIRLOCK_HOOK_SECRET=change-me
# and set REMOTE_USER to the authenticated pusher (agent-<n> for agents).
```

## Gate rules (`app/services/airlock/gate.rb`)

1. Protected refs (`main`) can only be updated by the merge queue identity.
2. Agents (`agent-*`) can only push to `refs/heads/agents/<their name>/*`.
3. Agents cannot touch forbidden paths.
4. No added secrets.
5. No deleted test files or new skip markers.
6. Every agent commit carries `Airlock-Evidence: sandbox=<id> checkpoint=<id> cmd="<command>"`.
7. Sensitive paths are accepted but routed to human review.

The gate is deterministic: the same push report always gets the same decision. No model is consulted.

## Tests

```sh
bin/rails test
```
