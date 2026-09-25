module Airlock
  # Risk score in [0, 1] as a noisy-OR of independent signals:
  #   risk = 1 - prod_i (1 - w_i * x_i),  x_i in [0, 1]
  # Each signal can only raise the risk, and no single weak signal saturates it.
  class Risk
    Score = Data.define(:value, :features)

    WEIGHTS = {
      dependency_change: 0.5,
      core_change: 0.3,
      breadth: 0.3,        # files changed, saturating at 20
      volume: 0.3,         # lines added, saturating at 400
      no_tests: 0.15,
      agent_failure_rate: 1.0,
      model_block: 0.4     # Nemotron Nano: probability a reviewer would block
    }.freeze

    MANIFESTS = %w[Cargo.toml Cargo.lock Gemfile Gemfile.lock package.json requirements.txt].freeze

    def initialize(policy, core_paths: ["src/core/"])
      @policy = policy
      @core_paths = core_paths
    end

    def score(push, agent_failure_rate:, classification:)
      paths = push.files.map(&:path)
      x = {
        dependency_change: paths.any? { |p| MANIFESTS.include?(File.basename(p)) } ? 1.0 : 0.0,
        core_change: paths.any? { |p| @core_paths.any? { |c| p.start_with?(c) } } ? 1.0 : 0.0,
        breadth: [paths.size / 20.0, 1.0].min,
        volume: [push.added_lines.size / 400.0, 1.0].min,
        no_tests: paths.none? { |p| @policy.test_path?(p) } ? 1.0 : 0.0,
        agent_failure_rate: agent_failure_rate.to_f.clamp(0.0, 1.0),
        model_block: classification.block_probability
      }
      value = 1.0 - x.reduce(1.0) { |acc, (k, v)| acc * (1.0 - WEIGHTS.fetch(k) * v) }
      Score.new(value: value.round(4), features: x)
    end
  end
end
