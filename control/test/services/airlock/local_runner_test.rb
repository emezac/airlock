require "test_helper"
require "tmpdir"

class Airlock::LocalRunnerTest < ActiveSupport::TestCase
  test "a run never inherits a shared cargo target directory" do
    ENV["CARGO_TARGET_DIR"] = "/tmp/shared-target"
    Dir.mktmpdir do |dir|
      result = Airlock::Runners::Local.new(command: 'test -z "$CARGO_TARGET_DIR"', timeout: 5).call(dir)
      assert result.ok, result.output
    end
  ensure
    ENV.delete("CARGO_TARGET_DIR")
  end

  test "agent code under test cannot read the service's secrets" do
    ENV["NEBIUS_TOKEN"] = "secret-token"
    ENV["AIRLOCK_HOOK_SECRET_PROBE"] = "x"
    Dir.mktmpdir do |dir|
      probe = 'test -z "$NEBIUS_TOKEN" && test -z "$AIRLOCK_HOOK_SECRET_PROBE" && test -n "$PATH"'
      result = Airlock::Runners::Local.new(command: probe, timeout: 5).call(dir)
      assert result.ok, result.output
    end
  ensure
    ENV.delete("NEBIUS_TOKEN")
    ENV.delete("AIRLOCK_HOOK_SECRET_PROBE")
  end
end
