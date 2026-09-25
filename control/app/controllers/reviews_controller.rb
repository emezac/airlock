# Human decisions on escalated changes. JSON for now; the dashboard uses it too.
class ReviewsController < ActionController::API
  before_action :authenticate_reviewer!

  def index
    render json: Change.awaiting_review.map { |c| summary(c) }
  end

  def approve = decide("approved")

  def reject = decide("rejected")

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
    change.slice(:id, :repo, :ref, :pusher, :state, :risk_score, :review_reasons, :classification, :review_requested_at)
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
