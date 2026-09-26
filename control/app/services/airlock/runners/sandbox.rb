module Airlock
  module Runners
    # Runs a command on a clean copy of a tree inside a Token Factory Sandbox.
    # The tree travels as a tarball, the sandbox has no network, and the base
    # image is fixed by policy, so the result depends only on the commit.
    class Sandbox
      Result = Data.define(:ok, :output, :checkpoint, :run_id)

      REPO_TARBALL = "/airlock/repo.tar.gz".freeze

      def initialize(client:, image:, command:, timeout:)
        @client = client
        @image = image
        @command = command
        @timeout = timeout
      end

      # dir: a checked-out tree (the merge queue workspace).
      # collect: path prefixes whose files are copied back into dir afterwards
      # (for example regenerated golden images).
      def call(dir, command: @command, collect: [])
        tarball = archive(dir)
        file = @client.upload(tarball)
        script = "mkdir -p /work && tar -xzf #{REPO_TARBALL} -C /work && cd /work && (#{command})"
        script += " && #{Collector.script(collect)}" if collect.any?
        op = @client.run(image: @image, command: script, files: { REPO_TARBALL => file }, timeout: @timeout)
        ok = op.status == "SUCCESS" && op.exit_code.to_i.zero? && !op.timed_out
        stdout = op.stdout.to_s
        stdout, = Collector.extract!(stdout, dir, collect) if ok && collect.any?
        Result.new(ok: ok, output: (stdout + op.stderr.to_s).lines.last(400).join, checkpoint: op.result_image, run_id: op.uuid)
      end

      private

      # Tracked files only: build outputs and untracked files never travel.
      def archive(dir)
        out, status = Open3.capture2("git", "archive", "--format=tar.gz", "HEAD", chdir: dir, binmode: true)
        raise GitWorkspace::Error, "git archive failed in #{dir}" unless status.success?

        out
      end
    end
  end
end
