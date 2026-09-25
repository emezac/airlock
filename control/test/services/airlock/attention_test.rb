require "test_helper"

class Airlock::AttentionTest < ActiveSupport::TestCase
  test "light load keeps the minimum threshold" do
    assert_equal 0.35, Airlock::Attention.threshold([0.1, 0.5, 0.9], 2.0, 6.0, 0.7, 0.35)
  end

  test "heavy load raises the threshold so only the riskiest share reaches humans" do
    scores = (1..100).map { |i| i / 100.0 }
    # lambda 60/h, mu 6/h, target 0.7 -> humans can take 7% of changes.
    t = Airlock::Attention.threshold(scores, 60.0, 6.0, 0.7, 0.35)
    share = scores.count { |s| s > t } / scores.size.to_f
    assert_operator share, :<=, 0.07
    assert_operator t, :>=, 0.9
  end

  test "load never lowers the bar below the minimum" do
    assert_equal 0.35, Airlock::Attention.threshold([0.01, 0.02], 50.0, 6.0, 0.7, 0.35)
  end
end
