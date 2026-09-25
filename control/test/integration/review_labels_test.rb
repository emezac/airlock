require "test_helper"

class ReviewLabelsTest < ActionDispatch::IntegrationTest
  AUTH = { "Authorization" => "Bearer rev", "X-Reviewer" => "enrique" }.freeze

  setup do
    ENV["AIRLOCK_REVIEW_TOKEN"] = "rev"
    report = { "repo" => "frameline", "pusher" => "agent-4", "ref" => "refs/heads/agents/agent-4/x", "old" => "a" * 40,
               "new" => "b" * 40, "commits" => [{ "sha" => "c" * 40, "message" => "Fix" }],
               "files" => [{ "path" => "src/films/boat.rs", "status" => "M" }, { "path" => "tests/boat.rs", "status" => "M" }],
               "added_lines" => [] }
    decision = GateDecision.create!(repo: "frameline", ref: report["ref"], pusher: "agent-4", verdict: "accept", review: "auto")
    @change = Change.create!(repo: "frameline", ref: report["ref"], pusher: "agent-4", head_sha: "b" * 40,
                             gate_decision: decision, state: "queued", report: report)
    @change.labels.create!(category: "safety_flag", value: "none", source: "model", confidence: 1.0)
  end

  teardown { ENV.delete("AIRLOCK_REVIEW_TOKEN") }

  test "a human override is validated, recorded and re-routes the change" do
    post "/reviews/#{@change.id}/label", params: { category: "safety_flag", value: "touches_money", reason: "charges credits" },
                                         headers: AUTH, as: :json
    assert_response :ok
    body = response.parsed_body
    assert_equal "needs_review", body["state"]
    assert_equal({ "value" => "touches_money", "source" => "override", "confidence" => 1.0, "reason" => "charges credits" },
                 body["labels"]["safety_flag"])
    audit = LabelOverride.last
    assert_equal ["none", "touches_money", "changed", "enrique"], [audit.previous_value, audit.new_value, audit.action, audit.reviewer]
  end

  test "values outside the vocabulary are refused" do
    post "/reviews/#{@change.id}/label", params: { category: "safety_flag", value: "probably_fine" }, headers: AUTH, as: :json
    assert_response :unprocessable_entity
    assert_equal 0, LabelOverride.count
  end
end
