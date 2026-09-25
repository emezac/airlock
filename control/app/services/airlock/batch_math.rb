module Airlock
  # Expected CI runs to integrate a batch of n changes by test-and-bisect, when
  # each change fails independently with probability f (q = 1 - f):
  #   R(1) = 1
  #   R(n) = 1 + R(a) + R(b) - 2 q^n,   a = floor(n/2), b = n - a
  # The batch runs once; if it fails, each half runs knowing its parent failed,
  # and E[R(half) | parent failed] = (R(half) - q^n) / (1 - q^n). Multiplying
  # by P(parent failed) = 1 - q^n gives the recursion above.
  module BatchMath
    module_function

    def expected_runs(n, f)
      q = 1.0 - f
      memo = { 1 => 1.0 }
      calc = lambda do |m|
        memo[m] ||= begin
          a = m / 2
          1.0 + calc.call(a) + calc.call(m - a) - 2.0 * (q**m)
        end
      end
      calc.call(n)
    end

    # Batch size that minimizes CI runs per integrated change.
    def best_size(f, max:)
      (1..max).min_by { |k| [expected_runs(k, f) / k, k] }
    end
  end
end
