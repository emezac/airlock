class DiagnoseFailureJob < ApplicationJob
  queue_as :default

  def perform(change_id)
    change = Change.find(change_id)
    return unless %w[failed conflict].include?(change.state)

    diagnosis = Airlock::Diagnosis.new(model: Airlock::Diagnosis.model).call(change)
    change.update!(diagnosis: diagnosis.merge("at" => Time.current.iso8601))
    Airlock::Floor.emit("diagnosis.done", repo: change.repo, actor: change.pusher, change: change,
                        task: Airlock::Floor.task_for(change.ref), text: diagnosis["summary"],
                        data: diagnosis.slice("category", "next_step", "culprit_files", "source"))
    Agentkit::Audit.record(event_type: "change.diagnosed", status: diagnosis["category"], subject: change,
                           payload: diagnosis.slice("category", "category_source", "next_step", "source", "model"))
  end
end
