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
end
