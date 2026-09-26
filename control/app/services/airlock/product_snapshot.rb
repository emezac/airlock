require "yaml"

module Airlock
  # What the Frameline tab shows: the brief, the backlog derived from it with
  # each task's live status, the boards before the swarm and on main now, the
  # visual changes waiting for a person, and the latest render of main.
  class ProductSnapshot
    DEMO = Rails.root.join("../demo").to_s

    def initialize(repo: "frameline", brief: ENV.fetch("AIRLOCK_BRIEF") { File.join(DEMO, "brief.yml") },
                   backlog: ENV.fetch("AIRLOCK_BACKLOG") { File.join(DEMO, "backlog.yml") },
                   files: RepoFiles.for(repo), renders: Renders.new(repo))
      @repo = repo
      @brief_path = brief
      @backlog_path = backlog
      @files = files
      @renders = renders
    end

    def to_h
      root = @files.exists? ? @files.root : nil
      main = @files.exists? ? @files.main : nil
      {
        repo: @repo,
        brief: brief,
        tasks: tasks,
        root: root, main: main,
        boards: boards(root, main),
        history: root && main ? @files.log_since(root, main).map { |c| c.merge(short: c[:sha][0, 7]) } : [],
        reviews: reviews(main),
        render: main && @renders.latest(among: @files.main_line),
      }
    end

    private

    def brief
      File.exist?(@brief_path) ? YAML.safe_load_file(@brief_path) : { "title" => "No brief", "asks" => [] }
    end

    def tasks
      raw = YAML.safe_load_file(@backlog_path).fetch("tasks")
      changes = Change.where(repo: @repo).order(:created_at).to_a
      runs = AgentRun.where(repo: @repo).order(:created_at).to_a
      raw.map do |t|
        a = Assignment.from_h(t)
        change = changes.select { |c| c.ref.end_with?("/#{a.slug}") }.last
        run = runs.select { |r| r.task == a.id }.last
        status, detail = status_for(change, run)
        { id: a.id, title: a.title, asks: Array(t["asks"]), status: status, detail: detail,
          agent: change&.pusher || run&.agent, attempts: run&.attempts, change_id: change&.id }
      end
    end

    # The furthest a task has got decides its status.
    def status_for(change, run)
      if change
        return ["merged", nil] if change.state == "merged"
        return ["review", change.review_reasons.first] if change.state == "needs_review"
        return ["queued", change.state] if %w[pending queued approved merging].include?(change.state)
        return ["rejected", change.diagnosis["summary"]] if change.state == "rejected"

        return ["failed", change.diagnosis["summary"]] if %w[failed conflict].include?(change.state)
      end
      return ["not started", nil] unless run

      case run.status
      when "working", "waiting" then ["working", run.last_note]
      when "gave_up" then ["gave up", run.last_note]
      when "pushed" then ["queued", nil]
      else ["error", run.last_note]
      end
    end

    def boards(root, main)
      return [] unless root && main

      names = (@files.goldens(root) | @files.goldens(main)).sort
      names.map do |path|
        before = @files.blob_id(root, path)
        after = @files.blob_id(main, path)
        { name: File.basename(path, ".png"), path: path, before: before && root, after: after && main,
          changed: before != after, added: before.nil? }
      end
    end

    # Every change waiting for a person; the ones that change how a board
    # looks carry the current and proposed boards side by side.
    def reviews(main)
      return [] unless main

      Change.where(repo: @repo, state: "needs_review").order(:created_at).map do |change|
        goldens = Array(change.gate_decision&.changed_paths).grep(RepoFiles::GOLDEN)
        base = goldens.any? ? (@files.merge_base(main, change.head_sha) || main) : main
        { id: change.id, branch: change.ref.delete_prefix("refs/heads/"), agent: change.pusher, sha: change.head_sha,
          render: goldens.any? ? @renders.manifest(change.head_sha) : nil,
          reasons: change.review_reasons, risk: change.risk_score&.round(2),
          boards: goldens.map { |p| { name: File.basename(p, ".png"), path: p, before: @files.blob_id(base, p) && base, after: change.head_sha } } }
      end
    end
  end
end
