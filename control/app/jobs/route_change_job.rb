class RouteChangeJob < ApplicationJob
  queue_as :default

  def perform(change_id, report)
    change = Change.find(change_id)
    return unless change.state == "pending"

    policy = Airlock::Policy.load
    push = Airlock::Push.from_params(report)
    return if policy.verify_evidence && !evidence_holds?(change, push, policy)

    change = Airlock::Intake.new(policy).route!(change, push)
    MergeQueueJob.perform_later(change.repo) if change.state == "queued"
  end

  private

  # A change whose evidence does not reproduce is rejected before anyone
  # spends attention on it. Without a sandbox the check is recorded as skipped.
  def evidence_holds?(change, push, policy)
    verifier = Airlock::QueueRunner.verifier(change.repo, policy: policy)
    unless verifier
      change.update!(evidence_status: "skipped")
      return true
    end

    outcome = verifier.verify(change, push)
    change.update!(evidence_status: outcome.status, evidence_command: outcome.command,
                   evidence_checkpoint: outcome.checkpoint, evidence_output: outcome.output)
    Agentkit::Audit.record(event_type: "change.evidence", status: outcome.status, subject: change,
                           payload: { command: outcome.command, checkpoint: outcome.checkpoint })
    return true if outcome.status == "verified"

    change.update!(state: "rejected", review_reasons: ["evidence did not reproduce: #{outcome.command}"])
    AgentStat.observe!(change.pusher, failed: true, alpha: policy.failure_rate_alpha)
    false
  end
end
