require "test_helper"
require "tmpdir"

# The verifier checks out the pushed branch and re-runs the cited command.
# Here a local runner stands in for the sandbox.
class Airlock::EvidenceVerifierTest < ActiveSupport::TestCase
  setup do
    @dir = Dir.mktmpdir
    @bare = File.join(@dir, "r.git")
    work = File.join(@dir, "w")
    system("git init -q --bare -b main #{@bare} && git init -q -b main #{work} && cd #{work} && " \
           "git config user.email t@t && git config user.name t && echo ok > status && git add -A && git commit -qm seed && " \
           "git push -q #{@bare} main && git checkout -qb good && git push -q #{@bare} good:refs/heads/agents/agent-1/good && " \
           "echo fail > status && git commit -qam break && git push -q #{@bare} HEAD:refs/heads/agents/agent-1/bad", exception: true)
    policy = Airlock::Policy.new({})
    workspace = Airlock::GitWorkspace.new(remote: @bare, path: File.join(@dir, "ws"))
    @verifier = Airlock::EvidenceVerifier.new(policy, workspace: workspace,
                                              runner: Airlock::Runners::Local.new(command: "true", timeout: 10))
  end

  teardown { FileUtils.rm_rf(@dir) }

  def outcome(branch)
    change = Change.new(ref: "refs/heads/agents/agent-1/#{branch}")
    push = Airlock::Push.from_params("repo" => "r", "pusher" => "agent-1", "ref" => change.ref, "old" => "", "new" => "b",
                                     "commits" => [{ "sha" => "c", "message" => %(x\n\nAirlock-Evidence: sandbox=s checkpoint=k cmd="grep -qx ok status") }],
                                     "files" => [], "added_lines" => [])
    @verifier.verify(change, push)
  end

  test "evidence that holds on the pushed commit is verified" do
    assert_equal "verified", outcome("good").status
  end

  test "evidence that does not hold on the pushed commit fails" do
    assert_equal "failed", outcome("bad").status
  end
end
