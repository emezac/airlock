require "json"
require "net/http"

module Airlock
  # Minimal client for Nebius Token Factory (OpenAI-compatible chat completions).
  class TokenFactory
    Error = Class.new(StandardError)
    Reply = Data.define(:content, :model, :input_tokens, :output_tokens)

    NANO = "nvidia/NVIDIA-Nemotron-3-Nano-30B-A3B".freeze
    SUPER = "nvidia/nemotron-3-super-120b-a12b".freeze
    ULTRA = "nvidia/Nemotron-3-Ultra-550b-a55b".freeze

    def self.configured? = ENV["NEBIUS_TOKEN"].present?

    def initialize(token: ENV.fetch("NEBIUS_TOKEN"), base: ENV.fetch("NEBIUS_API_BASE", "https://api.tokenfactory.nebius.com/v1/"))
      @token = token
      @uri = URI.join(base, "chat/completions")
    end

    # thinking: false keeps fast calls fast; the reasoning trace would otherwise
    # consume the whole token budget before any answer appears.
    def chat(model:, messages:, max_tokens: 400, temperature: 0.0, thinking: false, json: false)
      body = { model: model, messages: messages, max_tokens: max_tokens, temperature: temperature,
               chat_template_kwargs: { enable_thinking: thinking } }
      body[:response_format] = { type: "json_object" } if json
      request = Net::HTTP::Post.new(@uri, "Content-Type" => "application/json", "Authorization" => "Bearer #{@token}")
      request.body = JSON.generate(body)
      response = Net::HTTP.start(@uri.host, @uri.port, use_ssl: true, open_timeout: 10, read_timeout: 120) do |http|
        http.request(request)
      end
      raise Error, "Token Factory #{response.code}" unless response.is_a?(Net::HTTPSuccess)

      data = JSON.parse(response.body)
      usage = data["usage"] || {}
      Reply.new(content: data.dig("choices", 0, "message", "content").to_s, model: model,
                input_tokens: usage["prompt_tokens"].to_i, output_tokens: usage["completion_tokens"].to_i)
    end
  end
end
