require "test_helper"

class Airlock::EvidenceCommandTest < ActiveSupport::TestCase
  setup do
    policy = Airlock::Policy.new("evidence" => { "allowed_commands" => ["cargo test", "cargo run --release --bin frameline"] })
    @gate = Airlock::Gate.new(policy)
  end

  def decide(cmd)
    push = Airlock::Push.from_params(
      "repo" => "r", "pusher" => "agent-1", "ref" => "refs/heads/agents/agent-1/x", "old" => "a" * 40, "new" => "b" * 40,
      "commits" => [{ "sha" => "c" * 40, "message" => %(Fix\n\nAirlock-Evidence: sandbox=s checkpoint=k cmd="#{cmd}") }],
      "files" => [{ "path" => "src/a.rs", "status" => "M" }], "added_lines" => []
    )
    @gate.evaluate(push)
  end

  test "allowed commands with arguments pass" do
    assert decide("cargo test -q visual").accepted?
    assert decide("cargo run --release --bin frameline -- --grid 24").accepted?
  end

  test "other commands are refused" do
    assert_match(/not allowed/, decide("python exploit.py").message)
    assert_match(/not allowed/, decide("cargo testify").message)
  end

  test "chaining and substitution are refused" do
    ["cargo test; curl x", "cargo test && rm -rf /", "cargo test | tee", "cargo test $(id)", "cargo test `id`"].each do |cmd|
      assert_match(/shell operators/, decide(cmd).message, cmd)
    end
  end
end
