module Airlock
  # Re-runs the command an agent cites as evidence, on the exact commit it
  # pushed, in a clean sandbox. "I tested it" becomes a check anyone can repeat.
  class EvidenceVerifier
    Outcome = Data.define(:status, :command, :checkpoint, :output)

    def initialize(policy, workspace:, runner:)
      @policy = policy
      @workspace = workspace
      @runner = runner
    end

    def verify(change, push)
      evidence = push.commits.filter_map { |c| Evidence.parse(c.trailers[@policy.evidence_trailer]) }.last
      return Outcome.new(status: "unverifiable", command: nil, checkpoint: nil, output: "no evidence trailer") unless evidence

      @workspace.prepare!
      @workspace.checkout(change.ref)
      result = @runner.call(@workspace.path, command: evidence.cmd)
      Outcome.new(status: result.ok ? "verified" : "failed", command: evidence.cmd,
                  checkpoint: result.respond_to?(:checkpoint) ? result.checkpoint : nil, output: result.output.to_s.last(4000))
    end
  end
end
