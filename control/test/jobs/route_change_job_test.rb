require "test_helper"

class RouteChangeJobTest < ActiveJob::TestCase
  Outcome = Airlock::EvidenceVerifier::Outcome

  class StubVerifier
    def initialize(outcome) = @outcome = outcome
    def verify(*) = @outcome
  end

  setup do
    @report = { "repo" => "frameline", "pusher" => "agent-5", "ref" => "refs/heads/agents/agent-5/x", "old" => "a" * 40,
                "new" => "b" * 40, "commits" => [{ "sha" => "c" * 40, "message" => "Fix" }],
                "files" => [{ "path" => "src/films/boat.rs", "status" => "M" }, { "path" => "tests/boat.rs", "status" => "M" }],
                "added_lines" => [] }
    decision = GateDecision.create!(repo: "frameline", ref: @report["ref"], pusher: "agent-5", verdict: "accept", review: "auto")
    @change = Change.create!(repo: "frameline", ref: @report["ref"], pusher: "agent-5", head_sha: "b" * 40,
                             gate_decision: decision, state: "pending", report: @report)
  end

  def perform_with(outcome)
    verifier = outcome && StubVerifier.new(outcome)
    original = Airlock::QueueRunner.method(:verifier)
    Airlock::QueueRunner.define_singleton_method(:verifier) { |*, **| verifier }
    Airlock::Labeler.stub_any_instance_model_free { RouteChangeJob.perform_now(@change.id, @report) }
    @change.reload
  ensure
    Airlock::QueueRunner.define_singleton_method(:verifier, original)
  end

  test "evidence that does not reproduce rejects the change and counts as an agent failure" do
    perform_with(Outcome.new(status: "failed", command: "cargo test", checkpoint: "img", output: "1 failed"))
    assert_equal "rejected", @change.state
    assert_equal ["evidence did not reproduce: cargo test"], @change.review_reasons
    assert_operator AgentStat.rate_for("agent-5"), :>, AgentStat::PRIOR
  end

  test "verified evidence lets the change be labelled and routed" do
    perform_with(Outcome.new(status: "verified", command: "cargo test", checkpoint: "img", output: ""))
    assert_equal %w[verified queued], [@change.evidence_status, @change.state]
    assert_enqueued_with(job: MergeQueueJob, args: ["frameline"])
  end

  test "without a sandbox the check is recorded as skipped, never run locally" do
    perform_with(nil)
    assert_equal "skipped", @change.evidence_status
  end
end
