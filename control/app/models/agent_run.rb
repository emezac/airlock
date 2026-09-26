# One agent working one backlog task. Updated while the worker runs, so the
# panel can show the swarm at work, not only what reached the gate.
class AgentRun < ApplicationRecord
  STATUSES = %w[waiting working pushed gave_up error].freeze
  FINISHED = %w[pushed gave_up error].freeze

  validates :repo, :agent, :task, presence: true
  validates :status, inclusion: { in: STATUSES }

  scope :recent, -> { order(created_at: :desc) }

  def finished? = FINISHED.include?(status)

  def note!(line)
    attempt = line[/\Aattempt (\d+)/, 1]&.to_i
    update!(last_note: line.truncate(240), log: log + [line], attempts: [attempts, attempt.to_i].max)
  end

  def finish!(result)
    update!(status: result.status, attempts: result.attempts, input_tokens: result.input_tokens,
            output_tokens: result.output_tokens, branch: result.branch, sha: result.sha,
            gate_output: result.gate_output, finished_at: Time.current)
  end
end
