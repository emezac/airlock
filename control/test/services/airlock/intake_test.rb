require "test_helper"

class Airlock::IntakeTest < ActiveSupport::TestCase
  class FakeLabeler
    def initialize(model_labels) = @model = model_labels
    def call(push)
      Airlock::Labeler.new(Airlock::Policy.new({}), client: nil).rule_labels(push) +
        @model.map { |c, v| Airlock::Labeler::Label.new(category: c, value: v, source: "model", confidence: 1.0, reason: "fake") }
    end
  end

  def push(files, message: "Fix caption")
    Airlock::Push.from_params("repo" => "frameline", "pusher" => "agent-2", "ref" => "refs/heads/agents/agent-2/x",
                              "old" => "a" * 40, "new" => "b" * 40, "commits" => [{ "sha" => "c" * 40, "message" => message }],
                              "files" => files.map { |p| { "path" => p, "status" => "M" } }, "added_lines" => [])
  end

  def route(files, review: [], model: { "goal_clarity" => "clear", "safety_flag" => "none" }, message: "Fix caption")
    policy = Airlock::Policy.new({})
    decision = GateDecision.create!(repo: "frameline", ref: "refs/heads/agents/agent-2/x", pusher: "agent-2",
                                    verdict: "accept", review: review.any? ? "human" : "auto", review_reasons: review)
    p = push(files, message: message)
    change = Airlock::Intake.admit(policy, p, decision, p.to_h)
    Airlock::Intake.new(policy, labeler: FakeLabeler.new(model)).route!(change, p)
  end

  test "a low-risk change goes to the merge queue with its labels" do
    change = route(["src/films/boat.rs", "tests/boat.rs"])
    assert_equal "queued", change.state
    assert_equal "tests_modified", change.label("test_signal")
    assert_equal "clear", change.label("goal_clarity")
  end

  test "sensitive paths from the gate go to a human" do
    assert_equal "needs_review", route(["billing/credits.rs"], review: ["sensitive path billing/credits.rs"]).state
  end

  test "routing labels send the change to a human" do
    change = route(["src/films/boat.rs", "tests/boat.rs"], model: { "goal_clarity" => "vague" })
    assert_equal "needs_review", change.state
    assert_includes change.review_reasons, "label goal_clarity:vague"
    weakened = route(["tests/visual.rs"], message: "Relax visual threshold")
    assert_includes weakened.review_reasons, "label test_signal:tests_weakened"
  end

  test "high risk above the threshold goes to a human" do
    change = route(Array.new(30) { |i| "src/core/f#{i}.rs" })
    assert_equal "needs_review", change.state
    assert_match(/above threshold/, change.review_reasons.join)
  end
end
