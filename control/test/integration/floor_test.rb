require "test_helper"

class FloorTest < ActionDispatch::IntegrationTest
  setup do
    d = GateDecision.create!(repo: "frameline", ref: "refs/heads/agents/agent-kai/fl-10", pusher: "agent-kai", verdict: "accept")
    @change = Change.create!(repo: "frameline", ref: d.ref, pusher: "agent-kai", head_sha: "x", gate_decision: d, state: "needs_review")
    AgentRun.create!(repo: "frameline", agent: "agent-ada", task: "FL-3", status: "working", last_note: "attempt 2: running cargo test")
    @e1 = Airlock::Floor.emit("agent.testing", repo: "frameline", actor: "agent-ada", task: "FL-3", text: "cargo test --release",
                              at: 2.minutes.ago)
    @e2 = Airlock::Floor.emit("gate.accepted", repo: "frameline", actor: "agent-kai", task: "FL-10", text: "agents/agent-kai/fl-10",
                              at: 1.minute.ago)
  end

  test "the state places agents and changes at their stations" do
    get "/floor/state"
    s = response.parsed_body
    assert_equal %w[agent-ada agent-kai], s["agents"].map { |a| a["name"] }
    assert_equal "working", s["agents"].first["status"]
    assert_equal [["FL-10", "review"]], s["changes"].map { |c| c.values_at("task", "station") }
    assert_equal @e2.id, s["cursor"]
  end

  test "events come after a cursor, or inside a window for replay" do
    get "/floor/events", params: { after: @e1.id }
    body = response.parsed_body
    assert_equal ["gate.accepted"], body["events"].map { |e| e["kind"] }
    assert_equal @e2.id, body["cursor"]

    get "/floor/events", params: { from: 3.minutes.ago.iso8601, to: 90.seconds.ago.iso8601 }
    assert_equal ["agent.testing"], response.parsed_body["events"].map { |e| e["kind"] }

    get "/floor/events", params: { from: "yesterday" }
    assert_response :bad_request
  end

  test "sessions are stretches of events split by a quiet gap, newest first" do
    Airlock::Floor.emit("gate.rejected", repo: "frameline", actor: "agent-rex", task: "X", at: 3.hours.ago)
    get "/floor/sessions"
    s = response.parsed_body
    assert_equal [2, 1], s.map { |w| w["events"] }
    assert_equal %w[agent-ada agent-kai], s.first["agents"]
    assert_equal ["agent-rex"], s.last["agents"]
  end

  test "an unknown kind is refused and never breaks the caller" do
    assert_nil Airlock::Floor.emit("agent.dancing", repo: "frameline")
    assert_equal 2, FloorEvent.count
  end

  test "the page renders with the tabs" do
    get "/floor"
    assert_response :ok
    assert_includes response.body, "Floor"
    assert_includes response.body, "Control room"
  end
end
