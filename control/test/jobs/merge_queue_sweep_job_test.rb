require "test_helper"

class MergeQueueSweepJobTest < ActiveJob::TestCase
  test "wakes the queue for each repository with approved or queued changes, once" do
    %w[approved queued needs_review].each_with_index do |state, i|
      d = GateDecision.create!(repo: "frameline", ref: "refs/heads/agents/agent-1/x#{i}", pusher: "agent-1", verdict: "accept")
      Change.create!(repo: "frameline", ref: d.ref, pusher: "agent-1", head_sha: "x", gate_decision: d, state: state)
    end
    MergeQueueSweepJob.perform_now
    assert_enqueued_jobs 1, only: MergeQueueJob
    assert_enqueued_with(job: MergeQueueJob, args: ["frameline"])
  end
end
