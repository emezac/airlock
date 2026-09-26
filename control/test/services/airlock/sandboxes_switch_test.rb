require "test_helper"

class Airlock::SandboxesSwitchTest < ActiveSupport::TestCase
  KEYS = %w[NEBIUS_TOKEN NEBIUS_PROJECT_ID NEBIUS_SANDBOX].freeze

  setup { @saved = ENV.to_h.slice(*KEYS) }
  teardown { KEYS.each { |k| @saved.key?(k) ? ENV[k] = @saved[k] : ENV.delete(k) } }

  def env(token: "t", project: "p", switch: nil)
    token ? ENV["NEBIUS_TOKEN"] = token : ENV.delete("NEBIUS_TOKEN")
    project ? ENV["NEBIUS_PROJECT_ID"] = project : ENV.delete("NEBIUS_PROJECT_ID")
    switch.nil? ? ENV.delete("NEBIUS_SANDBOX") : ENV["NEBIUS_SANDBOX"] = switch
  end

  test "on only with the policy, the switch and credentials" do
    policy = Airlock::Policy.new
    env
    assert Airlock::Sandboxes.available?(policy), "unset switch means on"
    env(switch: "true")
    assert Airlock::Sandboxes.available?(policy)
    %w[false FALSE 0 off no].each do |value|
      env(switch: value)
      status = Airlock::Sandboxes.status(policy)
      refute status.on, value
      assert_match "NEBIUS_SANDBOX", status.reason
    end
    env(project: nil)
    assert_match "no credentials", Airlock::Sandboxes.status(policy).reason
    env
    assert_match "policy", Airlock::Sandboxes.status(Airlock::Policy.new("sandbox" => { "enabled" => false })).reason
  end

  test "switched off, nothing tries a sandbox even with credentials: local CI, no evidence re-run" do
    env(switch: "false")
    policy = Airlock::Policy.new
    assert_nil Airlock::QueueRunner.sandbox_runner(policy)
    assert_kind_of Airlock::Runners::Local, Airlock::QueueRunner.runner(policy)
    assert_nil Airlock::QueueRunner.verifier("frameline", policy: policy)
    env(switch: "true")
    assert_kind_of Airlock::Runners::Sandbox, Airlock::QueueRunner.runner(policy)
  end
end
