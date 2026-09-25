# Read-only live panel. It shows no secrets, report payloads or model prompts,
# so it can back the public demo URL.
class DashboardController < ApplicationController
  def show
    @snapshot = Airlock::DashboardSnapshot.new(repo: params[:repo]).to_h
    respond_to do |format|
      format.html { request.xhr? || params[:fragment] ? render(partial: "panel", locals: { s: @snapshot }) : render }
      format.json { render json: @snapshot }
    end
  end
end
