require "test_helper"
require "tmpdir"

# Real git repositories; the CI command checks the tree like a test suite would.
class Airlock::MergeQueueTest < ActiveSupport::TestCase
  CI = %(test ! -e BROKEN && ! (test -e a.txt && test -e b.txt && grep -q red a.txt && grep -q red b.txt))

  setup do
    @dir = Dir.mktmpdir
    @bare = File.join(@dir, "frameline.git")
    sh "git init -q --bare -b main #{@bare}"
    @work = File.join(@dir, "seed")
    sh "git init -q -b main #{@work}"
    git "config user.email t@t && git config user.name t"
    File.write(File.join(@work, "README"), "seed\n")
    git "add -A && git commit -qm seed && git remote add origin #{@bare} && git push -q origin main"
    @policy = Airlock::Policy.new("ci" => { "command" => CI, "timeout_seconds" => 30 }, "merge_queue" => { "max_batch" => 9 })
    workspace = Airlock::GitWorkspace.new(remote: @bare, path: File.join(@dir, "ws"))
    @queue = Airlock::MergeQueue.new(@policy, workspace: workspace,
                                     runner: Airlock::Runners::Local.new(command: CI, timeout: 30))
    AgentStat.create!(agent: "agent-1", failure_rate: 0.05)
  end

  teardown { FileUtils.rm_rf(@dir) }

  def sh(cmd) = assert(system(cmd), cmd)
  def git(cmd) = sh("cd #{@work} && git #{cmd}")

  # Creates an agent branch with one file and a queued Change for it.
  def branch(name, file, content = name)
    git "checkout -q main && git checkout -qb #{name}"
    File.write(File.join(@work, file), "#{content}\n")
    git "add -A && git commit -qm #{name} && git push -q origin #{name}:refs/heads/agents/agent-1/#{name} && git checkout -q main"
    decision = GateDecision.create!(repo: "frameline", ref: "refs/heads/agents/agent-1/#{name}", pusher: "agent-1", verdict: "accept")
    Change.create!(repo: "frameline", ref: decision.ref, pusher: "agent-1", head_sha: "x", gate_decision: decision, state: "queued")
  end

  def main_files = `git --git-dir=#{@bare} ls-tree --name-only main`.split

  test "a clean batch merges in one CI run" do
    changes = %w[one two three].map { |n| branch(n, "#{n}.txt") }
    batch = @queue.run_once("frameline")
    assert_equal "green", batch.state
    assert_equal 1, batch.ci_runs
    assert_equal %w[merged] * 3, changes.map { |c| c.reload.state }
    assert_includes main_files, "three.txt"
  end

  test "bisection isolates the one broken change" do
    good = %w[a1 a2 a3].map { |n| branch(n, "#{n}.txt") }
    bad = branch("bad", "BROKEN")
    batch = @queue.run_once("frameline")
    assert_equal "red", batch.state
    assert_equal "failed", bad.reload.state
    assert(good.all? { |c| c.reload.state == "merged" })
    refute_includes main_files, "BROKEN"
    # 5 runs of test-and-bisect plus 1 to confirm the surviving union together.
    assert_equal 6, batch.ci_runs
    assert_operator AgentStat.rate_for("agent-1"), :>, 0.05
  end

  test "two changes that pass alone but fail together are caught" do
    first = branch("left", "a.txt", "red")
    second = branch("right", "b.txt", "red")
    @queue.run_once("frameline")
    states = [first.reload.state, second.reload.state]
    assert_equal %w[failed merged].sort, states.sort, "exactly one of the clashing pair merges"
  end

  test "a conflicting change is marked as a conflict" do
    one = branch("x1", "same.txt", "one")
    two = branch("x2", "same.txt", "two")
    @queue.run_once("frameline")
    assert_equal %w[conflict merged], [one.reload.state, two.reload.state].sort
  end
end
