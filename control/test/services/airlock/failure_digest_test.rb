require "test_helper"

class Airlock::FailureDigestTest < ActiveSupport::TestCase
  test "compiler errors survive a wall of warnings" do
    log = (["warning: unused variable: `seed`", "  --> src/x.rs:1:5", ""] * 60 +
           ["error[E0599]: no method named `glow` found for struct `Canvas`", "  --> src/board.rs:33:16", "   |", "",
            "error: could not compile `taller_film` (lib) due to 1 previous error"]).join("\n")
    digest = Airlock::FailureDigest.call(log)
    assert_match "error[E0599]: no method named `glow`", digest
    assert_match "src/board.rs:33:16", digest
    refute_match "unused variable", digest
  end

  test "test failures keep the panic message and drop the backtrace" do
    log = <<~LOG
      running 2 tests
      test a ... ok
      test b ... FAILED

      failures:

      ---- b stdout ----
      thread 'b' panicked at tests/spec.rs:9:5:
      assertion failed: e.contains("scene 2")
      stack backtrace:
         0: __rustc::rust_begin_unwind
         1: core::panicking::panic_fmt

      failures:
          b

      test result: FAILED. 1 passed; 1 failed
    LOG
    digest = Airlock::FailureDigest.call(log)
    assert_match %(assertion failed: e.contains("scene 2")), digest
    refute_match "rust_begin_unwind", digest
  end

  test "anything else falls back to the tail" do
    assert_equal "boom", Airlock::FailureDigest.call("a\nboom\n").lines.last
  end
end
