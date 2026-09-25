require "open3"

module Airlock
  # A working clone the merge queue builds candidates in. Pushes to main go
  # back through the server, so the pre-receive hook audits the merge queue too.
  class GitWorkspace
    Error = Class.new(StandardError)

    attr_reader :path

    def initialize(remote:, path:, identity: "airlock", env: {})
      @remote = remote
      @path = path
      @identity = identity
      @env = env
    end

    def prepare!
      unless File.directory?(File.join(@path, ".git"))
        FileUtils.mkdir_p(File.dirname(@path))
        run!("git", "clone", "-q", @remote, @path, chdir: nil)
      end
      run!("git", "config", "user.name", "Airlock merge queue")
      run!("git", "config", "user.email", "#{@identity}@airlock.local")
      fetch!
    end

    def fetch! = run!("git", "fetch", "-q", "--prune", "origin", "+refs/heads/*:refs/remotes/origin/*")

    def main_sha = run!("git", "rev-parse", "refs/remotes/origin/main").strip

    def reset_to(sha)
      run!("git", "checkout", "-q", "--detach", sha)
      run!("git", "reset", "-q", "--hard", sha)
      run!("git", "clean", "-qfdx")
    end

    # Returns false (and leaves the tree clean) when the branch conflicts.
    def merge(ref, message)
      remote_ref = ref.sub(%r{\Arefs/heads/}, "refs/remotes/origin/")
      _, status = capture("git", "merge", "-q", "--no-ff", "-m", message, remote_ref)
      return true if status.success?

      capture("git", "merge", "--abort")
      false
    end

    def head = run!("git", "rev-parse", "HEAD").strip

    def contains?(sha, ref)
      _, status = capture("git", "merge-base", "--is-ancestor", ref.sub(%r{\Arefs/heads/}, "refs/remotes/origin/"), sha)
      status.success?
    end

    def push_main!(sha)
      run!("git", "push", "-q", "origin", "#{sha}:refs/heads/main", env: { "REMOTE_USER" => @identity })
    end

    private

    def capture(*cmd, chdir: @path, env: {})
      out, status = Open3.capture2e(@env.merge(env), *cmd, **(chdir ? { chdir: chdir } : {}))
      [out, status]
    end

    def run!(*cmd, chdir: @path, env: {})
      out, status = capture(*cmd, chdir: chdir, env: env)
      raise Error, "#{cmd.first(3).join(' ')} failed: #{out.lines.last(3).join.strip}" unless status.success?

      out
    end
  end
end
