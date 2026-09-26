module Airlock
  # Integrates queued changes in batches. A batch is tested on top of main; a
  # red batch is split in halves and each half tested again (test-and-bisect),
  # so a single broken change costs about log2(k) extra runs instead of k.
  # The batch size k minimizes expected CI runs per change (BatchMath) for the
  # current failure-rate estimate.
  class MergeQueue
    def initialize(policy, workspace:, runner:)
      @policy = policy
      @workspace = workspace
      @runner = runner
    end

    # Batches build on each other's main and share one workspace, so only one
    # may run per repository at a time, whatever the job backend does.
    def run_once(repo)
      key = Zlib.crc32("airlock-merge-queue:#{repo}")
      conn = ActiveRecord::Base.connection
      return :busy unless conn.select_value("SELECT pg_try_advisory_lock(#{key})")

      begin
        run_locked(repo)
      ensure
        conn.select_value("SELECT pg_advisory_unlock(#{key})")
      end
    end

    private

    def run_locked(repo)
      recover_orphans!(repo)
      candidates = Change.mergeable.where(repo: repo).to_a
      return nil if candidates.empty?

      f = failure_estimate(candidates)
      k = BatchMath.best_size(f, max: @policy.max_batch)
      picked = candidates.first(k)
      @workspace.prepare!
      base = @workspace.main_sha
      batch = MergeBatch.create!(repo: repo, state: "running", size: picked.size, base_sha: base,
                                 log: [{ event: "start", k: k, f: f.round(3),
                                         expected_runs_per_change: (BatchMath.expected_runs(k, f) / k).round(3) }])
      Change.where(id: picked.map(&:id)).update_all(state: "merging", merge_batch_id: batch.id)

      @runs = 0
      @log = batch.log
      @conflicts = []
      @green_sets = []
      good = bisect(base, picked)
      good = confirm(base, good)
      finish!(batch, base, picked, good)
    end

    # We hold the repository lock, so a batch still marked running was left by
    # a process that died mid-batch (a deploy, a crash). Its changes go back
    # to the queue, unless main already contains them, in which case the push
    # happened and only the bookkeeping was lost.
    def recover_orphans!(repo)
      orphans = MergeBatch.where(repo: repo, state: "running").to_a
      return if orphans.empty?

      @workspace.prepare!
      main = @workspace.main_sha
      orphans.each do |batch|
        batch.batch_changes.where(state: "merging").find_each do |change|
          if @workspace.contains?(main, change.ref)
            change.update!(state: "merged")
          else
            change.update!(state: "queued", merge_batch_id: nil)
          end
        end
        batch.update!(state: "abandoned", log: batch.log + [{ event: "abandoned", reason: "process ended mid-batch" }])
        Agentkit::Audit.record(event_type: "merge_queue.recovered", status: "abandoned", subject: batch,
                               payload: { repo: repo, main: main })
      end
    end

    def failure_estimate(changes)
      rates = changes.map { |c| AgentStat.rate_for(c.pusher) }
      (rates.sum / rates.size).clamp(0.01, 0.9)
    end

    # Returns the subset of changes that pass CI when tested as groups.
    def bisect(base, changes)
      return [] if changes.empty?
      return changes if green?(base, changes)
      return [] if changes.size == 1

      half = changes.size / 2
      bisect(base, changes.first(half)) + bisect(base, changes.drop(half))
    end

    # Halves that pass alone can still fail together (a semantic conflict).
    # Re-test the union once; if it fails, integrate one by one.
    def confirm(base, good)
      return good if good.size <= 1 || @green_sets.include?(good.map(&:id).sort) || green?(base, good)

      good.each_with_object([]) { |change, kept| kept << change if green?(base, kept + [change]) }
    end

    # Builds main + changes in the workspace. False when any change conflicts.
    def build(base, changes)
      @workspace.reset_to(base)
      changes.all? do |change|
        next true if @workspace.merge(change.ref, "Merge #{change.ref} (#{change.pusher})")

        @conflicts << change unless @conflicts.include?(change)
        false
      end
    end

    def green?(base, changes)
      return false unless build(base, changes)

      @runs += 1
      result = @runner.call(@workspace.path)
      @log << { event: "ci", changes: changes.map(&:id), ok: result.ok, output: result.ok ? nil : result.output.last(2000) }
      @green_sets << changes.map(&:id).sort if result.ok
      result.ok
    end

    def finish!(batch, base, picked, good)
      merged_sha = nil
      if good.any?
        build(base, good) || raise(GitWorkspace::Error, "confirmed set no longer merges")
        merged_sha = @workspace.head
        @workspace.push_main!(merged_sha)
        @workspace.fetch!
        unless @workspace.main_sha == merged_sha && good.all? { |c| @workspace.contains?(merged_sha, c.ref) }
          raise GitWorkspace::Error, "main does not contain the merged batch"
        end
      end

      picked.each do |change|
        state = if good.include?(change) then "merged"
                elsif @conflicts.include?(change) then "conflict"
                else "failed"
                end
        change.update!(state: state, outcome: state)
        AgentStat.observe!(change.pusher, failed: state != "merged", alpha: @policy.failure_rate_alpha)
      end
      failed = picked.reject { |c| good.include?(c) }
      batch.update!(state: good.size == picked.size ? "green" : "red", ci_runs: @runs, merged_sha: merged_sha,
                    log: @log + [{ event: "finish", merged: good.map(&:id), runs: @runs }])
      Agentkit::Audit.record(event_type: "merge_queue.batch", status: batch.state, subject: batch,
                             payload: { size: picked.size, merged: good.size, ci_runs: @runs })
      # After the batch's log is saved: the diagnosis reads it.
      failed.each { |change| DiagnoseFailureJob.perform_later(change.id) }
      batch
    end
  end
end
