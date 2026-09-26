namespace :airlock do
  desc "Run one agent on one backlog task: bin/rails airlock:work TASK=FL-1 AGENT=agent-ada [REPO=frameline]"
  task work: :environment do
    backlog = ENV.fetch("AIRLOCK_BACKLOG") { Rails.root.join("../demo/backlog.yml").to_s }
    assignment = Airlock::Assignment.load_all(backlog).find { |a| a.id == ENV.fetch("TASK") } or abort "no task #{ENV['TASK']}"
    agent = ENV.fetch("AGENT")
    repo = ENV.fetch("REPO", "frameline")
    policy = Airlock::Policy.load
    remote = ENV.fetch("AIRLOCK_PUSH_URL") { File.join(ENV.fetch("AIRLOCK_GIT_ROOT"), "#{repo}.git") }
    work = ENV.fetch("AIRLOCK_WORK_ROOT", Rails.root.join("tmp/workspaces").to_s)
    env = { "AIRLOCK_URL" => ENV["AIRLOCK_URL"], "AIRLOCK_HOOK_SECRET" => ENV["AIRLOCK_HOOK_SECRET"] }.compact
    workspace = Airlock::GitWorkspace.new(remote: remote, path: File.join(work, "#{repo}-#{agent}"), author: agent, identity: agent, env: env)
    runner = Airlock::QueueRunner.sandbox_runner(policy) ||
             Airlock::Runners::Local.new(command: policy.ci_command, timeout: policy.ci_timeout)
    worker = Airlock::Worker.new(agent: agent, workspace: workspace, runner: runner, model: Airlock::TokenFactory.new, policy: policy)
    result = worker.run(assignment)
    puts result.log
    puts "#{result.status}: #{result.branch} #{result.sha} · #{result.attempts} attempts · " \
         "#{result.input_tokens} in / #{result.output_tokens} out tokens"
    exit(result.pushed? ? 0 : 1)
  end
end
