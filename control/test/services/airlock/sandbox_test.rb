require "test_helper"
require "tmpdir"

class Airlock::SandboxTest < ActiveSupport::TestCase
  # Records calls and answers like the Sandboxes API would.
  class FakeClient
    attr_reader :uploads, :spawns

    def initialize(exit_code: 0, status: "SUCCESS", stdout: "ok\n")
      @exit_code, @status, @stdout = exit_code, status, stdout
      @uploads, @spawns = [], []
    end

    def upload(bytes)
      @uploads << bytes
      "file-#{@uploads.size}"
    end

    def run(**args)
      @spawns << args
      Airlock::Sandboxes::Client::Operation.new(uuid: "op-1", status: @status, exit_code: @exit_code, timed_out: false,
                                                stdout: @stdout, stderr: "", result_image: "img-after", duration: 1.2)
    end
  end

  setup do
    @dir = Dir.mktmpdir
    system("cd #{@dir} && git init -q && git config user.email t@t && git config user.name t && " \
           "echo tracked > a.txt && git add a.txt && git commit -qm x && echo untracked > secret.txt")
  end

  teardown { FileUtils.rm_rf(@dir) }

  test "ships only tracked files and runs the command on the unpacked tree" do
    client = FakeClient.new
    runner = Airlock::Runners::Sandbox.new(client: client, image: "tag:rust:1", command: "cargo test", timeout: 60)
    result = runner.call(@dir)
    assert result.ok
    assert_equal "img-after", result.checkpoint
    files = IO.popen(%w[tar -tz], "r+") { |io| io.write(client.uploads.first); io.close_write; io.read }.split
    assert_equal ["a.txt"], files
    spawn = client.spawns.first
    assert_equal "tag:rust:1", spawn[:image]
    assert_equal({ "/airlock/repo.tar.gz" => "file-1" }, spawn[:files])
    assert_match(/tar -xzf .* && cd \/work && \(cargo test\)\z/, spawn[:command])
    assert_equal "op-1", result.run_id
  end

  def payload(files, links: {})
    Dir.mktmpdir do |src|
      files.each { |name, body| FileUtils.mkdir_p(File.dirname(File.join(src, name))); File.write(File.join(src, name), body) }
      links.each { |name, target| FileUtils.mkdir_p(File.dirname(File.join(src, name))); File.symlink(target, File.join(src, name)) }
      tar = IO.popen(["tar", "-czf", "-", "-C", src, *(files.keys + links.keys)], &:read)
      "test ok\n#{Airlock::Collector::BEGIN_MARK}\n#{[tar].pack('m0')}\n#{Airlock::Collector::END_MARK}\n"
    end
  end

  test "collected files come back into the tree, and the payload leaves the output" do
    client = FakeClient.new(stdout: payload({ "tests/golden/a.png" => "PNG" }))
    runner = Airlock::Runners::Sandbox.new(client: client, image: "i", command: "make goldens", timeout: 5)
    result = runner.call(@dir, collect: ["tests/golden/"])
    assert result.ok
    assert_equal "PNG", File.read(File.join(@dir, "tests/golden/a.png"))
    assert_includes client.spawns.first[:command], "tar -czf - 'tests/golden/'"
    refute_includes result.output, Airlock::Collector::BEGIN_MARK
  end

  test "collection refuses files outside the allowed paths and links" do
    runner = ->(stdout) { Airlock::Runners::Sandbox.new(client: FakeClient.new(stdout: stdout), image: "i", command: "x", timeout: 5) }
    assert_raises(Airlock::Collector::Invalid) { runner.(payload({ "src/main.rs" => "evil" })).call(@dir, collect: ["tests/golden/"]) }
    assert_raises(Airlock::Collector::Invalid) do
      runner.(payload({}, links: { "tests/golden/x.png" => "/etc/passwd" })).call(@dir, collect: ["tests/golden/"])
    end
    refute File.exist?(File.join(@dir, "src/main.rs"))
  end

  test "a non-zero exit or a failed operation is a failure" do
    refute Airlock::Runners::Sandbox.new(client: FakeClient.new(exit_code: 101), image: "i", command: "x", timeout: 5).call(@dir).ok
    refute Airlock::Runners::Sandbox.new(client: FakeClient.new(status: "FAILED"), image: "i", command: "x", timeout: 5).call(@dir).ok
  end

  test "client decodes base64 output and polls until a terminal status" do
    polls = [{ "uuid" => "op", "status" => "EXECUTING" },
             { "uuid" => "op", "status" => "SUCCESS", "result_image_uuid" => "img",
               "result" => { "state" => { "exit_code" => 0 }, "stdout" => { "value" => Base64.strict_encode64("hi"), "encoding" => "base64" } } }]
    client = Airlock::Sandboxes::Client.new(token: "t", project: "p", sleeper: ->(_) {})
    client.define_singleton_method(:operation) { |_| parse_operation(polls.shift) }
    op = client.wait("op")
    assert_equal ["SUCCESS", 0, "hi", "img"], [op.status, op.exit_code, op.stdout, op.result_image]
  end
end
