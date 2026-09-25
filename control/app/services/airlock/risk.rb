module Airlock
  # Risk score in [0, 1] as a noisy-OR of independent signals:
  #   risk = 1 - prod_i (1 - w_i * x_i),  x_i in [0, 1]
  # Each signal can only raise the risk, and no single weak signal saturates it.
  # Label-driven signals come from the effective labels (override > rule > model),
  # so a human correction changes the score deterministically.
  class Risk
    Score = Data.define(:value, :features)

    MANIFESTS = %w[Cargo.toml Cargo.lock Gemfile Gemfile.lock package.json requirements.txt].freeze

    # [weight, {label value => x}]
    LABEL_SIGNALS = {
      "dependency" => [0.5, { "added_or_changed" => 1.0 }],
      "blast_radius" => [0.3, { "core" => 1.0, "multi_area" => 0.5 }],
      "test_signal" => [0.6, { "tests_weakened" => 1.0, "no_tests" => 0.25 }],
      "goal_clarity" => [0.3, { "vague" => 1.0, "partial" => 0.5 }],
      "safety_flag" => [0.5, { "touches_money" => 1.0, "touches_auth" => 1.0, "touches_data" => 1.0,
                               "weakens_safeguard" => 1.0 }]
    }.freeze

    NUMERIC_WEIGHTS = { breadth: 0.3, volume: 0.3, agent_failure_rate: 1.0 }.freeze

    def score(push, agent_failure_rate:, labels:)
      x = {
        breadth: [push.files.size / 20.0, 1.0].min,
        volume: [push.added_lines.size / 400.0, 1.0].min,
        agent_failure_rate: agent_failure_rate.to_f.clamp(0.0, 1.0)
      }
      weights = NUMERIC_WEIGHTS.dup
      LABEL_SIGNALS.each do |category, (weight, map)|
        key = :"label_#{category}"
        x[key] = map.fetch(labels[category], 0.0)
        weights[key] = weight
      end
      value = 1.0 - x.reduce(1.0) { |acc, (k, v)| acc * (1.0 - weights.fetch(k) * v) }
      Score.new(value: value.round(4), features: x)
    end
  end
end
