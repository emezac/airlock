module Airlock
  # Deterministic admission control for one push. Same input, same decision:
  # no model is consulted here.
  class Gate
    class Decision < Data.define(:verdict, :reasons, :review, :review_reasons)
      def accepted? = verdict == "accept"

      def message
        return (["rejected"] + reasons).join("\n- ") unless accepted?

        review == "human" ? "accepted; needs human review: #{review_reasons.join('; ')}" : "accepted"
      end
    end

    def initialize(policy)
      @policy = policy
    end

    def evaluate(push)
      reasons = []
      reasons.concat(ref_violations(push))
      unless push.deletion?
        reasons.concat(path_violations(push)) if @policy.agent?(push.pusher)
        reasons.concat(secret_violations(push)) if @policy.reject_on_secret
        reasons.concat(test_violations(push)) if @policy.reject_on_skipped_tests
        reasons.concat(evidence_violations(push)) if @policy.agent?(push.pusher)
      end
      return Decision.new(verdict: "reject", reasons: reasons, review: nil, review_reasons: []) if reasons.any?

      review_reasons = review_reasons_for(push)
      Decision.new(verdict: "accept", reasons: [],
                   review: review_reasons.any? ? "human" : "auto", review_reasons: review_reasons)
    end

    private

    def ref_violations(push)
      if @policy.protected_refs.include?(push.ref) && push.pusher != @policy.merge_identity
        return ["#{push.ref} is protected; only the merge queue (#{@policy.merge_identity}) may update it"]
      end
      if @policy.agent?(push.pusher) && !push.ref.start_with?(@policy.agent_branch_prefix(push.pusher))
        return ["agent #{push.pusher} may only push to #{@policy.agent_branch_prefix(push.pusher)}*"]
      end

      []
    end

    def path_violations(push)
      push.files.select { |f| @policy.forbidden?(f.path) }.map { |f| "forbidden path: #{f.path}" }
    end

    def secret_violations(push)
      push.added_lines.filter_map do |line|
        "possible secret added in #{line.path}" if @policy.secret_patterns.any? { |re| re.match?(line.text) }
      end.uniq
    end

    def test_violations(push)
      deleted = push.files.select { |f| f.deleted? && @policy.test_path?(f.path) }
                    .map { |f| "test file deleted: #{f.path}" }
      skipped = push.added_lines.filter_map do |line|
        "test skip marker added in #{line.path}" if @policy.skip_patterns.any? { |re| re.match?(line.text) }
      end
      deleted + skipped.uniq
    end

    def evidence_violations(push)
      return ["no commits to verify"] if push.commits.empty?

      push.commits.filter_map do |commit|
        evidence = Evidence.parse(commit.trailers[@policy.evidence_trailer])
        next "commit #{commit.sha[0, 12]} lacks a valid #{@policy.evidence_trailer} trailer (sandbox, checkpoint, cmd)" unless evidence

        command_violation(commit, evidence.cmd)
      end
    end

    # The evidence command is re-run in a sandbox, so it must be one command
    # from the allowed list, without chaining or substitution.
    SHELL_META = /[;&|`$<>\n\\]/

    def command_violation(commit, cmd)
      sha = commit.sha[0, 12]
      return "commit #{sha} evidence command uses shell operators: #{cmd}" if cmd.match?(SHELL_META)

      allowed = @policy.allowed_evidence_commands
      return nil if allowed.empty? || allowed.any? { |prefix| cmd == prefix || cmd.start_with?("#{prefix} ") }

      "commit #{sha} evidence command is not allowed: #{cmd} (allowed: #{allowed.join(', ')})"
    end

    def review_reasons_for(push)
      push.files.select { |f| @policy.human_review?(f.path) }.map { |f| "sensitive path #{f.path}" }.uniq
    end
  end
end
