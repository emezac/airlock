require "test_helper"

class Airlock::BatchMathTest < ActiveSupport::TestCase
  # Monte Carlo reference: 20,000 simulated batches per point (see docs).
  test "exact recursion matches simulated bisection" do
    { [4, 0.10] => 0.613, [9, 0.05] => 0.403, [4, 0.15] => 0.767, [2, 0.25] => 0.935 }.each do |(k, f), sim|
      assert_in_delta sim, Airlock::BatchMath.expected_runs(k, f) / k, 0.01, "k=#{k} f=#{f}"
    end
  end

  test "a batch of one costs one run and a clean batch costs one run" do
    assert_equal 1.0, Airlock::BatchMath.expected_runs(1, 0.3)
    assert_in_delta 1.0, Airlock::BatchMath.expected_runs(8, 0.0), 1e-9
  end

  test "best batch size shrinks as failures grow" do
    assert_equal 8, Airlock::BatchMath.best_size(0.05, max: 9)
    assert_equal 4, Airlock::BatchMath.best_size(0.10, max: 9)
    assert_equal 2, Airlock::BatchMath.best_size(0.25, max: 9)
    assert_equal 1, Airlock::BatchMath.best_size(0.60, max: 9)
  end
end
