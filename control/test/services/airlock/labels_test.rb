require "test_helper"

class Airlock::LabelsTest < ActiveSupport::TestCase
  L = Data.define(:category, :value, :source)

  setup { @policy = Airlock::Policy.new("labels" => { "area" => { "billing/" => "billing" } }) }

  test "rejects unknown categories and values" do
    assert_raises(Airlock::Labels::Invalid) { Airlock::Labels.validate!("mood", "happy", @policy) }
    assert_raises(Airlock::Labels::Invalid) { Airlock::Labels.validate!("goal_clarity", "sort_of", @policy) }
    assert_equal "vague", Airlock::Labels.validate!("goal_clarity", "vague", @policy)
  end

  test "area values come from the policy plus multiple and other" do
    assert_equal %w[billing multiple other], Airlock::Labels.allowed("area", @policy)
  end

  test "override beats rule beats model" do
    labels = [L.new("safety_flag", "none", "model"), L.new("safety_flag", "touches_money", "override"),
              L.new("area", "billing", "rule"), L.new("area", "other", "model")]
    effective = Airlock::Labels.effective(labels)
    assert_equal "touches_money", effective["safety_flag"].value
    assert_equal "billing", effective["area"].value
  end
end
