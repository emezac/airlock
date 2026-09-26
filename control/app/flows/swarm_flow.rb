# A swarm run: plan the backlog over the agents, let every agent work its share
# in parallel (one branch job each), then report. Agents never merge anything:
# their pushes go through the gate, and the merge queue decides.
#
#   SwarmFlow.perform_later(repo: "frameline", agents: %w[agent-ada agent-kai],
#                           backlog: "demo/backlog.yml", swarm_id: SecureRandom.uuid)
class SwarmFlow < Agentkit::Flow
  input :repo, :agents, :backlog, :only, :swarm_id

  step :plan do |ctx|
    input = ctx.input
    Airlock::Swarm.plan(repo: input[:repo], agents: input[:agents], backlog: input[:backlog],
                        only: input[:only], swarm_id: input[:swarm_id])
  end

  # One branch per agent: an agent works its tasks in order in its own clone,
  # so no two branches ever share a workspace.
  map :work, over: ->(ctx) { ctx[:plan].value }, branch_effect: :side_effecting,
             idempotency_key: ->(item) { "#{item['swarm_id']}:#{item['agent']}" } do |item|
    Airlock::Swarm.work(item)
  end

  join :work, on: :all_settled, timeout: 4 * 3600, on_timeout: :continue_with_partial

  step :report do |ctx|
    results = ctx[:work]
    results = results.value if results.respond_to?(:value) && !results.is_a?(Agentkit::Flow::StepResults)
    Airlock::Swarm.report(results.respond_to?(:values) ? results.values : results)
  end
end
