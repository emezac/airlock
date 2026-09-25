require "tomlrb"

module Airlock
  # Rules read from airlock.toml. Defaults keep the gate closed rather than open.
  class Policy
    DEFAULT_SECRET_PATTERNS = [
      /AKIA[0-9A-Z]{16}/,                                   # AWS access key id
      /-----BEGIN [A-Z ]*PRIVATE KEY-----/,
      /\b(?:sk|pk)-[A-Za-z0-9_-]{20,}/,                     # common provider key shape
      /\bgh[pousr]_[A-Za-z0-9]{36,}/,                       # GitHub tokens
      /\beyJ[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{20,}\.[A-Za-z0-9_-]{10,}/, # JWT
      /(?i:api[_-]?key|secret|token|password)\s*[:=]\s*["'][^"'\s]{16,}["']/
    ].freeze

    DEFAULT_SKIP_PATTERNS = [
      /#\[ignore\]/, /\bpytest\.mark\.skip/, /\bxit\s*\(/, /\bxdescribe\s*\(/,
      /\bskip\s*\(/, /\.skip\s*\(/, /@unittest\.skip/
    ].freeze

    DEFAULT_TEST_DIRS = %w[tests/ test/ spec/].freeze

    attr_reader :evidence_trailer, :protected_refs, :merge_identity, :agent_prefix,
                :forbidden_paths, :human_review_paths, :test_dirs,
                :secret_patterns, :skip_patterns, :reject_on_secret, :reject_on_skipped_tests,
                :core_paths, :area_labels, :human_when, :target_utilization, :failure_rate_alpha, :reviews_per_hour,
                :min_threshold, :ambiguity_clarity, :max_batch, :ci_command, :ci_timeout,
                :verify_evidence, :allowed_evidence_commands, :sandbox_enabled, :sandbox_image

    def self.load(path = ENV.fetch("AIRLOCK_POLICY", Rails.root.join("config/airlock/airlock.toml").to_s))
      new(File.exist?(path) ? Tomlrb.load_file(path) : {})
    end

    def initialize(data = {})
      gate = data.fetch("gate", {})
      paths = data.fetch("paths", {})
      @evidence_trailer = gate.fetch("require_evidence_trailer", "Airlock-Evidence")
      @protected_refs = gate.fetch("protected_refs", ["refs/heads/main"])
      @merge_identity = gate.fetch("merge_identity", "airlock")
      @agent_prefix = gate.fetch("agent_prefix", "agent-")
      @reject_on_secret = gate.fetch("reject_on_secret", true)
      @reject_on_skipped_tests = gate.fetch("reject_on_deleted_or_skipped_tests", true)
      @forbidden_paths = Array(paths.fetch("forbidden", []))
      @human_review_paths = Array(paths.fetch("human_review", []))
      @test_dirs = Array(paths.fetch("tests", DEFAULT_TEST_DIRS))
      @secret_patterns = DEFAULT_SECRET_PATTERNS
      risk = data.fetch("risk", {})
      @core_paths = Array(paths.fetch("core", ["src/core/"]))
      # Longest prefix wins, so "src/core/" beats "src/".
      @area_labels = data.dig("labels", "area") || {}
      @human_when = Array(data.dig("routing", "human_when") ||
                          %w[safety_flag:touches_money safety_flag:touches_auth safety_flag:touches_data
                             safety_flag:weakens_safeguard test_signal:tests_weakened goal_clarity:vague
                             dependency:added_or_changed])
      @target_utilization = risk.fetch("target_utilization", 0.7).to_f
      @failure_rate_alpha = risk.fetch("failure_rate_alpha", 0.1).to_f
      @reviews_per_hour = risk.fetch("human_reviews_per_hour", 6).to_f
      @min_threshold = risk.fetch("min_threshold", 0.35).to_f
      @ambiguity_clarity = risk.fetch("ambiguity_clarity", 0.4).to_f
      queue = data.fetch("merge_queue", {})
      @max_batch = queue.fetch("max_batch", 9).to_i
      ci = data.fetch("ci", {})
      @ci_command = ci.fetch("command", "cargo test -q")
      @ci_timeout = ci.fetch("timeout_seconds", 900).to_i
      evidence = data.fetch("evidence", {})
      @verify_evidence = evidence.fetch("verify", true)
      @allowed_evidence_commands = Array(evidence.fetch("allowed_commands", []))
      sandbox = data.fetch("sandbox", {})
      @sandbox_enabled = sandbox.fetch("enabled", true)
      @sandbox_image = sandbox.fetch("image", "ubuntu:latest")
      @skip_patterns = DEFAULT_SKIP_PATTERNS
    end

    def agent?(pusher) = pusher.to_s.start_with?(agent_prefix)

    def agent_branch_prefix(pusher) = "refs/heads/agents/#{pusher}/"

    def forbidden?(path) = under?(path, forbidden_paths)

    def area_for(path)
      prefix = area_labels.keys.select { |p| path.start_with?(p) }.max_by(&:size)
      prefix ? area_labels[prefix] : "other"
    end

    def human_review?(path) = under?(path, human_review_paths)

    def test_path?(path) = under?(path, test_dirs)

    private

    # A prefix ending in "/" is a directory; anything else is an exact file name.
    def under?(path, prefixes)
      prefixes.any? { |prefix| prefix.end_with?("/") ? path.start_with?(prefix) : path == prefix }
    end
  end
end
