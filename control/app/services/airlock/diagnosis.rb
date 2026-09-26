require "json"

module Airlock
  # Why did a change fail in the merge queue? Rules decide the category from
  # what the queue recorded (a git conflict, a change that passed alone but not
  # with the others, compiler errors, the visual test); Nemotron Ultra adds the
  # semantic part: which of the change's files is at fault, a short
  # explanation and what to do next. Every model value is checked against a
  # closed vocabulary, and without a model the rules alone still answer.
  class Diagnosis
    CATEGORIES = %w[merge_conflict interaction compile_error visual_regression test_failure infrastructure unknown].freeze
    NEXT_STEPS = %w[agent_retry human drop].freeze
    DEFAULT_NEXT = {
      "merge_conflict" => "agent_retry", "interaction" => "human", "compile_error" => "agent_retry",
      "visual_regression" => "human", "test_failure" => "agent_retry", "infrastructure" => "agent_retry",
      "unknown" => "human"
    }.freeze
    MAX_SUMMARY = 400
    MAX_DIFF_CHARS = 12_000

    def self.model
      return RelayModel.new("diag-#{SecureRandom.hex(3)}") if RelayModel.configured?
      return TokenFactory.new if TokenFactory.configured?

      nil
    end

    def initialize(model: nil, model_name: TokenFactory::ULTRA)
      @model = model
      @model_name = model_name
    end

    def call(change)
      facts = facts(change)
      category = rule_category(change, facts)
      result = { "category" => category, "category_source" => "rule", "combined_only" => facts[:combined_only],
                 "failed_alone" => facts[:failed_alone], "culprit_files" => [], "summary" => fallback_summary(category, facts),
                 "next_step" => DEFAULT_NEXT.fetch(category), "source" => "rule" }
      return result if @model.nil? || category == "merge_conflict"

      result.merge(model_part(change, category, facts))
    rescue TokenFactory::Error, JSON::ParserError => e
      result.merge("model_error" => e.message.truncate(200))
    end

    # What the queue saw: the smallest failing CI run that contained this change,
    # and whether the change ever passed in a set that was then combined.
    def facts(change)
      runs = Array(change.merge_batch&.log).select { |e| e["event"] == "ci" && Array(e["changes"]).include?(change.id) }
      failing = runs.reject { |e| e["ok"] }
      smallest = failing.min_by { |e| Array(e["changes"]).size }
      {
        output: smallest&.dig("output").to_s,
        failed_alone: failing.any? { |e| e["changes"] == [change.id] },
        combined_only: runs.any? { |e| e["ok"] } && failing.any? && failing.none? { |e| e["changes"] == [change.id] },
        runs: runs.size
      }
    end

    private

    def rule_category(change, facts)
      out = facts[:output]
      return "merge_conflict" if change.state == "conflict"
      return "interaction" if facts[:combined_only]
      return "infrastructure" if out.match?(/timed out|Terminated|No space left|Killed/i) && !out.include?("test result")
      return "compile_error" if out.match?(/^error\[E\d+\]/) || out.include?("could not compile")
      return "visual_regression" if out.match?(/pixels changed/)
      return "test_failure" if out.match?(/panicked|test result: FAILED|FAILED/)

      "unknown"
    end

    def fallback_summary(category, facts)
      return "The branch no longer merges cleanly with main." if category == "merge_conflict"
      return "Passes on its own but fails together with other changes in the batch." if category == "interaction"

      line = FailureDigest.call(facts[:output]).lines.map(&:strip).find { |l| l.match?(/error|panicked|assert|FAILED|pixels/) }
      (line || "The check failed without a recognizable error line.").truncate(MAX_SUMMARY)
    end

    def model_part(change, category, facts)
      files = Array(change.report["files"]).map { |f| f["path"] }.compact
      reply = @model.chat(model: @model_name, messages: prompt(change, category, facts, files), max_tokens: 1200,
                          temperature: 0.0, json: true)
      data = JSON.parse(reply.content.to_s[/\{.*\}/m] || "{}")
      part = { "source" => "model", "model" => reply.model }
      culprits = Array(data["culprit_files"]).map(&:to_s) & files
      part["culprit_files"] = culprits.first(5)
      summary = data["summary"].to_s.strip
      part["summary"] = summary.truncate(MAX_SUMMARY) if summary.present?
      part["next_step"] = data["next_step"] if NEXT_STEPS.include?(data["next_step"])
      if category == "unknown" && CATEGORIES.include?(data["category"]) && data["category"] != "merge_conflict"
        part["category"] = data["category"]
        part["category_source"] = "model"
      end
      part
    end

    def prompt(change, category, facts, files)
      added = Array(change.report["added_lines"]).map { |l| "#{l['path']}: #{l['text']}" }.join("\n").truncate(MAX_DIFF_CHARS)
      system = <<~TXT
        You diagnose why an agent's change failed in a merge queue. Reply with one JSON object:
        {"culprit_files": [paths from the changed files only], "summary": "at most two sentences a developer can act on",
         "next_step": one of #{NEXT_STEPS.to_json}#{category == 'unknown' ? %(, "category": one of #{CATEGORIES.to_json}) : ''}}
        agent_retry: the agent can fix it with the error in hand. human: needs judgement (visual change, design,
        interaction between changes). drop: the change should be abandoned.
      TXT
      user = <<~TXT
        Category (decided by rules): #{category}
        #{facts[:combined_only] ? 'The change passed on its own and failed only together with other changes.' : ''}
        Changed files: #{files.join(', ')}

        Failure output:
        ```
        #{FailureDigest.call(facts[:output])}
        ```

        Added lines:
        ```
        #{added}
        ```
      TXT
      [{ role: "system", content: system }, { role: "user", content: user }]
    end
  end
end
