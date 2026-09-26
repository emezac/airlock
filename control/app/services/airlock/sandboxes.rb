module Airlock
  # Whether commands run in Token Factory Sandboxes, and why not when they do
  # not. Three switches, all required: the policy's [sandbox] enabled, the
  # NEBIUS_SANDBOX environment switch (unset means on), and credentials.
  # NEBIUS_SANDBOX=false keeps the token for the models while sandbox access
  # is not granted yet.
  module Sandboxes
    OFF = %w[0 false off no].freeze
    Status = Data.define(:on, :reason)

    module_function

    def status(policy = Policy.load)
      return Status.new(on: false, reason: "disabled in the policy ([sandbox] enabled = false)") unless policy.sandbox_enabled
      return Status.new(on: false, reason: "switched off (NEBIUS_SANDBOX=#{ENV['NEBIUS_SANDBOX']})") if OFF.include?(ENV["NEBIUS_SANDBOX"].to_s.strip.downcase)
      return Status.new(on: false, reason: "no credentials (NEBIUS_TOKEN and NEBIUS_PROJECT_ID)") unless Client.configured?

      Status.new(on: true, reason: "Token Factory Sandboxes")
    end

    def available?(policy = Policy.load) = status(policy).on
  end
end
