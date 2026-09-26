# Approve or reject from the Frameline tab. Needs a signed-in reviewer and
# Rails' CSRF token, so a link on another site cannot decide anything.
class DecisionsController < ApplicationController
  before_action { redirect_to new_reviewer_session_path unless session[:reviewer].present? }

  def create
    change = Change.find(params[:review_id])
    state = params[:decision] == "approve" ? "approved" : "rejected"
    Airlock::Review.decide!(change, state: state, reviewer: session[:reviewer], reason: params[:reason])
    redirect_to product_path(anchor: "task-list"), notice: "#{change.ref.split('/').last} #{state}."
  rescue Airlock::Review::Conflict => e
    redirect_to product_path, alert: e.message
  end
end
