require "securerandom"

module Airlock
  # The swarm: backlog tasks spread over agents, each agent working its share
  # one task at a time in its own clone. SwarmFlow runs the agents in parallel;
  # this module holds the pieces so they can be tested without the flow engine.
  module Swarm
    # A task whose branch already reached one of these states is not redone.
    DONE_STATES = %w[pending queued approved merging merged needs_review].freeze

    class << self
      # Test and development seams: (agent, task) -> model, () -> runner.
      attr_writer :model_factory, :runner_factory

      # Round-robin over agents. Returns one work item per agent that got tasks.
      def plan(repo:, agents:, backlog:, only: nil, swarm_id: SecureRandom.uuid)
        raise ArgumentError, "no agents" if agents.blank?

        tasks = Assignment.load_all(backlog)
        tasks = tasks.select { |t| Array(only).include?(t.id) } if only.present?
        pending = tasks.reject { |t| done?(repo, t) || held?(repo, t) }
        groups = agents.index_with { [] }
        pending.each_with_index { |t, i| groups[agents[i % agents.size]] << t.id }
        groups.reject { |_, ids| ids.empty? }.map do |agent, ids|
          { "repo" => repo, "agent" => agent, "tasks" => ids, "backlog" => backlog, "swarm_id" => swarm_id }
        end
      end

      def done?(repo, task) = task_changes(repo, task).where(state: DONE_STATES).exists?

      # The last attempt failed in the merge queue and its diagnosis says a
      # person should decide (or drop it): agents do not retry it on their own.
      def held?(repo, task)
        last = task_changes(repo, task).where(state: %w[failed conflict]).order(:created_at).last
        %w[human drop].include?(last&.diagnosis&.dig("next_step"))
      end

      def task_changes(repo, task)
        Change.where(repo: repo).where("ref LIKE ?", "%/#{Change.sanitize_sql_like(task.slug)}")
      end

      # One agent's share, task after task. Safe to redeliver: finished tasks of
      # this swarm are not run again.
      def work(item)
        repo, agent, swarm = item.fetch("repo"), item.fetch("agent"), item.fetch("swarm_id")
        assignments = Assignment.load_all(item.fetch("backlog")).index_by(&:id)
        item.fetch("tasks").map do |id|
          run = AgentRun.find_or_create_by!(swarm_id: swarm, agent: agent, task: id) do |r|
            r.repo = repo
            r.title = assignments.fetch(id).title
          end
          run.finished? ? summary(run) : perform(run, assignments.fetch(id))
        end
      end

      def report(work)
        rows = Array(work).flatten.compact
        {
          "tasks" => rows.size,
          "by_status" => rows.map { |r| r["status"] }.tally,
          "tokens" => { "input" => rows.sum { |r| r["input_tokens"].to_i }, "output" => rows.sum { |r| r["output_tokens"].to_i } },
          "runs" => rows
        }
      end

      def worker_for(repo, agent, task)
        policy = Policy.load
        Worker.new(agent: agent, workspace: workspace_for(repo, agent), runner: runner(policy), model: model(agent, task),
                   policy: policy)
      end

      # An agent's own clone; pushes carry the hook's settings so they reach the gate.
      def workspace_for(repo, agent)
        remote = ENV.fetch("AIRLOCK_PUSH_URL") { File.join(ENV.fetch("AIRLOCK_GIT_ROOT"), "#{repo}.git") }
        work = ENV.fetch("AIRLOCK_WORK_ROOT", Rails.root.join("tmp/workspaces").to_s)
        env = { "AIRLOCK_URL" => ENV["AIRLOCK_URL"], "AIRLOCK_HOOK_SECRET" => ENV["AIRLOCK_HOOK_SECRET"] }.compact
        GitWorkspace.new(remote: remote, path: File.join(work, "#{repo}-#{agent}"), author: agent, identity: agent, env: env)
      end

      private

      def perform(run, assignment)
        assignment = with_history(run.repo, assignment)
        run.update!(status: "working", started_at: Time.current)
        result = worker_for(run.repo, run.agent, run.task).run(assignment, on_note: ->(line) { run.note!(line) })
        run.finish!(result)
        summary(run)
      rescue StandardError => e
        run.update!(status: "error", last_note: "#{e.class}: #{e.message}".truncate(240), finished_at: Time.current)
        summary(run)
      end

      # A task retried after failing in the merge queue carries that diagnosis,
      # so the agent starts from the error instead of repeating it.
      def with_history(repo, assignment)
        last = Change.where(repo: repo, state: %w[failed conflict])
                     .where("ref LIKE ?", "%/#{Change.sanitize_sql_like(assignment.slug)}")
                     .where.not(diagnosis: {}).order(:created_at).last
        return assignment unless last

        d = last.diagnosis
        note = <<~TXT

          An earlier attempt at this task passed its own check but failed in the merge queue.
          Diagnosis (#{d['category']}): #{d['summary']}
          #{d['culprit_files'].present? ? "Files involved: #{d['culprit_files'].join(', ')}" : ''}
        TXT
        assignment.with(instructions: assignment.instructions + note)
      end

      def summary(run)
        run.slice(:agent, :task, :status, :attempts, :input_tokens, :output_tokens, :branch, :sha).stringify_keys
      end

      def model(agent, task)
        return @model_factory.call(agent, task) if @model_factory
        return RelayModel.new("#{task}-#{agent}-#{SecureRandom.hex(3)}") if RelayModel.configured?

        TokenFactory.new
      end

      def runner(policy)
        return @runner_factory.call if @runner_factory

        QueueRunner.sandbox_runner(policy) || Runners::Local.new(command: policy.ci_command, timeout: policy.ci_timeout)
      end
    end
  end
end
