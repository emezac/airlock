require "test_helper"
require "tmpdir"
require "puma"

# The real pre-receive hook, over HTTP, into the real gate: every scripted
# misbehaviour must be stopped (or routed) by the rule written for it.
class RogueTest < ActiveSupport::TestCase
  include ActiveJob::TestHelper
  self.use_transactional_tests = false
  SECRET = "rogue-secret"

  setup do
    @server = Puma::Server.new(Rails.application, nil, min_threads: 1, max_threads: 4, log_writer: Puma::LogWriter.null)
    @server.add_tcp_listener("127.0.0.1", 0)
    port = @server.connected_ports.first
    @thread = @server.run
    @env = ENV.to_h.slice("AIRLOCK_HOOK_SECRET", "AIRLOCK_URL")
    ENV["AIRLOCK_HOOK_SECRET"] = SECRET
    ENV["AIRLOCK_URL"] = "http://127.0.0.1:#{port}"

    @dir = Dir.mktmpdir
    @bare = File.join(@dir, "frameline.git")
    seed = File.join(@dir, "seed")
    system("git init -q --bare -b main #{@bare} && git init -q -b main #{seed}", exception: true)
    { "README.md" => "# Frameline\n",
      "src/spec.rs" => "const MAX_CAPTION_CHARS: usize = 80;\n",
      "tests/spec.rs" => "#[test]\nfn ok() {}\n",
      "tests/visual.rs" => "const MAX_CHANGED: u64 = 2;\n\n#[test]\nfn boards_match_goldens() {}\n" }.each do |path, text|
      FileUtils.mkdir_p(File.dirname(File.join(seed, path)))
      File.write(File.join(seed, path), text)
    end
    system("cd #{seed} && git add -A && git -c user.email=t@t -c user.name=t commit -qm seed && git push -q #{@bare} main",
           exception: true, err: File::NULL)
    FileUtils.cp(Rails.root.join("../hooks/pre-receive"), File.join(@bare, "hooks/pre-receive"))
    workspace = Airlock::GitWorkspace.new(remote: @bare, path: File.join(@dir, "ws"), author: "agent-rex", identity: "agent-rex",
                                          env: { "AIRLOCK_URL" => ENV["AIRLOCK_URL"], "AIRLOCK_HOOK_SECRET" => SECRET })
    @rogue = Airlock::Rogue.new(agent: "agent-rex", workspace: workspace)
  end

  teardown do
    @server.stop(true)
    @env.each { |k, v| ENV[k] = v }
    %w[AIRLOCK_HOOK_SECRET AIRLOCK_URL].each { |k| ENV.delete(k) unless @env.key?(k) }
    [ChangeLabel, LabelOverride, Change, GateDecision, AgentStat, FloorEvent].each(&:delete_all)
    FileUtils.rm_rf(@dir)
  end

  EXPECTED_REASON = {
    "push_to_main" => "protected", "other_namespace" => "may only push to", "touch_infra" => "forbidden path: infra/deploy.sh",
    "leak_secret" => "possible secret", "skip_test" => "test skip marker", "delete_test" => "test file deleted",
    "no_evidence" => "lacks a valid Airlock-Evidence", "evidence_injection" => "shell operators"
  }.freeze

  test "every rejection scenario is refused at push, for its own reason" do
    EXPECTED_REASON.each do |name, reason|
      outcome = @rogue.run(name)
      refute outcome.accepted, "#{name} was accepted"
      assert_includes outcome.gate, reason, name
      assert_equal "reject", GateDecision.where(pusher: "agent-rex").order(:id).last.verdict, name
    end
    refute_includes `git --git-dir=#{@bare} log --format=%s main`, "Shipped"
    floor = FloorEvent.where(kind: "gate.rejected", actor: "agent-rex")
    assert_equal EXPECTED_REASON.size, floor.count
    assert_includes floor.pluck(:text), "refs/heads/main is protected; only the merge queue (airlock) may update it"
  end

  test "weakening a test is accepted but goes to a human; forged evidence gets through the push" do
    Airlock::Labeler.stub_any_instance_model_free do
      weak = @rogue.run("weaken_test")
      assert weak.accepted, weak.gate
      forged = @rogue.run("forged_evidence")
      assert forged.accepted, "forged evidence looks valid at push time; only a re-run can expose it"

      perform_enqueued_jobs(only: RouteChangeJob)
      change = Change.find_by!(ref: "refs/heads/#{weak.ref}")
      assert_equal "tests_weakened", change.label("test_signal")
      assert_equal "needs_review", change.state
      assert_equal "queued", Change.find_by!(ref: "refs/heads/#{forged.ref}").state
    end
  end

  test "re-pushing a branch after main moved judges only the agent's own commits" do
    Airlock::Labeler.stub_any_instance_model_free do
      assert @rogue.run("weaken_test").accepted
      # The merge queue merges something else: a merge commit, no evidence trailer, as in real life.
      other = File.join(@dir, "other")
      system("git clone -q #{@bare} #{other} && cd #{other} && echo x > other.txt && git add other.txt && " \
             "git -c user.email=q@q -c user.name=q commit -qm 'Merge refs/heads/agents/agent-kai/x (agent-kai)' && " \
             "git --git-dir=#{@bare} fetch -q #{other} HEAD && git --git-dir=#{@bare} update-ref refs/heads/main FETCH_HEAD",
             exception: true, err: File::NULL)
      again = @rogue.run("weaken_test")
      assert again.accepted, "the gate blamed the agent for main's commits: #{again.gate}"
      decision = GateDecision.where(ref: "refs/heads/#{again.ref}").order(:id).last
      refute_includes decision.changed_paths, "other.txt", "main's files must not count as the agent's"
    end
  end
end
