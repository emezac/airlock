module Airlock
  # Turns an accepted agent push into a Change, labels it and routes it: to the
  # merge queue, or to a human when a gate rule, a routing label or the risk
  # threshold says so.
  class Intake
    ROUTABLE = %w[pending queued needs_review].freeze

    # Records the change right away; labelling may call a model, so it runs later.
    # The hook's report is kept so a later label override can re-route the
    # change without asking git or the model again.
    def self.admit(policy, push, decision_row, report = {})
      return nil unless policy.agent?(push.pusher) && !push.deletion?

      Change.create!(repo: push.repo, ref: push.ref, pusher: push.pusher, head_sha: push.new_sha,
                     base_sha: push.old_sha, gate_decision: decision_row, state: "pending", report: report)
    end

    def initialize(policy, labeler: Labeler.new(policy), now: Time.current)
      @policy = policy
      @labeler = labeler
      @now = now
    end

    def route!(change, push)
      Change.transaction do
        change.labels.where.not(source: "override").delete_all
        @labeler.call(push).each do |l|
          change.labels.create!(category: l.category, value: l.value, source: l.source,
                                confidence: l.confidence, reason: l.reason)
        end
      end
      decide!(change.reload, push)
    end

    # Re-routes after a human label override, without asking the model again.
    def reroute!(change, push)
      return change unless ROUTABLE.include?(change.state)

      decide!(change, push)
    end

    private

    def decide!(change, push)
      effective = change.effective_labels.transform_values(&:value)
      score = Risk.new.score(push, agent_failure_rate: AgentStat.rate_for(push.pusher), labels: effective)
      change.update!(risk_score: score.value, risk_features: score.features, classification: effective)

      estimate = Attention.new(@policy, now: @now).estimate(push.repo)
      reasons = change.gate_decision.review_reasons.dup
      reasons += @policy.human_when.select { |rule| effective[rule.split(":").first] == rule.split(":").last }
                                   .map { |rule| "label #{rule}" }
      reasons << "risk #{score.value} above threshold #{estimate.threshold}" if score.value > estimate.threshold

      if reasons.any?
        change.update!(state: "needs_review", review_reasons: reasons, review_requested_at: change.review_requested_at || @now)
      else
        change.update!(state: "queued", review_reasons: [])
      end
      Agentkit::Audit.record(event_type: "change.routed", status: change.state, subject: change,
                             payload: { risk: score.value, threshold: estimate.threshold, rho: estimate.expected_rho,
                                        labels: effective, reasons: reasons })
      change
    end
  end
end
