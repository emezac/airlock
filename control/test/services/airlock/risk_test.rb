require "test_helper"

class Airlock::RiskTest < ActiveSupport::TestCase
  def push(files, lines: 10)
    Airlock::Push.from_params("repo" => "r", "pusher" => "agent-1", "ref" => "refs/heads/agents/agent-1/x",
                              "old" => "a" * 40, "new" => "b" * 40, "commits" => [],
                              "files" => files.map { |p| { "path" => p, "status" => "M" } },
                              "added_lines" => Array.new(lines) { { "path" => files.first, "text" => "x" } })
  end

  setup do
    @risk = Airlock::Risk.new(Airlock::Policy.new({}), core_paths: ["src/core/"])
    @calm = Airlock::Classifier::UNKNOWN
  end

  test "a small change with tests from a reliable agent is low risk" do
    score = @risk.score(push(["src/films/boat.rs", "tests/boat.rs"]), agent_failure_rate: 0.0, classification: @calm)
    assert_operator score.value, :<, 0.1
  end

  test "each signal can only raise the risk" do
    base = @risk.score(push(["src/films/boat.rs"]), agent_failure_rate: 0.1, classification: @calm).value
    assert_operator @risk.score(push(["src/core/canvas.rs"]), agent_failure_rate: 0.1, classification: @calm).value, :>, base
    assert_operator @risk.score(push(["src/films/boat.rs", "Cargo.toml"]), agent_failure_rate: 0.1, classification: @calm).value, :>, base
    assert_operator @risk.score(push(["src/films/boat.rs"]), agent_failure_rate: 0.6, classification: @calm).value, :>, base
    flagged = @calm.with(block_probability: 0.8, source: "nano")
    assert_operator @risk.score(push(["src/films/boat.rs"]), agent_failure_rate: 0.1, classification: flagged).value, :>, base
  end

  test "risk stays within [0, 1]" do
    worst = @risk.score(push(Array.new(40) { |i| "src/core/f#{i}.rs" } + ["Cargo.toml"], lines: 900),
                        agent_failure_rate: 1.0, classification: @calm.with(block_probability: 1.0))
    assert_equal 1.0, worst.value
  end
end
