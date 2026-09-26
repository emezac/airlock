require "test_helper"
require "tmpdir"

# Real git, a scripted model and a local runner: the loop, not the model, is under test.
class Airlock::WorkerTest < ActiveSupport::TestCase
  CHECK = "test -f feature.txt && ! grep -rq BUG --include=*.txt ."

  class ScriptedModel
    attr_reader :calls

    def initialize(*replies)
      @replies = replies
      @calls = []
    end

    def chat(model:, messages:, **)
      @calls << messages.dup
      Airlock::TokenFactory::Reply.new(content: @replies.shift, model: model, input_tokens: 100, output_tokens: 20)
    end
  end

  setup do
    @dir = Dir.mktmpdir
    @bare = File.join(@dir, "frameline.git")
    sh "git init -q --bare -b main #{@bare}"
    seed = File.join(@dir, "seed")
    sh "git init -q -b main #{seed}"
    File.write(File.join(seed, "README.md"), "# Frameline\n")
    sh "cd #{seed} && git -c user.email=t@t -c user.name=t commit -qm seed --allow-empty && git add -A && " \
       "git -c user.email=t@t -c user.name=t commit -qm readme && git push -q #{@bare} main"
    @assignment = Airlock::Assignment.new(id: "FL-1", title: "Add the feature", instructions: "Create feature.txt.",
                                          context: ["README.md"], check: CHECK)
  end

  teardown { FileUtils.rm_rf(@dir) }

  def sh(cmd) = assert(system(cmd), cmd)

  def worker(model)
    ws = Airlock::GitWorkspace.new(remote: @bare, path: File.join(@dir, "ws"), author: "agent-7")
    Airlock::Worker.new(agent: "agent-7", workspace: ws, runner: Airlock::Runners::Local.new(command: "true", timeout: 30),
                        model: model, policy: Airlock::Policy.new, update_goldens_command: "mkdir -p tests/golden && touch tests/golden/a.png")
  end

  def reply(summary, *blocks, goldens: "no") = "SUMMARY: #{summary}\nUPDATE_GOLDENS: #{goldens}\n\n#{blocks.join("\n")}"
  def write(path, content) = "#{path}\n<<<<<<< SEARCH\n=======\n#{content}>>>>>>> REPLACE\n"
  def edit(path, search, replace) = "#{path}\n<<<<<<< SEARCH\n#{search}\n=======\n#{replace}\n>>>>>>> REPLACE\n"

  def pushed_message = `git --git-dir=#{@bare} log -1 --format=%B agents/agent-7/fl-1`

  test "a passing change is pushed as one commit with evidence" do
    model = ScriptedModel.new(reply("Adds the feature.", write("feature.txt", "ok\n")))
    result = worker(model).run(@assignment)

    assert result.pushed?, result.log.join("\n")
    assert_equal 1, result.attempts
    assert_equal "1", `git --git-dir=#{@bare} rev-list --count main..agents/agent-7/fl-1`.strip
    assert_match %(Airlock-Evidence: sandbox=local checkpoint=none cmd="#{CHECK}"), pushed_message
    assert_match "Airlock-Task: FL-1", pushed_message
    assert_includes model.calls.first.last[:content], "# Frameline"
  end

  test "every step is reported as a floor event" do
    events = []
    model = ScriptedModel.new(reply("First try.", write("feature.txt", "BUG\n")), reply("Fix.", edit("feature.txt", "BUG", "ok")))
    worker(model).run(@assignment, on_event: ->(kind, text, data) { events << [kind, data[:attempt]] })
    assert_equal [["agent.thinking", 1], ["agent.editing", 1], ["agent.testing", 1], ["agent.check_failed", 1],
                  ["agent.thinking", 2], ["agent.editing", 2], ["agent.testing", 2], ["agent.pushing", 2]], events
    assert(events.all? { |k, _| FloorEvent::KINDS.include?(k) })
  end

  test "a failing check goes back to the model with the output" do
    model = ScriptedModel.new(reply("First try.", write("feature.txt", "BUG\n")), reply("Fix.", edit("feature.txt", "BUG", "ok")))
    result = worker(model).run(@assignment)

    assert result.pushed?
    assert_equal 2, result.attempts
    assert_match "failed", model.calls.last.last[:content]
    assert_match "### feature.txt", model.calls.last.last[:content]
    assert_equal "ok\n", `git --git-dir=#{@bare} show agents/agent-7/fl-1:feature.txt`
  end

  test "invalid edits are refused and explained" do
    model = ScriptedModel.new(reply("Bad.", edit("README.md", "nope", "x")), reply("Good.", write("feature.txt", "ok\n")))
    result = worker(model).run(@assignment)

    assert result.pushed?
    assert_match "not applied", model.calls.last.last[:content]
    assert_match "### README.md", model.calls.last.last[:content]
  end

  test "the gate's refusal is fed back" do
    hook = File.join(@bare, "hooks/pre-receive")
    File.write(hook, "#!/bin/sh\nwhile read o n r; do git show $n:feature.txt | grep -q secret && " \
                     "{ echo 'airlock: rejected: secret in feature.txt' >&2; exit 1; }; done; exit 0\n")
    FileUtils.chmod("+x", hook)
    model = ScriptedModel.new(reply("Leaky.", write("feature.txt", "secret\n")), reply("Clean.", edit("feature.txt", "secret", "ok")))
    result = worker(model).run(@assignment)

    assert result.pushed?
    assert_equal 2, result.attempts
    assert_match "rejected: secret", model.calls.last.last[:content]
  end

  test "golden images are regenerated when the model asks" do
    model = ScriptedModel.new(reply("New board.", write("feature.txt", "ok\n"), goldens: "yes"))
    result = worker(model).run(@assignment)

    assert result.pushed?
    assert_includes `git --git-dir=#{@bare} ls-tree -r --name-only agents/agent-7/fl-1`, "tests/golden/a.png"
  end

  test "gives up after the attempt budget without pushing" do
    first = reply("Buggy.", write("feature.txt", "BUG\n"))
    again = reply("Still buggy.", edit("feature.txt", "BUG", "BUG BUG"))
    result = worker(ScriptedModel.new(first, *[again] * 5)).run(@assignment)

    assert_equal "gave_up", result.status
    assert_empty `git --git-dir=#{@bare} branch --list 'agents/*'`.strip
    assert_equal 600, result.input_tokens
  end
end
