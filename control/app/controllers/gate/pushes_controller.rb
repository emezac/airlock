module Gate
  # Called by the pre-receive hook. 2xx admits the ref update; 403 rejects it.
  class PushesController < ActionController::API
    before_action :authenticate_hook!

    def create
      push = Airlock::Push.from_params(params.to_unsafe_h)
      decision = Airlock::Gate.new(Airlock::Policy.load).evaluate(push)
      record!(push, decision)
      render plain: decision.message, status: decision.accepted? ? :ok : :forbidden
    rescue KeyError, ActionController::ParameterMissing => e
      render plain: "malformed push report: #{e.message}", status: :unprocessable_entity
    end

    private

    # The hook and the service share a secret; without it anyone could ask the gate.
    def authenticate_hook!
      expected = ENV["AIRLOCK_HOOK_SECRET"].to_s
      given = request.headers["X-Airlock-Hook-Secret"].to_s
      return if expected.present? && ActiveSupport::SecurityUtils.secure_compare(expected, given)

      render plain: "unauthorized hook", status: :unauthorized
    end

    def record!(push, decision)
      GateDecision.transaction do
        row = GateDecision.create!(
          repo: push.repo, ref: push.ref, pusher: push.pusher, old_sha: push.old_sha, new_sha: push.new_sha,
          verdict: decision.verdict, review: decision.review, reasons: decision.reasons,
          review_reasons: decision.review_reasons, commit_count: push.commits.size,
          changed_paths: push.files.map(&:path)
        )
        Agentkit::Audit.record(event_type: "gate.decided", status: decision.verdict, subject: row,
                               payload: { repo: push.repo, ref: push.ref, pusher: push.pusher,
                                          review: decision.review, reasons: decision.reasons },
                               failure_mode: :required)
      end
    end
  end
end
