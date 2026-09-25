require "test_helper"

class Airlock::GateTest < ActiveSupport::TestCase
  EVIDENCE = %(Airlock-Evidence: sandbox=sb-1 checkpoint=ck-7 cmd="cargo test -q")

  setup do
    @policy = Airlock::Policy.new(
      "gate" => { "protected_refs" => ["refs/heads/main"], "merge_identity" => "airlock", "agent_prefix" => "agent-" },
      "paths" => { "forbidden" => ["infra/"], "human_review" => ["billing/", "Cargo.toml"] }
    )
    @gate = Airlock::Gate.new(@policy)
  end

  def push(pusher: "agent-7", ref: "refs/heads/agents/agent-7/fix-caption", message: "Fix caption overflow\n\n#{EVIDENCE}",
           files: [["src/render/captions.rs", "M"]], added: [["src/render/captions.rs", "let w = width * 0.9;"]], new_sha: "b" * 40)
    Airlock::Push.from_params(
      "repo" => "frameline", "pusher" => pusher, "ref" => ref, "old" => "a" * 40, "new" => new_sha,
      "commits" => message ? [{ "sha" => "c" * 40, "author" => "#{pusher}@airlock", "message" => message }] : [],
      "files" => files.map { |path, status| { "path" => path, "status" => status } },
      "added_lines" => added.map { |path, text| { "path" => path, "text" => text } }
    )
  end

  test "accepts an agent change with evidence on its own branch" do
    decision = @gate.evaluate(push)
    assert decision.accepted?, decision.message
    assert_equal "auto", decision.review
  end

  test "rejects agent pushes to main" do
    decision = @gate.evaluate(push(ref: "refs/heads/main"))
    refute decision.accepted?
    assert_match(/protected/, decision.message)
  end

  test "rejects humans pushing to main too; only the merge queue may" do
    refute @gate.evaluate(push(pusher: "enrique", ref: "refs/heads/main")).accepted?
    assert @gate.evaluate(push(pusher: "airlock", ref: "refs/heads/main")).accepted?
  end

  test "rejects an agent pushing to another agent's branch" do
    decision = @gate.evaluate(push(ref: "refs/heads/agents/agent-9/x"))
    assert_match(%r{only push to refs/heads/agents/agent-7/}, decision.message)
  end

  test "rejects forbidden paths" do
    decision = @gate.evaluate(push(files: [["infra/deploy.yml", "M"]]))
    assert_match(%r{forbidden path: infra/deploy.yml}, decision.message)
  end

  test "forbidden paths bind agents, not the humans who write the policy" do
    assert @gate.evaluate(push(pusher: "enrique", ref: "refs/heads/ops", message: "ops", files: [["infra/deploy.yml", "M"]])).accepted?
  end

  test "rejects commits without valid evidence" do
    refute @gate.evaluate(push(message: "Fix caption overflow")).accepted?
    refute @gate.evaluate(push(message: "Fix\n\nAirlock-Evidence: sandbox=sb-1 checkpoint=ck-7")).accepted?
    refute @gate.evaluate(push(message: nil)).accepted?
  end

  test "humans do not need evidence trailers on their own branches" do
    assert @gate.evaluate(push(pusher: "enrique", ref: "refs/heads/feature", message: "WIP")).accepted?
  end

  test "rejects added secrets" do
    decision = @gate.evaluate(push(added: [["config/voice.toml", %(api_key = "sk-live-0123456789abcdefghijklmn")]]))
    assert_match(/possible secret added in config\/voice.toml/, decision.message)
  end

  test "rejects deleted tests and new skip markers" do
    assert_match(/test file deleted/, @gate.evaluate(push(files: [["tests/visual.rs", "D"]])).message)
    assert_match(/skip marker/, @gate.evaluate(push(added: [["tests/visual.rs", "#[ignore]"]])).message)
  end

  test "sensitive paths are accepted but routed to a human" do
    decision = @gate.evaluate(push(files: [["billing/credits.rs", "M"], ["Cargo.toml", "M"]]))
    assert decision.accepted?
    assert_equal "human", decision.review
    assert_equal ["sensitive path billing/credits.rs", "sensitive path Cargo.toml"], decision.review_reasons
  end

  test "branch deletions skip content checks but not ref rules" do
    assert @gate.evaluate(push(new_sha: "0" * 40, message: nil, files: [], added: [])).accepted?
    refute @gate.evaluate(push(ref: "refs/heads/main", new_sha: "0" * 40, message: nil, files: [], added: [])).accepted?
  end

  test "same input gives the same decision" do
    assert_equal @gate.evaluate(push), @gate.evaluate(push)
  end
end
