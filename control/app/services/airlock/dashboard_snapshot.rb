module Airlock
  # Everything the live panel shows, computed from the database in one place,
  # so the HTML view and the JSON endpoint cannot disagree.
  class DashboardSnapshot
    COLUMNS = {
      "review" => %w[needs_review],
      "queue" => %w[pending queued approved merging],
      "merged" => %w[merged],
      "stopped" => %w[rejected failed conflict]
    }.freeze

    def initialize(repo: nil, policy: Policy.load, now: Time.current)
      @repo = repo || Change.order(created_at: :desc).pick(:repo) || "frameline"
      @policy = policy
      @now = now
    end

    def to_h
      changes = Change.where(repo: @repo).includes(:labels).order(created_at: :desc).limit(200).to_a
      estimate = Attention.new(@policy, now: @now).estimate(@repo)
      f = failure_rate(changes)
      k = BatchMath.best_size(f, max: @policy.max_batch)
      {
        repo: @repo,
        generated_at: @now.iso8601,
        attention: estimate.to_h.merge(target_rho: @policy.target_utilization),
        batching: { failure_rate: f.round(3), batch_size: k,
                    ci_runs_per_change: (BatchMath.expected_runs(k, f) / k).round(3) },
        counts: changes.map(&:state).tally,
        columns: COLUMNS.transform_values { |states| changes.select { |c| states.include?(c.state) }.first(12).map { |c| card(c) } },
        batches: MergeBatch.where(repo: @repo).order(created_at: :desc).limit(20).map { |b| batch(b) },
        agents: AgentStat.order(:agent).map { |a| { agent: a.agent, failure_rate: a.failure_rate.round(3), observations: a.observations } },
        gate: GateDecision.where(repo: @repo).group(:verdict).count
      }
    end

    private

    def failure_rate(changes)
      rates = changes.map(&:pusher).uniq.map { |agent| AgentStat.rate_for(agent) }
      rates.empty? ? AgentStat::PRIOR : (rates.sum / rates.size).clamp(0.01, 0.9)
    end

    def card(change)
      {
        id: change.id, branch: change.ref.split("/").last, agent: change.pusher, state: change.state,
        risk: change.risk_score&.round(2), evidence: change.evidence_status,
        reasons: change.review_reasons.first(3),
        labels: change.effective_labels.transform_values { |l| { value: l.value, source: l.source } },
        age_seconds: (@now - change.created_at).round
      }
    end

    def batch(batch)
      { id: batch.id, state: batch.state, size: batch.size, ci_runs: batch.ci_runs,
        merged: batch.batch_changes.count { |c| c.state == "merged" }, at: batch.created_at.iso8601 }
    end
  end
end
