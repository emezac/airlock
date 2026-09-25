# Per-agent failure rate with exponential forgetting:
#   f(t) = (1 - alpha) f(t-1) + alpha * 1[the change failed]
# Recent failures weigh more; an agent recovers its autonomy gradually.
class AgentStat < ApplicationRecord
  PRIOR = 0.1

  validates :agent, presence: true, uniqueness: true

  def self.rate_for(agent) = find_by(agent: agent)&.failure_rate || PRIOR

  def self.observe!(agent, failed:, alpha:)
    stat = find_or_create_by!(agent: agent) { |s| s.failure_rate = PRIOR }
    stat.with_lock do
      stat.update!(failure_rate: (1 - alpha) * stat.failure_rate + alpha * (failed ? 1.0 : 0.0),
                   observations: stat.observations + 1)
    end
    stat
  end
end
