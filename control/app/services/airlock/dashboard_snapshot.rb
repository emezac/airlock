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
        swarm: swarm,
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

    # The latest swarm: one row per agent, showing the task it is on now (or
    # its last one) and how far through its share it is.
    def swarm
      recent = AgentRun.where(repo: @repo).where.not(swarm_id: nil).order(created_at: :desc).limit(100).to_a
      id = recent.first&.swarm_id
      runs = recent.select { |r| r.swarm_id == id }
      agents = runs.group_by(&:agent).sort.map do |agent, rs|
        now = rs.find { |r| r.status == "working" } || rs.select(&:finished?).max_by(&:finished_at) || rs.min_by(&:id)
        { agent: agent, task: now.task, title: now.title, status: now.status, attempts: now.attempts,
          note: now.last_note, done: rs.count(&:finished?), total: rs.size,
          tokens: rs.sum { |r| r.input_tokens + r.output_tokens } }
      end
      { id: id, agents: agents, outcomes: runs.select(&:finished?).map(&:status).tally,
        tokens: runs.sum { |r| r.input_tokens + r.output_tokens } }
    end

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
        diagnosis: change.diagnosis.presence&.slice("category", "category_source", "summary", "next_step", "culprit_files", "source"),
        age_seconds: (@now - change.created_at).round
      }
    end

    def batch(batch)
      { id: batch.id, state: batch.state, size: batch.size, ci_runs: batch.ci_runs,
        merged: batch.batch_changes.count { |c| c.state == "merged" }, at: batch.created_at.iso8601 }
    end
  end
end
