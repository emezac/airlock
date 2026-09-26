# syntax=docker/dockerfile:1
# Airlock on one machine: the Rails control service, the git repositories
# with the pre-receive hook, and the local runner's toolchain (Rust, ffmpeg).
#
# Build from the repository root:
#   docker build -f deploy/Dockerfile -t airlock .
#
# The local runner builds and tests agent-written code inside this container.
# It gets a minimal environment (no secrets), but it is not a sandbox: run the
# container on a machine dedicated to Airlock. See deploy/README.md.

ARG RUBY_VERSION=3.3.6
FROM docker.io/library/ruby:$RUBY_VERSION-slim AS base

WORKDIR /app/control

RUN apt-get update -qq && \
    apt-get install --no-install-recommends -y \
      ca-certificates curl git ffmpeg libjemalloc2 postgresql-client \
      build-essential pkg-config && \
    ln -s /usr/lib/$(uname -m)-linux-gnu/libjemalloc.so.2 /usr/local/lib/libjemalloc.so && \
    rm -rf /var/lib/apt/lists /var/cache/apt/archives

ENV RAILS_ENV="production" \
    BUNDLE_DEPLOYMENT="1" \
    BUNDLE_PATH="/usr/local/bundle" \
    BUNDLE_WITHOUT="development:test" \
    LD_PRELOAD="/usr/local/lib/libjemalloc.so"

# ---- gems and assets -------------------------------------------------------
FROM base AS build

RUN apt-get update -qq && \
    apt-get install --no-install-recommends -y libpq-dev libyaml-dev && \
    rm -rf /var/lib/apt/lists /var/cache/apt/archives

COPY control/Gemfile control/Gemfile.lock ./
RUN bundle install && \
    rm -rf ~/.bundle/ "${BUNDLE_PATH}"/ruby/*/cache "${BUNDLE_PATH}"/ruby/*/bundler/gems/*/.git && \
    bundle exec bootsnap precompile -j 1 --gemfile

COPY control/ /app/control/
RUN bundle exec bootsnap precompile -j 1 app/ lib/ && \
    SECRET_KEY_BASE_DUMMY=1 AGENTKIT_AUDIT_KEY=build-only \
      DATABASE_URL=postgres://build:build@127.0.0.1:1/none \
      ./bin/rails assets:precompile

# ---- runtime ---------------------------------------------------------------
FROM base

RUN apt-get update -qq && \
    apt-get install --no-install-recommends -y libpq5 libyaml-0-2 && \
    rm -rf /var/lib/apt/lists /var/cache/apt/archives && \
    groupadd --system --gid 1000 airlock && \
    useradd airlock --uid 1000 --gid 1000 --create-home --shell /bin/bash && \
    mkdir -p /data/git /data/work && chown -R airlock:airlock /data

USER 1000:1000

# Rust for the local runner (Frameline's checks, golden images and renders).
ENV CARGO_HOME=/home/airlock/.cargo RUSTUP_HOME=/home/airlock/.rustup \
    PATH=/home/airlock/.cargo/bin:$PATH
RUN curl -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable && \
    cargo --version

# Warm the crate cache with the demo's dependencies, so the first agent does
# not spend its check downloading them.
COPY --chown=airlock:airlock demo/frameline/Cargo.toml demo/frameline/Cargo.lock /tmp/frameline/
RUN cd /tmp/frameline && mkdir -p src && echo 'fn main() {}' > src/main.rs && \
    cargo fetch && rm -rf /tmp/frameline

COPY --chown=airlock:airlock --from=build "${BUNDLE_PATH}" "${BUNDLE_PATH}"
COPY --chown=airlock:airlock --from=build /app/control /app/control
COPY --chown=airlock:airlock hooks/ /app/hooks/
COPY --chown=airlock:airlock demo/ /app/demo/

RUN git config --global init.defaultBranch main && \
    git config --global user.name airlock && \
    git config --global user.email airlock@airlock.local

ENV AIRLOCK_GIT_ROOT=/data/git \
    AIRLOCK_WORK_ROOT=/data/work \
    AIRLOCK_URL=http://127.0.0.1:3000 \
    SOLID_QUEUE_IN_PUMA=1 \
    PORT=3000 \
    BINDING=0.0.0.0

VOLUME ["/data"]
EXPOSE 3000
ENTRYPOINT ["/app/control/bin/docker-entrypoint"]
CMD ["./bin/rails", "server"]
