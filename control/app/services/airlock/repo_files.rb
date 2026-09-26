require "open3"

module Airlock
  # Read-only views into a bare repository: the images the Frameline tab shows
  # are blobs already in git, so showing them runs none of the agents' code.
  class RepoFiles
    SHA = /\A[0-9a-f]{40}\z/
    GOLDEN = %r{\Atests/golden/[A-Za-z0-9_.-]+\.png\z}

    def self.for(repo) = new(File.join(ENV.fetch("AIRLOCK_GIT_ROOT"), "#{repo}.git"))

    def initialize(git_dir)
      @git_dir = git_dir
    end

    def exists? = File.directory?(@git_dir)

    def sha(rev)
      out, ok = git("rev-parse", "--verify", "--quiet", "#{rev}^{commit}")
      ok ? out.strip : nil
    end

    def main = sha("refs/heads/main")

    # The first commit of main: the tree before any agent touched it.
    def root
      out, ok = git("rev-list", "--max-parents=0", "refs/heads/main")
      ok ? out.split.last : nil
    end

    # Commits main has pointed at, newest first.
    def main_line(limit: 200)
      out, ok = git("rev-list", "--first-parent", "-n", limit.to_s, "refs/heads/main")
      ok ? out.split : []
    end

    def merge_base(a, b)
      out, ok = git("merge-base", a, b)
      ok ? out.strip : nil
    end

    def goldens(rev)
      return [] unless rev&.match?(SHA)

      out, ok = git("ls-tree", "--name-only", "#{rev}:tests/golden")
      ok ? out.split.select { |n| n.end_with?(".png") }.map { |n| "tests/golden/#{n}" } : []
    end

    def blob_id(rev, path)
      out, ok = git("rev-parse", "--verify", "--quiet", "#{rev}:#{path}")
      ok ? out.strip : nil
    end

    # Only full commit ids and golden images: nothing else leaves the repository.
    def golden_png(rev, path)
      return nil unless rev.to_s.match?(SHA) && path.to_s.match?(GOLDEN)

      out, ok = git("cat-file", "blob", "#{rev}:#{path}", binmode: true)
      ok ? out : nil
    end

    def log_since(base, head, limit: 30)
      out, ok = git("log", "--no-merges", "--format=%H%x1f%an%x1f%s%x1f%cI", "-n", limit.to_s, "#{base}..#{head}")
      return [] unless ok

      out.lines.map { |l| l.chomp.split("\x1f") }.map { |sha, author, subject, at| { sha: sha, author: author, subject: subject, at: at } }
    end

    private

    def git(*args, binmode: false)
      out, _err, status = Open3.capture3("git", "--git-dir", @git_dir, *args, binmode: binmode)
      [out, status.success?]
    end
  end
end
