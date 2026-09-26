require "json"

module Airlock
  # Development only: a model client that never holds the API token. Each
  # request is written to a file; something on the machine that has the token
  # sends it to Token Factory and drops the response next to it. Used while
  # the control service ran in a sandbox without the token.
  #
  #   AIRLOCK_RELAY_OUT  where requests are written (req-<name>.json)
  #   AIRLOCK_RELAY_IN   where responses appear (resp-<name>.json)
  class RelayModel
    def self.configured? = ENV["AIRLOCK_RELAY_OUT"].present? && ENV["AIRLOCK_RELAY_IN"].present?

    def initialize(tag, out: ENV.fetch("AIRLOCK_RELAY_OUT"), inbox: ENV.fetch("AIRLOCK_RELAY_IN"), poll: 1, timeout: 3600)
      @tag = tag
      @out = out
      @inbox = inbox
      @poll = poll
      @timeout = timeout
      @n = 0
      FileUtils.mkdir_p(@out)
    end

    def chat(model:, messages:, max_tokens: 400, temperature: 0.0, thinking: false, json: false)
      @n += 1
      name = "#{@tag}-#{@n}"
      body = { model: model, messages: messages, max_tokens: max_tokens, temperature: temperature,
               chat_template_kwargs: { enable_thinking: thinking } }
      body[:response_format] = { type: "json_object" } if json
      tmp = File.join(@out, "req-#{name}.json.tmp")
      File.write(tmp, JSON.generate(body))
      File.rename(tmp, File.join(@out, "req-#{name}.json"))
      data = wait_for(File.join(@inbox, "resp-#{name}.json"))
      usage = data["usage"] || {}
      TokenFactory::Reply.new(content: data.dig("choices", 0, "message", "content").to_s, model: model,
                              input_tokens: usage["prompt_tokens"].to_i, output_tokens: usage["completion_tokens"].to_i)
    end

    private

    def wait_for(path)
      deadline = Time.now + @timeout
      loop do
        data = File.exist?(path) && (JSON.parse(File.read(path)) rescue nil)
        return data if data
        raise TokenFactory::Error, "no relayed response for #{File.basename(path)}" if Time.now > deadline

        # Waiting must not hold Rails' load interlock, or every other agent
        # thread that needs to load a constant blocks behind this one.
        ActiveSupport::Dependencies.interlock.permit_concurrent_loads { sleep @poll }
      end
    end
  end
end
