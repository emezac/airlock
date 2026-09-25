module Airlock
  # Produces labels for a change: rules first, then Nemotron 3 Nano for the
  # semantic categories. The model votes several times and the majority wins;
  # the share of agreeing votes becomes the label's confidence.
  class Labeler
    Label = Data.define(:category, :value, :source, :confidence, :reason)

    WEAKENING = /\b(relax|loosen|lower(ed)?|raise[ds]? the (threshold|tolerance)|tolerance|skip|disable|flaky|ignore|xfail|increase.*threshold)\b/i

    def initialize(policy, client: TokenFactory.configured? ? TokenFactory.new : nil, votes: 3)
      @policy = policy
      @client = client
      @votes = votes
    end

    def call(push)
      rule_labels(push) + model_labels(push)
    end

    def rule_labels(push)
      paths = push.files.map(&:path)
      areas = paths.map { |p| @policy.area_for(p) }.uniq
      tests = push.files.select { |f| @policy.test_path?(f.path) }
      messages = push.commits.map(&:message).join("\n")
      test_signal =
        if tests.any? && messages.match?(WEAKENING) then "tests_weakened"
        elsif tests.any? { |f| f.status.start_with?("A") } then "tests_added"
        elsif tests.any? then "tests_modified"
        else "no_tests"
        end
      [
        rule("area", areas.size == 1 ? areas.first : "multiple", "paths: #{areas.join(', ')}"),
        rule("blast_radius", blast_radius(paths, areas), "#{paths.size} files in #{areas.size} areas"),
        rule("test_signal", test_signal, "#{tests.size} test files"),
        rule("dependency", paths.any? { |p| Risk::MANIFESTS.include?(File.basename(p)) } ? "added_or_changed" : "none",
             "manifest check")
      ]
    end

    def model_labels(push)
      return [] unless @client

      ballots = Array.new(@votes) { ask(push) }.compact
      return [] if ballots.empty?

      Labels::MODEL_CATEGORIES.filter_map do |category|
        values = ballots.filter_map { |b| b[category] }.select { |v| Labels.valid?(category, v, @policy) }
        next if values.empty?

        value, count = values.tally.max_by { |v, n| [n, v] }
        Label.new(category: category, value: value, source: "model",
                  confidence: (count.to_f / @votes).round(2), reason: "#{count}/#{@votes} votes, #{TokenFactory::NANO}")
      end
    end

    private

    def rule(category, value, reason)
      Label.new(category: category, value: Labels.validate!(category, value, @policy), source: "rule",
                confidence: 1.0, reason: reason)
    end

    def blast_radius(paths, areas)
      return "core" if paths.any? { |p| @policy.core_paths.any? { |c| p.start_with?(c) } }

      areas.size > 1 ? "multi_area" : "single_area"
    end

    def prompt
      lines = Labels::MODEL_CATEGORIES.map { |c| "- #{c}: one of #{Labels::SCHEMA[c][:values].join(', ')}" }.join("\n")
      <<~TEXT
        You label changes made by coding agents. Pick exactly one value per category from the allowed lists.
        #{lines}
        Definitions: "weakens_safeguard" means it disables, loosens, relaxes or skips any validation, test threshold, limit or check.
        "vague" means nobody could verify the goal as done.
        Answer with JSON only: {"labels": {"<category>": "<value>", ...}}.
      TEXT
    end

    def ask(push)
      summary = "Files:\n#{push.files.map(&:path).first(60).join("\n")}\n\nCommit messages:\n" \
                "#{push.commits.map(&:message).first(10).join("\n---\n")}"
      reply = @client.chat(model: TokenFactory::NANO, json: true, max_tokens: 120,
                           messages: [{ role: "system", content: prompt }, { role: "user", content: summary.first(8000) }])
      Agentkit::Audit.record(event_type: "llm.label", status: "ok", model: reply.model,
                             payload: { input_tokens: reply.input_tokens, output_tokens: reply.output_tokens })
      JSON.parse(reply.content[/\{.*\}/m] || "{}").fetch("labels", {})
    rescue TokenFactory::Error, JSON::ParserError, Net::OpenTimeout, Net::ReadTimeout, SocketError
      nil
    end
  end
end
