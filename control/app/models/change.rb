# One agent branch waiting to reach main.
class Change < ApplicationRecord
  STATES = %w[pending queued needs_review approved rejected merging merged failed conflict].freeze

  belongs_to :gate_decision
  belongs_to :merge_batch, optional: true

  validates :repo, :ref, :pusher, :head_sha, presence: true
  validates :state, inclusion: { in: STATES }

  scope :mergeable, -> { where(state: %w[queued approved]).order(:created_at) }
  scope :awaiting_review, -> { where(state: "needs_review").order(:review_requested_at) }
end
