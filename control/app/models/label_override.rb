# Append-only record of every human label change.
class LabelOverride < ApplicationRecord
  belongs_to :change

  validates :category, :new_value, presence: true
  validates :action, inclusion: { in: %w[added changed] }

  def readonly? = persisted?
end
