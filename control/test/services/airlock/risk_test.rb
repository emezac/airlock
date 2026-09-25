require "test_helper"

class Airlock::RiskTest < ActiveSupport::TestCase
  def push(files, lines: 10)
    Airlock::Push.from_params("repo" => "r", "pusher" => "agent-1", "ref" => "refs/heads/agents/agent-1/x",
                              "old" => "a" * 40, "new" => "b" * 40, "commits" => [],
                              "files" => files.map { |p| { "path" => p, "status" => "M" } },
                              "added_lines" => Array.new(lines) { { "path" => files.first, "text" => "x" } })
  end

  CALM = { "blast_radius" => "single_area", "test_signal" => "tests_modified", "dependency" => "none",
           "goal_clarity" => "clear", "safety_flag" => "none" }.freeze

  setup { @risk = Airlock::Risk.new }

  test "a small, tested, clear change from a reliable agent is low risk" do
    assert_operator @risk.score(push(["src/films/boat.rs"]), agent_failure_rate: 0.0, labels: CALM).value, :<, 0.1
  end

  test "each risky label can only raise the risk" do
    base = @risk.score(push(["a"]), agent_failure_rate: 0.1, labels: CALM).value
    [%w[blast_radius core], %w[test_signal tests_weakened], %w[dependency added_or_changed],
     %w[goal_clarity vague], %w[safety_flag touches_money]].each do |category, value|
      raised = @risk.score(push(["a"]), agent_failure_rate: 0.1, labels: CALM.merge(category => value)).value
      assert_operator raised, :>, base, "#{category}=#{value}"
    end
  end

  test "unknown or missing labels contribute nothing" do
    assert_equal @risk.score(push(["a"]), agent_failure_rate: 0.1, labels: {}).value,
                 @risk.score(push(["a"]), agent_failure_rate: 0.1, labels: CALM).value
  end

  test "same labels, same score" do
    a = @risk.score(push(["a"]), agent_failure_rate: 0.2, labels: CALM)
    assert_equal a, @risk.score(push(["a"]), agent_failure_rate: 0.2, labels: CALM)
  end
end
