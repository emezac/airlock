require "test_helper"

class Airlock::LabelerTest < ActiveSupport::TestCase
  # Answers a scripted sequence of label ballots.
  class ScriptedClient
    Reply = Airlock::TokenFactory::Reply

    def initialize(*ballots) = @ballots = ballots

    def chat(**)
      Reply.new(content: JSON.generate("labels" => @ballots.shift || {}), model: "nano", input_tokens: 1, output_tokens: 1)
    end
  end

  setup do
    @policy = Airlock::Policy.new("labels" => { "area" => { "src/" => "source", "src/core/" => "core_renderer", "billing/" => "billing" } },
                                  "paths" => { "core" => ["src/core/"] })
  end

  def push(files, message: "Fix caption overflow")
    Airlock::Push.from_params("repo" => "r", "pusher" => "agent-1", "ref" => "x", "old" => "a" * 40, "new" => "b" * 40,
                              "commits" => [{ "sha" => "c" * 40, "message" => message }],
                              "files" => files.map { |p, st| { "path" => p, "status" => st || "M" } }, "added_lines" => [])
  end

  def rules(*args, **kw) = Airlock::Labeler.new(@policy, client: nil).rule_labels(push(*args, **kw)).to_h { |l| [l.category, l.value] }

  test "rules derive area, blast radius, tests and dependencies from paths" do
    labels = rules([["src/core/canvas.rs"], ["tests/canvas.rs", "A"]])
    assert_equal "multiple", labels["area"]
    assert_equal "core", labels["blast_radius"]
    assert_equal "tests_added", labels["test_signal"]
    assert_equal "none", labels["dependency"]
    assert_equal "added_or_changed", rules([["Cargo.toml"]])["dependency"]
  end

  test "longest path prefix wins for area" do
    assert_equal "core_renderer", rules([["src/core/canvas.rs"]])["area"]
    assert_equal "source", rules([["src/films/boat.rs"]])["area"]
    assert_equal "other", rules([["README.md"]])["area"]
  end

  test "touching tests with a weakening message is tests_weakened" do
    assert_equal "tests_weakened",
                 rules([["tests/visual.rs"]], message: "Relax visual threshold to stop flaky failures")["test_signal"]
  end

  test "the model's majority wins and its share becomes the confidence" do
    client = ScriptedClient.new({ "change_type" => "fix", "goal_clarity" => "clear" },
                                { "change_type" => "fix", "goal_clarity" => "vague" },
                                { "change_type" => "feature", "goal_clarity" => "clear" })
    labels = Airlock::Labeler.new(@policy, client: client, votes: 3).model_labels(push([["src/a.rs"]])).index_by(&:category)
    assert_equal ["fix", 0.67], [labels["change_type"].value, labels["change_type"].confidence]
    assert_equal "clear", labels["goal_clarity"].value
  end

  test "values outside the vocabulary are discarded" do
    client = ScriptedClient.new({ "change_type" => "yolo", "safety_flag" => "touches_money" },
                                { "change_type" => "rewrite" }, {})
    labels = Airlock::Labeler.new(@policy, client: client, votes: 3).model_labels(push([["src/a.rs"]])).index_by(&:category)
    refute labels.key?("change_type")
    assert_equal ["touches_money", 0.33], [labels["safety_flag"].value, labels["safety_flag"].confidence]
  end
end
