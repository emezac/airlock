module Airlock
  # Turns an accepted agent push into a Change and routes it: to the merge
  # queue, or to a human when a rule, the risk threshold or ambiguity says so.
  class Intake
    def initialize(policy, classifier: Classifier.new, now: Time.current)
      @policy = policy
      @classifier = classifier
      @now = now
    end

    # Records the change right away; routing may call a model, so it runs later.
    def self.admit(policy, push, decision_row)
      return nil unless policy.agent?(push.pusher) && !push.deletion?

      Change.create!(repo: push.repo, ref: push.ref, pusher: push.pusher, head_sha: push.new_sha,
                     base_sha: push.old_sha, gate_decision: decision_row, state: "pending")
    end

    def route!(change, push)
      classification = @classifier.classify(files: push.files.map(&:path), messages: push.commits.map(&:message))
      score = Risk.new(@policy, core_paths: @policy.core_paths)
                  .score(push, agent_failure_rate: AgentStat.rate_for(push.pusher), classification: classification)
      change.update!(risk_score: score.value, risk_features: score.features, classification: classification.to_h)

      estimate = Attention.new(@policy, now: @now).estimate(push.repo)
      reasons = change.gate_decision.review_reasons.dup
      reasons << "risk #{score.value} above threshold #{estimate.threshold}" if score.value > estimate.threshold
      if classification.source != "none" && classification.clarity < @policy.ambiguity_clarity
        reasons << "goal is unclear (clarity #{classification.clarity})"
      end

      if reasons.any?
        change.update!(state: "needs_review", review_reasons: reasons, review_requested_at: @now)
      else
        change.update!(state: "queued")
      end
      Agentkit::Audit.record(event_type: "change.routed", status: change.state, subject: change,
                             payload: { risk: score.value, threshold: estimate.threshold, rho: estimate.expected_rho,
                                        reasons: reasons })
      change
    end
  end
end
