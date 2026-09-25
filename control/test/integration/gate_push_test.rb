require "test_helper"

class GatePushTest < ActionDispatch::IntegrationTest
  SECRET = "test-hook-secret"

  setup { ENV["AIRLOCK_HOOK_SECRET"] = SECRET }
  teardown { ENV.delete("AIRLOCK_HOOK_SECRET") }

  def report(ref:, message:)
    { repo: "frameline", pusher: "agent-7", ref: ref, old: "a" * 40, new: "b" * 40,
      commits: [{ sha: "c" * 40, author: "agent-7@airlock", message: message }],
      files: [{ path: "src/lib.rs", status: "M" }], added_lines: [] }
  end

  test "rejects calls without the shared secret" do
    post "/gate/push", params: report(ref: "refs/heads/agents/agent-7/x", message: "x"), as: :json
    assert_response :unauthorized
    assert_equal 0, GateDecision.count
  end

  test "accepts, records and audits a valid push" do
    msg = %(Fix\n\nAirlock-Evidence: sandbox=sb checkpoint=ck cmd="cargo test")
    post "/gate/push", params: report(ref: "refs/heads/agents/agent-7/x", message: msg), as: :json,
                       headers: { "X-Airlock-Hook-Secret" => SECRET }
    assert_response :ok
    assert_equal "accepted", response.body
    row = GateDecision.last
    assert_equal %w[accept auto], [row.verdict, row.review]
    assert_equal ["src/lib.rs"], row.changed_paths
  end

  test "rejects with reasons and records the rejection" do
    post "/gate/push", params: report(ref: "refs/heads/main", message: "x"), as: :json,
                       headers: { "X-Airlock-Hook-Secret" => SECRET }
    assert_response :forbidden
    assert_match(/protected/, response.body)
    assert_equal "reject", GateDecision.last.verdict
  end
end
