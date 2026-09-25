# Human decisions on escalated changes. JSON for now; the dashboard uses it too.
class ReviewsController < ActionController::API
  before_action :authenticate_reviewer!

  def index
    render json: Change.awaiting_review.map { |c| summary(c) }
  end

  def approve = decide("approved")

  def reject = decide("rejected")

  # A human corrects one label. The value must belong to the closed vocabulary;
  # every correction is kept and the change is re-routed with the new labels.
  def label
    policy = Airlock::Policy.load
    change = Change.find(params[:id])
    category = params.require(:category).to_s
    value = Airlock::Labels.validate!(category, params.require(:value).to_s, policy)
    reason = params[:reason].to_s.first(500).presence

    Change.transaction do
      previous = change.label(category)
      override = change.labels.find_or_initialize_by(category: category, source: "override")
      override.update!(value: value, confidence: 1.0, reason: reason)
      change.label_overrides.create!(category: category, previous_value: previous, new_value: value,
                                     action: previous ? "changed" : "added", reason: reason, reviewer: @reviewer)
      Agentkit::Audit.record(event_type: "change.label_overridden", status: "ok", subject: change,
                             payload: { category: category, from: previous, to: value, reviewer: @reviewer },
                             failure_mode: :required)
    end
    change = Airlock::Intake.new(policy).reroute!(change.reload, Airlock::Push.from_params(change.report))
    MergeQueueJob.perform_later(change.repo) if change.state == "queued"
    render json: summary(change)
  rescue Airlock::Labels::Invalid => e
    render plain: e.message, status: :unprocessable_entity
  end

  private

  def decide(state)
    change = Change.find(params[:id])
    return render(plain: "change is #{change.state}", status: :conflict) unless change.state == "needs_review"

    change.update!(state: state, reviewed_at: Time.current, reviewer: @reviewer)
    Agentkit::Audit.record(event_type: "change.reviewed", status: state, subject: change,
                           payload: { reviewer: @reviewer, reason: params[:reason].to_s.first(500) },
                           failure_mode: :required)
    AgentStat.observe!(change.pusher, failed: true, alpha: Airlock::Policy.load.failure_rate_alpha) if state == "rejected"
    MergeQueueJob.perform_later(change.repo) if state == "approved"
    render json: summary(change)
  end

  def summary(change)
    change.slice(:id, :repo, :ref, :pusher, :state, :risk_score, :review_reasons, :review_requested_at).merge(
      labels: change.effective_labels.transform_values { |l| l.slice(:value, :source, :confidence, :reason) }
    )
  end

  def authenticate_reviewer!
    expected = ENV["AIRLOCK_REVIEW_TOKEN"].to_s
    given = request.headers["Authorization"].to_s.delete_prefix("Bearer ")
    unless expected.present? && ActiveSupport::SecurityUtils.secure_compare(expected, given)
      return render(plain: "unauthorized", status: :unauthorized)
    end

    @reviewer = request.headers["X-Reviewer"].presence || "human"
  end
end
