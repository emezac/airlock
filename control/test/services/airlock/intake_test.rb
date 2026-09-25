require "test_helper"

class Airlock::IntakeTest < ActiveSupport::TestCase
  class FakeClassifier
    def initialize(result) = @result = result
    def classify(**) = @result
  end

  def push(files)
    Airlock::Push.from_params("repo" => "frameline", "pusher" => "agent-2", "ref" => "refs/heads/agents/agent-2/x",
                              "old" => "a" * 40, "new" => "b" * 40, "commits" => [{ "sha" => "c" * 40, "message" => "m" }],
                              "files" => files.map { |p| { "path" => p, "status" => "M" } }, "added_lines" => [])
  end

  def route(files, review: [], classification: Airlock::Classifier::UNKNOWN)
    policy = Airlock::Policy.new({})
    decision = GateDecision.create!(repo: "frameline", ref: "refs/heads/agents/agent-2/x", pusher: "agent-2",
                                    verdict: "accept", review: review.any? ? "human" : "auto", review_reasons: review)
    change = Airlock::Intake.admit(policy, push(files), decision)
    Airlock::Intake.new(policy, classifier: FakeClassifier.new(classification)).route!(change, push(files))
  end

  test "a low-risk change goes to the merge queue" do
    assert_equal "queued", route(["src/films/boat.rs", "tests/boat.rs"]).state
  end

  test "sensitive paths from the gate go to a human" do
    change = route(["billing/credits.rs"], review: ["sensitive path billing/credits.rs"])
    assert_equal "needs_review", change.state
    assert change.review_requested_at
  end

  test "a vague goal sends the change to a human" do
    unsure = Airlock::Classifier::UNKNOWN.with(source: "nano", clarity: 0.2)
    change = route(["src/films/boat.rs", "tests/boat.rs"], classification: unsure)
    assert_equal "needs_review", change.state
    assert_match(/goal is unclear/, change.review_reasons.join)
  end

  test "high risk above the threshold goes to a human" do
    change = route(Array.new(30) { |i| "src/core/f#{i}.rs" } + ["Cargo.toml"])
    assert_equal "needs_review", change.state
    assert_match(/above threshold/, change.review_reasons.join)
  end
end
