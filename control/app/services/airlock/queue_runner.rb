module Airlock
  # Builds the merge queue for a repository from the environment.
  module QueueRunner
    module_function

    def for(repo, policy: Policy.load)
      root = ENV.fetch("AIRLOCK_GIT_ROOT")       # directory with the bare repositories
      work = ENV.fetch("AIRLOCK_WORK_ROOT", Rails.root.join("tmp/workspaces").to_s)
      env = { "AIRLOCK_URL" => ENV["AIRLOCK_URL"], "AIRLOCK_HOOK_SECRET" => ENV["AIRLOCK_HOOK_SECRET"] }.compact
      workspace = GitWorkspace.new(remote: File.join(root, "#{repo}.git"), path: File.join(work, repo),
                                   identity: policy.merge_identity, env: env)
      MergeQueue.new(policy, workspace: workspace, runner: Runners::Local.new(command: policy.ci_command,
                                                                              timeout: policy.ci_timeout))
    end
  end
end
