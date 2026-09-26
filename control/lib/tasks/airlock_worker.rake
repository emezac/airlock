namespace :airlock do
  def airlock_backlog = ENV.fetch("AIRLOCK_BACKLOG") { Rails.root.join("../demo/backlog.yml").to_s }

  desc "Run one agent on one backlog task: bin/rails airlock:work TASK=FL-1 AGENT=agent-ada [REPO=frameline]"
  task work: :environment do
    assignment = Airlock::Assignment.load_all(airlock_backlog).find { |a| a.id == ENV.fetch("TASK") } or abort "no task #{ENV['TASK']}"
    worker = Airlock::Swarm.worker_for(ENV.fetch("REPO", "frameline"), ENV.fetch("AGENT"), assignment.id)
    result = worker.run(assignment, on_note: ->(line) { puts line })
    puts "#{result.status}: #{result.branch} #{result.sha} · #{result.attempts} attempts · " \
         "#{result.input_tokens} in / #{result.output_tokens} out tokens"
    exit(result.pushed? ? 0 : 1)
  end

  desc "Run the swarm over the backlog: bin/rails airlock:swarm AGENTS=agent-ada,agent-kai [ONLY=FL-3,FL-4] [REPO=frameline]"
  task swarm: :environment do
    swarm_id = SecureRandom.uuid
    input = { repo: ENV.fetch("REPO", "frameline"), agents: ENV.fetch("AGENTS").split(","), backlog: airlock_backlog,
              only: ENV["ONLY"]&.split(","), swarm_id: swarm_id }
    # Each agent's share is its own branch job, so agents work in parallel. With
    # the in-process job adapter this task must stay alive until the run ends.
    # Load everything first: agent threads then never wait on the autoloader.
    Rails.application.eager_load!
    run = SwarmFlow.perform_later(**input)
    store = Agentkit::Flow.shared_store
    scope = { tenant_key: run.tenant_key }
    puts "swarm #{swarm_id} (flow run #{run.run_id})"
    seen = {}
    loop do
      # uncached: the same query every few seconds must see new rows, not the
      # query cache (it did, and the progress looked frozen while agents worked).
      finished = ActiveRecord::Base.uncached do
        AgentRun.where(swarm_id: swarm_id).order(:id).each do |r|
          line = "#{r.agent} #{r.task} #{r.status} #{r.last_note}"
          puts "  #{line}" unless seen[r.id] == line
          seen[r.id] = line
        end
        store.find_run_by_uuid(run.run_id, scope: scope).finished?
      end
      break if finished

      sleep 5
    end
    current = ActiveRecord::Base.uncached { store.find_run_by_uuid(run.run_id, scope: scope) }
    puts "flow #{current.status}"
    AgentRun.where(swarm_id: swarm_id).order(:agent, :id).each do |r|
      puts format("%-10s %-6s %-8s %d attempts  %6d tokens", r.agent, r.task, r.status, r.attempts, r.input_tokens + r.output_tokens)
    end
  end
end
