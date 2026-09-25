class ChangeLabel < ApplicationRecord
  belongs_to :change

  validates :category, :value, presence: true
  validates :source, inclusion: { in: Airlock::Labels::SOURCES }
  validates :category, uniqueness: { scope: %i[change_id source] }
end
