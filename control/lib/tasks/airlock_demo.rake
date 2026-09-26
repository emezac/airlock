namespace :airlock do
  namespace :demo do
    def demo_repo = ENV.fetch("REPO", "frameline")
    def demo_bare = File.join(ENV.fetch("AIRLOCK_GIT_ROOT"), "#{demo_repo}.git")

    desc "Create the demo repository behind the gate from demo/frameline: bin/rails airlock:demo:seed"
    task seed: :environment do
      source = Rails.root.join("../demo/#{demo_repo}").to_s
      abort "#{demo_bare} already exists; run airlock:demo:reset FORCE=1 first" if File.exist?(demo_bare)

      Dir.mktmpdir do |tmp|
        system("git", "init", "-q", "--bare", "-b", "main", demo_bare, exception: true)
        system("cp", "-R", "#{source}/.", tmp, exception: true)
        FileUtils.rm_rf(File.join(tmp, "target"))
        git = ["git", "-C", tmp, "-c", "user.name=airlock", "-c", "user.email=airlock@airlock.local"]
        system(*git, "init", "-q", "-b", "main", exception: true)
        system(*git, "add", "-A", exception: true)
        system(*git, "commit", "-qm", "#{demo_repo.capitalize} seed", exception: true)
        # The seed goes in before the hook exists: nothing else ever bypasses the gate.
        system(*git, "push", "-q", demo_bare, "main", exception: true)
      end
      hook = File.join(demo_bare, "hooks/pre-receive")
      FileUtils.cp(Rails.root.join("../hooks/pre-receive"), hook)
      FileUtils.chmod("+x", hook)
      puts "seeded #{demo_bare}; pushes now go through #{ENV.fetch('AIRLOCK_URL', 'AIRLOCK_URL (unset!)')}"
    end

    desc "Forget everything about the demo repository (development only): bin/rails airlock:demo:reset [FORCE=1 also deletes the bare repo]"
    task reset: :environment do
      abort "refusing to reset in production" if Rails.env.production?

      changes = Change.where(repo: demo_repo)
      ChangeLabel.where(change_id: changes.select(:id)).delete_all
      LabelOverride.where(change_id: changes.select(:id)).delete_all
      counts = {
        changes: changes.delete_all, batches: MergeBatch.where(repo: demo_repo).delete_all,
        gate_decisions: GateDecision.where(repo: demo_repo).delete_all, agent_runs: AgentRun.where(repo: demo_repo).delete_all,
        agent_stats: AgentStat.delete_all
      }
      if ENV["FORCE"] == "1"
        FileUtils.rm_rf(demo_bare)
        FileUtils.rm_rf(Dir[File.join(ENV.fetch("AIRLOCK_WORK_ROOT", Rails.root.join("tmp/workspaces").to_s), "#{demo_repo}*")])
        counts[:repository] = "deleted"
      end
      puts counts.map { |k, v| "#{k}: #{v}" }.join(", ")
    end
  end
end
