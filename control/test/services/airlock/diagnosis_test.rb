require "test_helper"

class Airlock::DiagnosisTest < ActiveSupport::TestCase
  class FakeModel
    attr_reader :messages

    def initialize(reply) = @reply = reply

    def chat(model:, messages:, **)
      @messages = messages
      raise Airlock::TokenFactory::Error, "Token Factory 503" if @reply == :error

      Airlock::TokenFactory::Reply.new(content: JSON.generate(@reply), model: model, input_tokens: 1, output_tokens: 1)
    end
  end

  COMPILE = "   Compiling taller_film\nerror[E0425]: cannot find function `render_scene_tile` in this scope\n  --> src/bin/frameline.rs:66:28\n\n" \
            "error: could not compile `taller_film` (bin \"frameline\") due to 1 previous error\n"
  VISUAL = "---- boards_match_goldens stdout ----\nthread 'boards_match_goldens' panicked at tests/visual.rs:64:5:\n" \
           "examples/specs/lumen_ad.yaml: 3110 pixels changed (mean difference 0.00551)\n"

  def change_with(log, state: "failed", files: %w[src/board.rs src/spec.rs])
    batch = MergeBatch.create!(repo: "frameline", state: "red", size: 2, log: log)
    decision = GateDecision.create!(repo: "frameline", ref: "refs/heads/agents/agent-a/fl-3", pusher: "agent-a", verdict: "accept")
    Change.create!(repo: "frameline", ref: decision.ref, pusher: "agent-a", head_sha: "x", gate_decision: decision, state: state,
                   merge_batch: batch, report: { "files" => files.map { |f| { "path" => f, "status" => "M" } },
                                                 "added_lines" => [{ "path" => "src/board.rs", "text" => "fn x() {}" }] })
  end

  def ci(ids, ok, output = nil) = { "event" => "ci", "changes" => ids, "ok" => ok, "output" => output }

  test "rules name the category from what the queue recorded" do
    compile = change_with([])
    compile.merge_batch.update!(log: [ci([compile.id, 99], false, COMPILE), ci([compile.id], false, COMPILE)])
    d = Airlock::Diagnosis.new.call(compile)
    assert_equal %w[compile_error rule agent_retry rule], d.values_at("category", "category_source", "next_step", "source")
    assert d["failed_alone"]
    assert_match "cannot find function `render_scene_tile`", d["summary"]

    visual = change_with([])
    visual.merge_batch.update!(log: [ci([visual.id], false, VISUAL)])
    assert_equal %w[visual_regression human], Airlock::Diagnosis.new.call(visual).values_at("category", "next_step")

    together = change_with([])
    together.merge_batch.update!(log: [ci([together.id, 7], false, VISUAL), ci([together.id], true), ci([together.id, 7], false, VISUAL)])
    d = Airlock::Diagnosis.new.call(together)
    assert_equal "interaction", d["category"]
    assert d["combined_only"]

    load_panic = change_with([])
    load_panic.merge_batch.update!(log: [ci([load_panic.id], false,
      "---- boards_match_goldens stdout ----\nthread 'boards_match_goldens' panicked at tests/visual.rs:40:9:\n" \
      "example spec is valid: scene 1 (Hook): caption longer than 8 characters\n")])
    assert_equal "test_failure", Airlock::Diagnosis.new.call(load_panic)["category"], "a panic in the visual test is not a visual change"

    # Real cargo output from the forged-evidence scenario: "error: test failed" is not a compile error.
    real = change_with([])
    real.merge_batch.update!(log: [ci([real.id], false, file_fixture("cargo_test_failure.txt").read)])
    assert_equal "test_failure", Airlock::Diagnosis.new.call(real)["category"]

    assert_equal "merge_conflict", Airlock::Diagnosis.new(model: FakeModel.new(:error)).call(change_with([], state: "conflict"))["category"]
  end

  test "the model adds culprits, a summary and a next step, all checked" do
    change = change_with([])
    change.merge_batch.update!(log: [ci([change.id], false, COMPILE)])
    model = FakeModel.new({ "culprit_files" => ["src/board.rs", "/etc/passwd"], "summary" => "The tile helper was never added.",
                            "next_step" => "rewrite_everything", "category" => "infrastructure" })
    d = Airlock::Diagnosis.new(model: model).call(change)

    assert_equal ["src/board.rs"], d["culprit_files"], "files outside the change are dropped"
    assert_equal "agent_retry", d["next_step"], "a next step outside the vocabulary is ignored"
    assert_equal "compile_error", d["category"], "the model does not override a rule category"
    assert_equal "The tile helper was never added.", d["summary"]
    assert_equal "model", d["source"]
    assert_includes model.messages.last[:content], "cannot find function"
  end

  test "the model may name the category only when the rules could not" do
    change = change_with([])
    change.merge_batch.update!(log: [ci([change.id], false, "something odd happened\n")])
    d = Airlock::Diagnosis.new(model: FakeModel.new({ "category" => "infrastructure", "next_step" => "agent_retry" })).call(change)
    assert_equal %w[infrastructure model], d.values_at("category", "category_source")
  end

  test "a model failure leaves the rule diagnosis in place" do
    change = change_with([])
    change.merge_batch.update!(log: [ci([change.id], false, COMPILE)])
    d = Airlock::Diagnosis.new(model: FakeModel.new(:error)).call(change)
    assert_equal %w[compile_error rule], d.values_at("category", "source")
    assert_match "503", d["model_error"]
  end

  test "the job stores the diagnosis on failed changes only" do
    change = change_with([])
    change.merge_batch.update!(log: [ci([change.id], false, COMPILE)])
    DiagnoseFailureJob.perform_now(change.id)
    assert_equal "compile_error", change.reload.diagnosis["category"]

    change.update!(state: "merged", diagnosis: {})
    DiagnoseFailureJob.perform_now(change.id)
    assert_equal({}, change.reload.diagnosis)
  end
end
