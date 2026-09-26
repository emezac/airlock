require "pathname"

module Airlock
  # The only way a worker's model changes files. The reply is plain text with
  # SEARCH/REPLACE blocks, so code never needs JSON escaping:
  #
  #   SUMMARY: one or two sentences
  #   UPDATE_GOLDENS: no
  #
  #   src/spec.rs
  #   <<<<<<< SEARCH
  #   exact lines from the file
  #   =======
  #   the new lines
  #   >>>>>>> REPLACE
  #
  # An empty SEARCH creates a new file. Anything ambiguous is refused with a
  # message the model can act on, and nothing is half-applied.
  class Edits
    Invalid = Class.new(StandardError)
    Edit = Data.define(:path, :search, :replace)

    MAX_FILE_BYTES = 64 * 1024
    MAX_OPERATIONS = 20
    MAX_SEARCH_LINES = 120

    SEARCH = /^<{5,9} SEARCH\s*$/
    DIVIDER = /^={5,9}\s*$/
    REPLACE = /^>{5,9} REPLACE\s*$/

    attr_reader :summary, :edits

    def self.parse(text)
      lines = text.to_s.gsub("\r\n", "\n").lines
      summary = lines.find { |l| l.start_with?("SUMMARY:") }.to_s.delete_prefix("SUMMARY:").strip
      goldens = lines.find { |l| l.start_with?("UPDATE_GOLDENS:") }.to_s.delete_prefix("UPDATE_GOLDENS:").strip.downcase
      new(summary: summary, edits: blocks(lines), update_goldens: %w[yes true].include?(goldens))
    end

    def self.blocks(lines)
      edits = []
      i = 0
      while i < lines.size
        unless lines[i].match?(SEARCH)
          i += 1
          next
        end
        path = lines[0...i].reverse.map(&:strip).find { |l| !l.empty? && !l.start_with?("```") }.to_s
        # A block right after another one without a new path edits the same file.
        path = edits.last.path if path.match?(REPLACE) && edits.any?
        raise Invalid, "a SEARCH block has no file path on the line before it" if path.empty? || path.match?(REPLACE)

        finish = (i + 1...lines.size).find { |j| lines[j].match?(REPLACE) } or
          raise Invalid, "#{path}: the block at line #{i + 1} of your reply has no >>>>>>> REPLACE"
        dividers = (i + 1...finish).select { |j| lines[j].match?(DIVIDER) }
        unless dividers.size == 1
          raise Invalid, "#{path}: the block at line #{i + 1} of your reply needs exactly one ======= line " \
                         "between SEARCH and REPLACE (found #{dividers.size})"
        end
        divider = dividers.first
        edits << Edit.new(path: path.delete_prefix("`").delete_suffix("`"),
                          search: lines[i + 1...divider].join, replace: lines[divider + 1...finish].join)
        i = finish + 1
      end
      edits
    end

    # Points at the first line of a search that does not match the file, so a
    # model can fix a mistyped copy instead of guessing.
    def self.mismatch(text, search)
      file = text.lines.map(&:chomp)
      wanted = search.lines.map(&:chomp)
      first = wanted.find { |l| !l.strip.empty? }
      offset = wanted.index(first)
      starts = file.each_index.select { |i| file[i].strip == first.to_s.strip }
      return "Its first line #{first.to_s.strip.inspect} is not in the file" if starts.empty?

      k, start = starts.map do |i|
        from = i - offset
        [(0...wanted.size).find { |j| (from + j).negative? || file[from + j] != wanted[j] } || wanted.size, from]
      end.max
      return "Check whitespace and line endings" if k >= wanted.size

      "Line #{k + 1} of your SEARCH is #{wanted[k].inspect} but the file has #{file[start + k].to_s.inspect} there"
    end

    def initialize(summary:, edits:, update_goldens: false)
      @summary = summary.to_s.strip
      @edits = edits
      @update_goldens = update_goldens
      raise Invalid, "the reply has no SUMMARY: line" if @summary.empty?
      raise Invalid, "the reply has no SEARCH/REPLACE blocks" if @edits.empty?
      raise Invalid, "at most #{MAX_OPERATIONS} blocks" if @edits.size > MAX_OPERATIONS
    end

    def update_goldens? = @update_goldens

    def paths = edits.map(&:path).uniq

    # Applies every block or none. Returns the paths changed.
    def apply!(root)
      root = Pathname(root).realpath
      staged = {}
      edits.each do |edit|
        file = resolve(root, edit.path)
        exists = staged.key?(file) || file.file?
        if edit.search.strip.empty?
          raise Invalid, "#{edit.path}: already exists; an empty SEARCH only creates new files" if exists

          staged[file] = edit.replace
          next
        end
        raise Invalid, "#{edit.path}: file does not exist (use an empty SEARCH to create it)" unless exists
        if edit.search.lines.size > MAX_SEARCH_LINES
          raise Invalid, "#{edit.path}: SEARCH has #{edit.search.lines.size} lines (at most #{MAX_SEARCH_LINES}); " \
                         "use the lines around the change"
        end

        text = staged.fetch(file) { file.read }
        count = text.scan(edit.search).size
        raise Invalid, "#{edit.path}: SEARCH not found. #{Edits.mismatch(text, edit.search)}" if count.zero?
        raise Invalid, "#{edit.path}: SEARCH matches #{count} times; include more lines" if count > 1

        staged[file] = text.sub(edit.search) { edit.replace }
      end
      staged.each do |file, text|
        raise Invalid, "#{file.relative_path_from(root)}: larger than #{MAX_FILE_BYTES} bytes" if text.bytesize > MAX_FILE_BYTES
      end
      staged.each do |file, text|
        file.dirname.mkpath
        file.write(text)
      end
      staged.keys.map { |f| f.relative_path_from(root).to_s }
    end

    private

    # Relative, inside the tree, never in .git, never through a symlink.
    def resolve(root, path)
      clean = Pathname(path).cleanpath
      raise Invalid, "#{path}: paths must be relative to the repository root" if clean.absolute?
      raise Invalid, "#{path}: path leaves the repository" if clean.each_filename.first == ".."
      raise Invalid, "#{path}: .git is off limits" if clean.each_filename.include?(".git")

      file = root.join(clean)
      file.ascend do |part|
        break if part == root
        raise Invalid, "#{path}: path goes through a symlink" if part.symlink?
      end
      file
    end
  end
end
