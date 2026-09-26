require "test_helper"
require "tmpdir"

class Airlock::SwarmTest < ActiveSupport::TestCase
  # Every agent answers every task by creating a file named after the task.
  class TaskModel
    def initialize(task) = @task = task

    def chat(model:, **)
      body = "SUMMARY: Adds #{@task}.\nUPDATE_GOLDENS: no\n\n#{@task}.txt\n<<<<<<< SEARCH\n=======\nok\n>>>>>>> REPLACE\n"
      Airlock::TokenFactory::Reply.new(content: body, model: model, input_tokens: 10, output_tokens: 5)
    end
  end

  setup do
    @dir = Dir.mktmpdir
    FileUtils.mkdir_p(File.join(@dir, "git"))
    bare = File.join(@dir, "git/frameline.git")
    system("git init -q --bare -b main #{bare} && git init -q -b main #{@dir}/seed && cd #{@dir}/seed && " \
           "git -c user.email=t@t -c user.name=t commit -q --allow-empty -m seed && git push -q #{bare} main", exception: true)
    @backlog = File.join(@dir, "backlog.yml")
    tasks = %w[T-1 T-2 T-3].map { |id| { "id" => id, "title" => "Task #{id}", "instructions" => "Do #{id}.", "check" => "true" } }
    File.write(@backlog, { "tasks" => tasks }.to_yaml)
    @env = ENV.to_h.slice("AIRLOCK_GIT_ROOT", "AIRLOCK_WORK_ROOT", "AIRLOCK_URL", "AIRLOCK_PUSH_URL")
    ENV["AIRLOCK_GIT_ROOT"] = File.join(@dir, "git")
    ENV["AIRLOCK_WORK_ROOT"] = File.join(@dir, "ws")
    ENV.delete("AIRLOCK_PUSH_URL")
    Airlock::Swarm.model_factory = ->(_agent, task) { TaskModel.new(task) }
    Airlock::Swarm.runner_factory = -> { Airlock::Runners::Local.new(command: "true", timeout: 10) }
  end

  teardown do
    Airlock::Swarm.model_factory = nil
    Airlock::Swarm.runner_factory = nil
    %w[AIRLOCK_GIT_ROOT AIRLOCK_WORK_ROOT AIRLOCK_URL AIRLOCK_PUSH_URL].each { |k| @env[k] ? ENV[k] = @env[k] : ENV.delete(k) }
    FileUtils.rm_rf(@dir)
  end

  def branches = `git --git-dir=#{@dir}/git/frameline.git branch --list 'agents/*' --format='%(refname:short)'`.split

  test "plan spreads tasks round-robin and skips work already done" do
    decision = GateDecision.create!(repo: "frameline", ref: "refs/heads/agents/agent-x/t-2", pusher: "agent-x", verdict: "accept")
    Change.create!(repo: "frameline", ref: decision.ref, pusher: "agent-x", head_sha: "x", gate_decision: decision, state: "merged")

    plan = Airlock::Swarm.plan(repo: "frameline", agents: %w[agent-a agent-b], backlog: @backlog, swarm_id: "s1")
    assert_equal({ "agent-a" => %w[T-1], "agent-b" => %w[T-3] }, plan.to_h { |i| [i["agent"], i["tasks"]] })
    assert(plan.all? { |i| i["swarm_id"] == "s1" })
  end

  test "work runs an agent's tasks in order, records them, and is safe to redeliver" do
    item = { "repo" => "frameline", "agent" => "agent-a", "tasks" => %w[T-1 T-3], "backlog" => @backlog, "swarm_id" => "s2" }
    rows = Airlock::Swarm.work(item)

    assert_equal %w[pushed pushed], rows.map { |r| r["status"] }
    assert_equal %w[agents/agent-a/t-1 agents/agent-a/t-3], branches
    run = AgentRun.find_by!(swarm_id: "s2", task: "T-1")
    assert_equal 1, run.attempts
    assert_match "running true", run.log.join("\n")

    Airlock::Swarm.model_factory = ->(*) { raise "a finished task must not run again" }
    assert_equal rows, Airlock::Swarm.work(item)
  end

  test "an error in one task does not stop the agent's next task" do
    Airlock::Swarm.model_factory = ->(_agent, task) { task == "T-1" ? raise("model down") : TaskModel.new(task) }
    item = { "repo" => "frameline", "agent" => "agent-a", "tasks" => %w[T-1 T-2], "backlog" => @backlog, "swarm_id" => "s3" }
    assert_equal %w[error pushed], Airlock::Swarm.work(item).map { |r| r["status"] }
    assert_match "model down", AgentRun.find_by!(swarm_id: "s3", task: "T-1").last_note
  end

  test "the flow plans, works every agent's share and reports" do
    result = SwarmFlow.call(repo: "frameline", agents: %w[agent-a agent-b], backlog: @backlog, swarm_id: "s4")

    assert result.ok?, result.error.to_s
    report = result.value
    assert_equal 3, report["tasks"]
    assert_equal({ "pushed" => 3 }, report["by_status"])
    assert_equal 30, report["tokens"]["input"]
    assert_equal 3, branches.size
  end

  def failed_change(task, next_step, category: "compile_error")
    decision = GateDecision.create!(repo: "frameline", ref: "refs/heads/agents/agent-x/#{task.downcase}", pusher: "agent-x", verdict: "accept")
    Change.create!(repo: "frameline", ref: decision.ref, pusher: "agent-x", head_sha: "x", gate_decision: decision, state: "failed",
                   diagnosis: { "category" => category, "summary" => "render_scene_tile is missing", "next_step" => next_step,
                                "culprit_files" => ["src/board.rs"] })
  end

  test "a task whose failure needs a person is held; one the agent can fix is retried with the diagnosis" do
    failed_change("T-1", "human", category: "visual_regression")
    failed_change("T-2", "agent_retry")
    plan = Airlock::Swarm.plan(repo: "frameline", agents: %w[agent-a], backlog: @backlog, swarm_id: "s5")
    assert_equal %w[T-2 T-3], plan.first["tasks"]

    seen = nil
    Airlock::Swarm.model_factory = lambda do |_agent, task|
      model = TaskModel.new(task)
      model.define_singleton_method(:chat) { |**kw| seen ||= kw[:messages].last[:content]; super(**kw) }
      model
    end
    Airlock::Swarm.work(plan.first.merge("tasks" => %w[T-2]))
    assert_match "failed in the merge queue", seen
    assert_match "render_scene_tile is missing", seen
    assert_match "src/board.rs", seen
  end
end
