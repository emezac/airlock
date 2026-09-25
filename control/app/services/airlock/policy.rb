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
                :secret_patterns, :skip_patterns, :reject_on_secret, :reject_on_skipped_tests

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
      @skip_patterns = DEFAULT_SKIP_PATTERNS
    end

    def agent?(pusher) = pusher.to_s.start_with?(agent_prefix)

    def agent_branch_prefix(pusher) = "refs/heads/agents/#{pusher}/"

    def forbidden?(path) = under?(path, forbidden_paths)

    def human_review?(path) = under?(path, human_review_paths)

    def test_path?(path) = under?(path, test_dirs)

    private

    # A prefix ending in "/" is a directory; anything else is an exact file name.
    def under?(path, prefixes)
      prefixes.any? { |prefix| prefix.end_with?("/") ? path.start_with?(prefix) : path == prefix }
    end
  end
end
