require "test_helper"
require "tmpdir"

class Airlock::EditsTest < ActiveSupport::TestCase
  setup do
    @dir = Dir.mktmpdir
    File.write(File.join(@dir, "a.rs"), %(fn a() {}\nfn b() { println!("{}", "q\\"x"); }\n))
  end

  teardown { FileUtils.rm_rf(@dir) }

  def block(path, search, replace) = "#{path}\n<<<<<<< SEARCH\n#{search}=======\n#{replace}>>>>>>> REPLACE\n"
  def reply(*blocks, goldens: "no") = "SUMMARY: s\nUPDATE_GOLDENS: #{goldens}\n\n#{blocks.join("\n")}"
  def apply(text) = Airlock::Edits.parse(text).apply!(@dir)

  test "blocks carry code verbatim, with no escaping" do
    code = %(fn b() { println!("{}", "q\\"x"); }\n)
    changed = apply(reply(block("a.rs", code, code.sub("q", "r")), block("examples/specs/x.yaml", "", "title: x\n")))
    assert_equal %w[a.rs examples/specs/x.yaml], changed
    assert_includes File.read(File.join(@dir, "a.rs")), %("r\\"x")
    assert_equal "title: x\n", File.read(File.join(@dir, "examples/specs/x.yaml"))
  end

  test "summary, golden flag and fenced replies are understood" do
    edits = Airlock::Edits.parse("```\n#{reply(block('a.rs', "fn a() {}\n", "fn a() { 1 }\n"), goldens: 'yes')}```\n")
    assert_equal "s", edits.summary
    assert edits.update_goldens?
    assert_equal ["a.rs"], edits.paths
  end

  test "a block without its own path edits the previous block's file" do
    text = "SUMMARY: s\na.rs\n<<<<<<< SEARCH\nfn a() {}\n=======\nfn a() { 1 }\n>>>>>>> REPLACE\n" \
           "<<<<<<< SEARCH\nfn a() { 1 }\n=======\nfn a() { 2 }\n>>>>>>> REPLACE\n"
    assert_equal %w[a.rs a.rs], Airlock::Edits.parse(text).edits.map(&:path)
    apply(text)
    assert_match "fn a() { 2 }", File.read(File.join(@dir, "a.rs"))
  end

  test "nothing is written when one block fails" do
    error = assert_raises(Airlock::Edits::Invalid) { apply(reply(block("new.txt", "", "x\n"), block("a.rs", "fn c()\n", ""))) }
    assert_match "not in the file", error.message
    refute File.exist?(File.join(@dir, "new.txt"))
  end

  test "a mistyped copy is reported line by line" do
    msg = Airlock::Edits.mismatch("fn a() {\n    let x = 1;\n    let y = 2;\n}\n", "fn a() {\n    let x = 1;\n    let y = 3;\n")
    assert_equal %(Line 3 of your SEARCH is "    let y = 3;" but the file has "    let y = 2;" there), msg
  end

  test "ambiguous, missing and malformed blocks are refused" do
    assert_match "2 times", assert_raises(Airlock::Edits::Invalid) { apply(reply(block("a.rs", "}\n", "} // end\n"))) }.message
    assert_match "already exists", assert_raises(Airlock::Edits::Invalid) { apply(reply(block("a.rs", "", "x\n"))) }.message
    assert_match "no SUMMARY", assert_raises(Airlock::Edits::Invalid) { Airlock::Edits.parse(block("a.rs", "a\n", "b\n")) }.message
    assert_match "no SEARCH/REPLACE", assert_raises(Airlock::Edits::Invalid) { Airlock::Edits.parse("SUMMARY: s\n") }.message
    assert_match "line 3 of your reply has no >>>>>>> REPLACE",
                 assert_raises(Airlock::Edits::Invalid) { Airlock::Edits.parse("SUMMARY: s\na.rs\n<<<<<<< SEARCH\nx\n=======\ny\n") }.message
    missing = "SUMMARY: s\na.rs\n<<<<<<< SEARCH\nx\n>>>>>>> REPLACE\n<<<<<<< SEARCH\nx\n=======\ny\n>>>>>>> REPLACE\n"
    assert_match "needs exactly one ======= line", assert_raises(Airlock::Edits::Invalid) { Airlock::Edits.parse(missing) }.message
  end

  test "paths stay inside the tree" do
    ["../x", "/etc/passwd", ".git/config", "a/../../x"].each do |path|
      assert_raises(Airlock::Edits::Invalid, path) { apply(reply(block(path, "", "x\n"))) }
    end
  end

  test "symlinks are not followed" do
    outside = Dir.mktmpdir
    File.symlink(outside, File.join(@dir, "link"))
    assert_match "symlink", assert_raises(Airlock::Edits::Invalid) { apply(reply(block("link/x", "", "x\n"))) }.message
    assert_empty Dir.children(outside)
  ensure
    FileUtils.rm_rf(outside)
  end
end
