# What happened, for the floor view: a presentation stream, not a record of
# truth (that is the signed audit log). Kinds are a closed vocabulary so the
# floor can map every one of them to a station and an animation.
class FloorEvent < ApplicationRecord
  KINDS = %w[
    agent.started agent.thinking agent.editing agent.edits_refused agent.goldens agent.testing agent.check_failed
    agent.pushing agent.finished
    gate.accepted gate.rejected
    evidence.verified evidence.failed evidence.skipped
    change.routed
    review.approved review.rejected
    queue.batch_started queue.ci_run queue.batch_finished queue.recovered
    change.merged change.failed
    diagnosis.done
    render.started render.finished
  ].freeze

  validates :repo, :occurred_at, presence: true
  validates :kind, inclusion: { in: KINDS }

  def as_json(*) = { id: id, at: occurred_at.iso8601(3), kind: kind, actor: actor, change: change_id, task: task,
                     text: text, data: data }
end
