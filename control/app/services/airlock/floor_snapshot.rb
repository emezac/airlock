module Airlock
  # Where everyone is right now, so the floor can lay out the scene before it
  # starts playing events: agents at their desks, and each change at the
  # station its state puts it in.
  class FloorSnapshot
    STATION = {
      "pending" => "labels", "needs_review" => "review", "approved" => "queue", "queued" => "queue",
      "merging" => "queue", "merged" => "main", "failed" => "doctor", "conflict" => "doctor", "rejected" => "returned"
    }.freeze

    def initialize(repo:, policy: Policy.load)
      @repo = repo
      @policy = policy
    end

    def to_h
      runs = AgentRun.where(repo: @repo).order(:created_at).to_a
      changes = Change.where(repo: @repo).order(:created_at).to_a
      pushers = changes.map(&:pusher).select { |p| @policy.agent?(p) }
      agents = (runs.map(&:agent) | pushers).sort.map do |name|
        run = runs.select { |r| r.agent == name }.last
        { name: name, status: run&.status || "idle", task: run&.task, note: run&.last_note }
      end
      in_flight = changes.reject { |c| %w[merged rejected].include?(c.state) }
      recent_done = changes.select { |c| %w[merged rejected].include?(c.state) }.last(8)
      {
        repo: @repo,
        cursor: FloorEvent.where(repo: @repo).maximum(:id) || 0,
        agents: agents,
        changes: (in_flight + recent_done).map do |c|
          { id: c.id, task: Floor.task_for(c.ref), agent: c.pusher, state: c.state, station: STATION.fetch(c.state, "labels"),
            risk: c.risk_score&.round(2), diagnosis: c.diagnosis.presence&.slice("category", "next_step") }
        end,
        counts: { merged: changes.count { |c| c.state == "merged" }, rejected_at_gate: GateDecision.where(repo: @repo, verdict: "reject").count,
                  reviews: changes.count { |c| c.state == "needs_review" } },
        sandbox: Sandboxes.available?(@policy),
        sandbox_reason: Sandboxes.status(@policy).reason,
        render: latest_render
      }
    end

    private

    def latest_render
      files = RepoFiles.for(@repo)
      return nil unless files.exists?

      manifest = Renders.new(@repo).latest(among: files.main_line)
      board = manifest && manifest["files"].find { |f| f.end_with?(".png") }
      board && { sha: manifest["sha"], board: board }
    rescue KeyError
      nil
    end
  end
end
