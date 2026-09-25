require "open3"

module Airlock
  module Runners
    # Runs the CI command in the candidate tree on this machine. The sandbox
    # runner implements the same #call(dir) contract on Token Factory Sandboxes.
    class Local
      Result = Data.define(:ok, :output)

      def initialize(command:, timeout:)
        @command = command
        @timeout = timeout
      end

      def call(dir)
        out, status = Open3.capture2e("timeout", @timeout.to_s, "sh", "-c", @command, chdir: dir)
        Result.new(ok: status.success?, output: out.lines.last(40).join)
      end
    end
  end
end
