module Airlock
  # What the pre-receive hook reports about one ref update.
  class Push < Data.define(:repo, :pusher, :ref, :old_sha, :new_sha, :commits, :files, :added_lines)
    ZERO = ("0" * 40).freeze

    def self.from_params(params)
      new(
        repo: params.fetch("repo").to_s,
        pusher: params.fetch("pusher").to_s,
        ref: params.fetch("ref").to_s,
        old_sha: params["old"].to_s,
        new_sha: params["new"].to_s,
        commits: Array(params["commits"]).map { |c| Commit.new(sha: c["sha"].to_s, author: c["author"].to_s, message: c["message"].to_s) },
        files: Array(params["files"]).map { |f| ChangedFile.new(path: f["path"].to_s, status: f["status"].to_s) },
        added_lines: Array(params["added_lines"]).map { |l| AddedLine.new(path: l["path"].to_s, text: l["text"].to_s) }
      )
    end

    def deletion? = new_sha.empty? || new_sha == ZERO
  end

  class Commit < Data.define(:sha, :author, :message)
    # Trailers are "Key: value" lines in the last paragraph of the message.
    def trailers
      last = message.strip.split(/\n\s*\n/).last.to_s
      last.lines.filter_map { |line| line.strip.match(/\A([A-Za-z][A-Za-z0-9-]*):\s*(.+)\z/)&.captures }
          .each_with_object({}) { |(key, value), acc| acc[key] = value }
    end
  end

  class ChangedFile < Data.define(:path, :status)
    def deleted? = status.start_with?("D")
  end

  AddedLine = Data.define(:path, :text)
end
