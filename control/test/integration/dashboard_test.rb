require "test_helper"

class DashboardTest < ActionDispatch::IntegrationTest
  setup do
    decision = GateDecision.create!(repo: "frameline", ref: "refs/heads/agents/agent-1/pay", pusher: "agent-1", verdict: "accept")
    change = Change.create!(repo: "frameline", ref: decision.ref, pusher: "agent-1", head_sha: "b", gate_decision: decision,
                            state: "needs_review", risk_score: 0.61, review_reasons: ["label safety_flag:touches_money"])
    change.labels.create!(category: "safety_flag", value: "touches_money", source: "model", confidence: 1.0)
    MergeBatch.create!(repo: "frameline", state: "red", size: 4, ci_runs: 5)
  end

  test "renders the panel with KPIs, cards and batches" do
    get "/dashboard"
    assert_response :ok
    assert_includes response.body, "Human load ρ"
    assert_includes response.body, "pay"
    assert_includes response.body, "touches_money"
    assert_includes response.body, "label safety_flag:touches_money"
  end

  test "serves the same snapshot as JSON" do
    get "/dashboard.json"
    body = response.parsed_body
    assert_equal "frameline", body["repo"]
    assert_equal "pay", body.dig("columns", "review", 0, "branch")
    assert_equal({ "value" => "touches_money", "source" => "model" }, body.dig("columns", "review", 0, "labels", "safety_flag"))
    assert_equal "red", body.dig("batches", 0, "state")
    assert body.dig("batching", "batch_size").positive?
  end

  test "the fragment is the panel without the page shell" do
    get "/dashboard", params: { fragment: 1 }
    refute_includes response.body, "<html"
    assert_includes response.body, "Merge batches"
  end

  test "never exposes stored push reports" do
    Change.last.update!(report: { "added_lines" => [{ "path" => "x", "text" => "SECRET_VALUE" }] })
    get "/dashboard.json"
    refute_includes response.body, "SECRET_VALUE"
  end
end
