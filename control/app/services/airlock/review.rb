module Airlock
  # A person's decision on a change the gate escalated. Shared by the JSON API
  # and the Frameline tab, so both leave the same audit trail.
  module Review
    Conflict = Class.new(StandardError)

    module_function

    def decide!(change, state:, reviewer:, reason: nil)
      raise ArgumentError, "unknown decision #{state}" unless %w[approved rejected].include?(state)
      raise Conflict, "change is #{change.state}" unless change.state == "needs_review"

      change.update!(state: state, reviewed_at: Time.current, reviewer: reviewer)
      # A reviewer's reason is the "why" of the stop, like a failure diagnosis:
      # an agent that retries the task starts from it.
      if state == "rejected" && reason.present?
        change.update!(diagnosis: { "category" => "rejected_by_reviewer", "category_source" => "human", "source" => "human",
                                    "summary" => reason.to_s.strip.first(500), "next_step" => "agent_retry",
                                    "culprit_files" => [], "reviewer" => reviewer, "at" => Time.current.iso8601 })
      end
      Agentkit::Audit.record(event_type: "change.reviewed", status: state, subject: change,
                             payload: { reviewer: reviewer, reason: reason.to_s.first(500) },
                             failure_mode: :required)
      AgentStat.observe!(change.pusher, failed: true, alpha: Policy.load.failure_rate_alpha) if state == "rejected"
      MergeQueueJob.perform_later(change.repo) if state == "approved"
      Floor.emit(state == "approved" ? "review.approved" : "review.rejected", repo: change.repo, actor: reviewer, change: change,
                 task: Floor.task_for(change.ref), text: reason.presence || state, data: { agent: change.pusher })
      change
    end

    def token_ok?(given)
      expected = ENV["AIRLOCK_REVIEW_TOKEN"].to_s
      expected.present? && ActiveSupport::SecurityUtils.secure_compare(expected, given.to_s)
    end
  end
end
