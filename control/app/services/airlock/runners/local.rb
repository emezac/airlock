require "open3"

module Airlock
  module Runners
    # Runs the CI command in the candidate tree on this machine. The sandbox
    # runner implements the same #call(dir) contract on Token Factory Sandboxes.
    class Local
      Result = Data.define(:ok, :output, :checkpoint, :run_id)

      def initialize(command:, timeout:)
        @command = command
        @timeout = timeout
      end

      # Build caches shared between trees let one tree's artifacts answer for
      # another's code (seen with parallel agents), so each run builds only
      # inside its own tree.
      ISOLATED_ENV = { "CARGO_TARGET_DIR" => nil }.freeze

      # The tree holds agent-written code, and `cargo test` runs it (tests,
      # build scripts). It gets a minimal environment: never the service's
      # secrets (NEBIUS_TOKEN, SECRET_KEY_BASE, DATABASE_URL, the hook secret).
      # This is not a sandbox; it only keeps the obvious keys out of reach.
      PASSED_ENV = %w[PATH HOME LANG LC_ALL TZ TMPDIR CARGO_HOME RUSTUP_HOME].freeze

      def self.env
        ENV.to_h.slice(*PASSED_ENV).merge(ISOLATED_ENV)
      end

      # collect: ignored here, the command already writes into dir.
      def call(dir, command: @command, collect: [])
        out, status = ActiveSupport::Dependencies.interlock.permit_concurrent_loads do
          Open3.capture2e(self.class.env, "timeout", @timeout.to_s, "sh", "-c", command,
                          chdir: dir, unsetenv_others: true)
        end
        Result.new(ok: status.success?, output: out.lines.last(400).join, checkpoint: nil, run_id: nil)
      end
    end
  end
end
