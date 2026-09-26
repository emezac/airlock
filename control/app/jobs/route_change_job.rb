class RouteChangeJob < ApplicationJob
  queue_as :default

  def perform(change_id, report)
    change = Change.find(change_id)
    return unless change.state == "pending"

    policy = Airlock::Policy.load
    push = Airlock::Push.from_params(report)
    return if policy.verify_evidence && !evidence_holds?(change, push, policy)

    change = Airlock::Intake.new(policy).route!(change, push)
    Airlock::Floor.emit("change.routed", repo: change.repo, actor: change.pusher, change: change,
                        task: Airlock::Floor.task_for(change.ref), text: change.state == "queued" ? "to the merge queue" : change.review_reasons.first,
                        data: { state: change.state, risk: change.risk_score&.round(2), reasons: change.review_reasons.first(3),
                                labels: change.effective_labels.transform_values(&:value) })
    MergeQueueJob.perform_later(change.repo) if change.state == "queued"
    # A visual change waiting for a person gets its own render to compare.
    if change.state == "needs_review" && push.files.any? { |f| f.path.match?(Airlock::RepoFiles::GOLDEN) }
      RenderJob.perform_later(change.repo, change.head_sha)
    end
  end

  private

  # A change whose evidence does not reproduce is rejected before anyone
  # spends attention on it. Without a sandbox the check is recorded as skipped.
  def evidence_holds?(change, push, policy)
    verifier = Airlock::QueueRunner.verifier(change.repo, policy: policy)
    unless verifier
      change.update!(evidence_status: "skipped")
      floor_evidence(change, "skipped", "no sandbox configured")
      return true
    end

    outcome = verifier.verify(change, push)
    change.update!(evidence_status: outcome.status, evidence_command: outcome.command,
                   evidence_checkpoint: outcome.checkpoint, evidence_output: outcome.output)
    Agentkit::Audit.record(event_type: "change.evidence", status: outcome.status, subject: change,
                           payload: { command: outcome.command, checkpoint: outcome.checkpoint })
    floor_evidence(change, outcome.status, outcome.command)
    return true if outcome.status == "verified"

    change.update!(state: "rejected", review_reasons: ["evidence did not reproduce: #{outcome.command}"])
    AgentStat.observe!(change.pusher, failed: true, alpha: policy.failure_rate_alpha)
    false
  end

  def floor_evidence(change, status, text)
    kind = { "verified" => "evidence.verified", "skipped" => "evidence.skipped" }.fetch(status, "evidence.failed")
    Airlock::Floor.emit(kind, repo: change.repo, actor: change.pusher, change: change, task: Airlock::Floor.task_for(change.ref),
                        text: text, data: { status: status })
  end
end
