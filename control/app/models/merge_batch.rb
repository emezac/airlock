class MergeBatch < ApplicationRecord
  STATES = %w[running green red].freeze

  # `changes` is taken by ActiveRecord dirty tracking.
  has_many :batch_changes, class_name: "Change", foreign_key: :merge_batch_id, inverse_of: :merge_batch

  validates :repo, presence: true
  validates :state, inclusion: { in: STATES }
end
