module ApplicationHelper
  # Values that say "nothing to see" are hidden on cards to keep signal visible.
  QUIET_LABELS = %w[none single_area].freeze
  LABEL_SHORT = { "area" => "area", "blast_radius" => "scope", "test_signal" => "tests", "dependency" => "deps",
                  "change_type" => "type", "goal_clarity" => "goal", "safety_flag" => "safety" }.freeze

  def quiet_label?(value) = QUIET_LABELS.include?(value)

  def label_short(category) = LABEL_SHORT.fetch(category, category)
end
