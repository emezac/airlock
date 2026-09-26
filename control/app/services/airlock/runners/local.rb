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

      # collect: ignored here, the command already writes into dir.
      def call(dir, command: @command, collect: [])
        out, status = Open3.capture2e(ISOLATED_ENV, "timeout", @timeout.to_s, "sh", "-c", command, chdir: dir)
        Result.new(ok: status.success?, output: out.lines.last(400).join, checkpoint: nil, run_id: nil)
      end
    end
  end
end
