require "json"

module Airlock
  # Rendered storyboards and animatics of main, one folder per commit:
  #   <AIRLOCK_WORK_ROOT>/renders/<repo>/<sha>/{manifest.json, *.png, *.mp4}
  class Renders
    FILE = /\A[A-Za-z0-9_-][A-Za-z0-9_.-]*\.(png|mp4)\z/
    MAX_BYTES = 50 * 1024 * 1024
    TYPES = { "png" => "image/png", "mp4" => "video/mp4" }.freeze

    def initialize(repo, root: ENV.fetch("AIRLOCK_WORK_ROOT", Rails.root.join("tmp/workspaces").to_s))
      @dir = File.join(root, "renders", repo)
    end

    def exists?(sha) = File.exist?(manifest_path(sha))

    def manifest(sha)
      return nil unless sha.to_s.match?(RepoFiles::SHA) && exists?(sha)

      JSON.parse(File.read(manifest_path(sha)))
    end

    # The most recent successful render of any of these commits (by default,
    # any commit at all).
    def latest(among: nil)
      Dir[File.join(@dir, "*", "manifest.json")].filter_map { |f| JSON.parse(File.read(f)) rescue nil }
                                                .select { |m| m["ok"] && (among.nil? || among.include?(m["sha"])) }
                                                .max_by { |m| m["at"].to_s }
    end

    # Copies only images and videos with plain names, and records the run.
    def store!(sha, from:, ok:, output:)
      raise ArgumentError, "not a commit id: #{sha}" unless sha.to_s.match?(RepoFiles::SHA)

      target = File.join(@dir, sha)
      FileUtils.mkdir_p(target)
      files = Dir.exist?(from) ? Dir.children(from).sort : []
      kept = files.select do |name|
        src = File.join(from, name)
        name.match?(FILE) && File.file?(src) && !File.symlink?(src) && File.size(src) <= MAX_BYTES
      end
      kept.each { |name| FileUtils.cp(File.join(from, name), File.join(target, name)) }
      manifest = { "sha" => sha, "ok" => ok && kept.any?, "files" => kept, "at" => Time.current.iso8601,
                   "output" => output.to_s.lines.last(20).join }
      File.write(manifest_path(sha), JSON.pretty_generate(manifest))
      manifest
    end

    def file(sha, name)
      return nil unless sha.to_s.match?(RepoFiles::SHA) && name.to_s.match?(FILE)

      path = File.join(@dir, sha, name)
      File.file?(path) ? path : nil
    end

    def self.type(name) = TYPES.fetch(File.extname(name).delete("."))

    private

    def manifest_path(sha) = File.join(@dir, sha, "manifest.json")
  end
end
