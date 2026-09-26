module Airlock
  # Emits floor events. Best effort by design: the floor is a view, and a
  # failure to record an animation must never fail a push, a batch or a review.
  module Floor
    module_function

    def emit(kind, repo:, actor: nil, change: nil, task: nil, text: nil, data: {}, at: Time.current)
      FloorEvent.create!(repo: repo, kind: kind, actor: actor, change_id: change.respond_to?(:id) ? change.id : change,
                         task: task, text: text.to_s.truncate(200).presence, data: data, occurred_at: at)
    rescue StandardError => e
      Rails.logger.warn("floor event #{kind} not recorded: #{e.class}: #{e.message}")
      nil
    end

    # Which backlog task a change belongs to, from its branch name.
    def task_for(ref) = ref.to_s.split("/").last.to_s.upcase.presence
  end
end
