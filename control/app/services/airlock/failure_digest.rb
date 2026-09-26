module Airlock
  # Turns a long build and test log into what a model needs to fix it:
  # compiler errors first, then the test failures, and only then the tail.
  # Warnings are dropped; they bury the one line that matters.
  module FailureDigest
    MAX_LINES = 120

    module_function

    def call(output)
      lines = output.to_s.lines.map(&:rstrip)
      parts = errors(lines) + failures(lines)
      parts = lines.last(40) if parts.empty?
      parts.first(MAX_LINES).join("\n")
    end

    # rustc diagnostics: from "error..." to the blank line that ends it.
    def errors(lines)
      out = []
      lines.each_with_index do |line, i|
        next unless line.match?(/\Aerror(\[E\d+\])?: /) && !line.start_with?("error: test failed", "error: could not compile")

        block = lines[i..].take_while.with_index { |l, j| j.zero? || !l.strip.empty? }
        out.concat(block.first(25) + [""])
      end
      out
    end

    # libtest's "failures:" section, which carries every panic message.
    def failures(lines)
      start = lines.index("failures:")
      return [] unless start

      finish = lines.index { |l| l.start_with?("test result:") && lines.index(l) > start } || lines.size
      lines[start...finish].reject { |l| l.match?(/\A\s+\d+: /) || l.include?("stack backtrace") || l.start_with?("note: Some details") }
    end
  end
end
