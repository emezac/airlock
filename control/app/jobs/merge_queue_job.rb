class MergeQueueJob < ApplicationJob
  queue_as :merge_queue
  # One batch at a time per repository: batches build on each other's main.
  limits_concurrency to: 1, key: ->(repo) { repo } if respond_to?(:limits_concurrency)

  def perform(repo)
    loop do
      result = Airlock::QueueRunner.for(repo).run_once(repo)
      # Another worker holds the repository: try again shortly, so a change
      # queued while that worker was finishing is never stranded.
      return self.class.set(wait: 5.seconds).perform_later(repo) if result == :busy
      break unless result.is_a?(MergeBatch)
    end
  end
end
