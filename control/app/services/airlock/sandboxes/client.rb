require "json"
require "net/http"
require "base64"

module Airlock
  module Sandboxes
    # REST client for Nebius Token Factory Sandboxes.
    # Every instance run produces a result image: that image is a checkpoint
    # another run can start from.
    class Client
      Error = Class.new(StandardError)
      Operation = Data.define(:uuid, :status, :exit_code, :timed_out, :stdout, :stderr, :result_image, :duration)

      TERMINAL = %w[SUCCESS FAILED CANCELLED].freeze

      def self.configured? = ENV["NEBIUS_TOKEN"].present? && ENV["NEBIUS_PROJECT_ID"].present?

      def initialize(token: ENV.fetch("NEBIUS_TOKEN"), project: ENV.fetch("NEBIUS_PROJECT_ID"),
                     base: ENV.fetch("NEBIUS_SANDBOXES_BASE", "https://api.tokenfactory.nebius.com/sandboxes/v1/"),
                     sleeper: ->(s) { ActiveSupport::Dependencies.interlock.permit_concurrent_loads { sleep(s) } })
        @token = token
        @project = project
        @base = base.end_with?("/") ? base : "#{base}/"
        @sleeper = sleeper
      end

      # Returns the file uuid the instance can mount.
      def upload(bytes)
        response = request(Net::HTTP::Post, "files", body: bytes, type: "application/octet-stream")
        JSON.parse(response.body).fetch("uuid")
      end

      def spawn(image:, command:, files: {}, env: {}, timeout: 900, network: false)
        body = { image: image, command: command, shell: true, env: env, timeout: timeout, disposable: false,
                 networking: { enabled: network },
                 files: files.transform_values { |uuid| { uuid: uuid, mode: "0644" } } }
        response = request(Net::HTTP::Post, "instances", body: JSON.generate(body), type: "application/json")
        JSON.parse(response.body).fetch("uuid")
      end

      def operation(uuid)
        parse_operation(JSON.parse(request(Net::HTTP::Get, "operations/#{uuid}").body))
      end

      def wait(uuid, timeout: 1200, interval: 2)
        deadline = Process.clock_gettime(Process::CLOCK_MONOTONIC) + timeout
        loop do
          op = operation(uuid)
          return op if TERMINAL.include?(op.status)
          raise Error, "operation #{uuid} still #{op.status} after #{timeout}s" if Process.clock_gettime(Process::CLOCK_MONOTONIC) > deadline

          @sleeper.call(interval)
        end
      end

      def run(**spawn_args)
        wait(spawn(**spawn_args), timeout: spawn_args.fetch(:timeout, 900) + 300)
      end

      private

      def request(klass, path, body: nil, type: nil)
        uri = URI.join(@base, path)
        req = klass.new(uri, "Authorization" => "Bearer #{@token}", "Project" => @project)
        if body
          req["Content-Type"] = type
          req.body = body
        end
        response = Net::HTTP.start(uri.host, uri.port, use_ssl: uri.scheme == "https", open_timeout: 10, read_timeout: 120) do |http|
          http.request(req)
        end
        raise Error, "Sandboxes #{klass::METHOD} #{path}: #{response.code} #{response.body.to_s[0, 200]}" unless response.is_a?(Net::HTTPSuccess)

        response
      end

      def parse_operation(data)
        result = data["result"] || {}
        state = result["state"] || {}
        Operation.new(uuid: data["uuid"], status: data["status"], exit_code: state["exit_code"],
                      timed_out: state["timed_out"] == true, stdout: decode(result["stdout"]), stderr: decode(result["stderr"]),
                      result_image: data["result_image_uuid"], duration: data["duration"])
      end

      def decode(stream)
        return "" unless stream

        stream["encoding"] == "base64" ? Base64.decode64(stream["value"].to_s) : stream["value"].to_s
      end
    end
  end
end
