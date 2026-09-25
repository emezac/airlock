module Airlock
  # Asks Nemotron 3 Nano what a change is about. Advisory only: the result is a
  # feature of the risk score, never a decision by itself.
  class Classifier
    Result = Data.define(:area, :intent, :block_probability, :clarity, :why, :source)

    # A bare "is it risky?" made Nano flag every change at 0.95. A rubric with
    # anchored bands gives graded, usable scores (see docs/risk.md).
    PROMPT = <<~TEXT.freeze
      You triage changes made by coding agents before a merge queue. Most changes are routine.
      Score how likely a careful reviewer would BLOCK this change, from 0.0 to 1.0, using this rubric:
      0.0-0.2 routine: a local bug fix or small feature with a test, in one area.
      0.3-0.5 notable: broad edits, shared core code, or no test for new behavior.
      0.6-1.0 dangerous: money, auth, data migration, new dependencies, deleting safeguards.
      Also rate "clarity" from 0.0 to 1.0: how clearly the commit message states a specific, checkable goal.
      Vague goals such as "make it better" or "more cinematic" have clarity below 0.4.
      Answer with JSON only: {"area": string, "intent": "fix"|"feature"|"refactor"|"deps"|"config"|"test"|"other",
      "block_probability": number, "clarity": number, "why": string of at most 20 words}.
    TEXT

    UNKNOWN = Result.new(area: "unknown", intent: "other", block_probability: 0.0, clarity: 1.0, why: "no classifier", source: "none")

    def initialize(client: TokenFactory.configured? ? TokenFactory.new : nil)
      @client = client
    end

    def classify(files:, messages:)
      return UNKNOWN unless @client

      summary = "Files:\n#{files.first(60).join("\n")}\n\nCommit messages:\n#{messages.first(10).join("\n---\n")}"
      reply = @client.chat(model: TokenFactory::NANO, json: true, max_tokens: 200,
                           messages: [{ role: "system", content: PROMPT }, { role: "user", content: summary.first(8000) }])
      record_usage(reply)
      parse(reply.content)
    rescue TokenFactory::Error, JSON::ParserError, Net::OpenTimeout, Net::ReadTimeout, SocketError => e
      UNKNOWN.with(why: "classifier unavailable: #{e.class}")
    end

    private

    def parse(content)
      data = JSON.parse(content[/\{.*\}/m] || "{}")
      Result.new(area: data["area"].to_s.first(60), intent: data["intent"].to_s.first(20),
                 block_probability: data["block_probability"].to_f.clamp(0.0, 1.0),
                 clarity: (data["clarity"] || 1.0).to_f.clamp(0.0, 1.0),
                 why: data["why"].to_s.first(200), source: TokenFactory::NANO)
    end

    def record_usage(reply)
      Agentkit::Audit.record(event_type: "llm.classify", status: "ok", model: reply.model,
                             payload: { input_tokens: reply.input_tokens, output_tokens: reply.output_tokens })
    end
  end
end
