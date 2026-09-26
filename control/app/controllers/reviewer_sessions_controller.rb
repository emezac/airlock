# Signs a person in as a reviewer for the Frameline tab, with the same token
# as the reviews API. Only a name and a flag live in the signed session.
class ReviewerSessionsController < ApplicationController
  rate_limit to: 10, within: 1.minute, only: :create

  def new; end

  def create
    name = params[:name].to_s.strip.first(40)
    if name.present? && Airlock::Review.token_ok?(params[:token])
      reset_session
      session[:reviewer] = name
      redirect_to product_path, notice: "Signed in as #{name}."
    else
      flash.now[:alert] = "That name and token do not match."
      render :new, status: :unauthorized
    end
  end

  def destroy
    reset_session
    redirect_to product_path, notice: "Signed out."
  end
end
