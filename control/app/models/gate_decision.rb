class GateDecision < ApplicationRecord
  VERDICTS = %w[accept reject].freeze
  REVIEWS = %w[auto human].freeze

  validates :repo, :ref, :pusher, presence: true
  validates :verdict, inclusion: { in: VERDICTS }
  validates :review, inclusion: { in: REVIEWS }, allow_nil: true

  scope :recent, -> { order(created_at: :desc) }
end
