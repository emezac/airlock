class RouteChangeJob < ApplicationJob
  queue_as :default

  def perform(change_id, report)
    change = Change.find(change_id)
    return unless change.state == "pending"

    change = Airlock::Intake.new(Airlock::Policy.load).route!(change, Airlock::Push.from_params(report))
    MergeQueueJob.perform_later(change.repo) if change.state == "queued"
  end
end
