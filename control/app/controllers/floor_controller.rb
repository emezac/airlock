# The floor: the same system as the control room, told as a place where
# agents and changes move between stations. Read-only.
class FloorController < ApplicationController
  MAX_EVENTS = 500

  def show
    @repo = params.fetch(:repo, "frameline")
  end

  def state
    render json: Airlock::FloorSnapshot.new(repo: params.fetch(:repo, "frameline")).to_h
  end

  # Live: events after a cursor. Replay: events in a time window, oldest first.
  def events
    scope = FloorEvent.where(repo: params.fetch(:repo, "frameline")).order(:id)
    scope = scope.where("id > ?", params[:after].to_i) if params[:after].present?
    scope = scope.where(occurred_at: Time.iso8601(params[:from])..) if params[:from].present?
    scope = scope.where(occurred_at: ..Time.iso8601(params[:to])) if params[:to].present?
    rows = scope.limit([params.fetch(:limit, MAX_EVENTS).to_i, MAX_EVENTS].min.clamp(1, MAX_EVENTS)).to_a
    render json: { events: rows, cursor: rows.last&.id || params[:after].to_i }
  rescue ArgumentError
    head :bad_request
  end

  # Windows worth replaying: stretches of floor events separated by a quiet
  # gap, newest first, whoever produced them (a swarm, the scripted agent, a review).
  GAP = 15.minutes

  def sessions
    times = FloorEvent.where(repo: params.fetch(:repo, "frameline")).order(:occurred_at).pluck(:occurred_at, :actor, :kind)
    windows = times.slice_when { |a, b| b[0] - a[0] > GAP }.map do |rows|
      actors = rows.filter_map { |_, actor, kind| actor if kind.start_with?("agent.", "gate.") }.uniq
      { from: (rows.first[0] - 1).iso8601, to: (rows.last[0] + 1).iso8601, events: rows.size, agents: actors.sort }
    end
    render json: windows.reverse.first(12)
  end
end
