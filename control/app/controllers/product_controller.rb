# The Frameline tab: what was asked for and what came out. Read-only, like the
# control room; images are served straight from git blobs or from renders of
# main, and only by full commit id, so every URL is immutable.
class ProductController < ApplicationController
  IMMUTABLE = "public, max-age=31536000, immutable".freeze

  def show
    @repo = params.fetch(:repo, "frameline")
    @p = Airlock::ProductSnapshot.new(repo: @repo).to_h
    respond_to do |format|
      format.html { request.xhr? || params[:fragment] ? render(partial: "page", locals: { p: @p, reviewer: session[:reviewer] }) : render }
      format.json { render json: @p }
    end
  end

  def golden
    png = Airlock::RepoFiles.for(params.fetch(:repo, "frameline")).golden_png(params[:rev], "tests/golden/#{params[:name]}.png")
    return head :not_found unless png

    response.headers["Cache-Control"] = IMMUTABLE
    send_data png, type: "image/png", disposition: "inline"
  end

  def rendered
    path = Airlock::Renders.new(params.fetch(:repo, "frameline")).file(params[:sha], params[:name])
    return head :not_found unless path

    response.headers["Cache-Control"] = IMMUTABLE
    send_file path, type: Airlock::Renders.type(path), disposition: "inline"
  end
end
