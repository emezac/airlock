# Renders one commit with the policy's render command, through the same runner
# as CI (a Token Factory Sandbox when configured), in its own clone. Used for
# main after every green batch, and for a branch whose visual change waits for
# a reviewer, so motion can be judged on the animatic, not on a still.
class RenderJob < ApplicationJob
  queue_as :default
  limits_concurrency to: 1, key: ->(repo, _sha) { "render-#{repo}" } if respond_to?(:limits_concurrency)

  def perform(repo, sha)
    policy = Airlock::Policy.load
    renders = Airlock::Renders.new(repo)
    return if policy.render_command.blank? || renders.exists?(sha)

    workspace = Airlock::GitWorkspace.new(remote: File.join(ENV.fetch("AIRLOCK_GIT_ROOT"), "#{repo}.git"),
                                          path: File.join(ENV.fetch("AIRLOCK_WORK_ROOT", Rails.root.join("tmp/workspaces").to_s), "#{repo}-render"))
    workspace.prepare!
    workspace.reset_to(sha)
    output_dir = File.join(workspace.path, policy.render_output)
    FileUtils.rm_rf(output_dir)
    runner = Airlock::QueueRunner.runner(policy, timeout: policy.render_timeout)
    result = runner.call(workspace.path, command: policy.render_command, collect: [policy.render_output])
    manifest = renders.store!(sha, from: output_dir, ok: result.ok, output: result.output)
    Agentkit::Audit.record(event_type: "commit.rendered", status: manifest["ok"] ? "ok" : "failed",
                           payload: { repo: repo, sha: sha, files: manifest["files"] })
  end
end
