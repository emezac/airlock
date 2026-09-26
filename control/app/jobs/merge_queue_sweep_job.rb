# Wakes the merge queue for every repository with changes ready to merge. A
# process that dies after an approval would otherwise leave them waiting until
# the next push. Runs at server boot; in production also as a recurring task.
class MergeQueueSweepJob < ApplicationJob
  queue_as :merge_queue

  def perform
    Change.mergeable.reorder(nil).distinct.pluck(:repo).each { |repo| MergeQueueJob.perform_later(repo) }
  end
end
