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
      MergeQueue.new(policy, workspace: workspace, runner: runner(policy))
    end

    # The evidence verifier gets its own working clone so it never races the queue.
    def verifier(repo, policy: Policy.load)
      runner = sandbox_runner(policy)
      return nil unless runner

      root = ENV.fetch("AIRLOCK_GIT_ROOT")
      work = ENV.fetch("AIRLOCK_WORK_ROOT", Rails.root.join("tmp/workspaces").to_s)
      workspace = GitWorkspace.new(remote: File.join(root, "#{repo}.git"), path: File.join(work, "#{repo}-evidence"))
      EvidenceVerifier.new(policy, workspace: workspace, runner: runner)
    end

    # CI runs in a sandbox when one is configured; locally otherwise, because
    # CI commands come from the policy, not from agents.
    def runner(policy, timeout: policy.ci_timeout)
      sandbox_runner(policy, timeout: timeout) || Runners::Local.new(command: policy.ci_command, timeout: timeout)
    end

    # Agent-supplied evidence commands only ever run in a sandbox.
    def sandbox_runner(policy, timeout: policy.ci_timeout)
      return nil unless policy.sandbox_enabled && Sandboxes::Client.configured?

      Runners::Sandbox.new(client: Sandboxes::Client.new, image: policy.sandbox_image,
                           command: policy.ci_command, timeout: timeout)
    end
  end
end
