module Airlock
  # Closed vocabularies for describing a change. A label outside these sets is
  # rejected, whoever proposes it: a rule, a model or a human.
  module Labels
    Invalid = Class.new(ArgumentError)

    SOURCES = %w[rule model override].freeze

    # Rules own the categories they can compute from paths and diffs; the model
    # only fills semantic ones. Each category has exactly one automatic owner.
    SCHEMA = {
      "area" => { owner: "rule", values: nil }, # values come from the policy's path map
      "blast_radius" => { owner: "rule", values: %w[single_area multi_area core] },
      "test_signal" => { owner: "rule", values: %w[tests_added tests_modified no_tests tests_weakened] },
      "dependency" => { owner: "rule", values: %w[none added_or_changed] },
      "change_type" => { owner: "model", values: %w[fix feature refactor deps config test docs revert] },
      "goal_clarity" => { owner: "model", values: %w[clear partial vague] },
      "safety_flag" => { owner: "model", values: %w[none touches_money touches_auth touches_data weakens_safeguard] }
    }.freeze

    MODEL_CATEGORIES = SCHEMA.select { |_, spec| spec[:owner] == "model" }.keys.freeze

    module_function

    def categories = SCHEMA.keys

    def allowed(category, policy)
      spec = SCHEMA.fetch(category) { raise Invalid, "unknown label category '#{category}'" }
      spec[:values] || (policy.area_labels.values.uniq + %w[multiple other])
    end

    def validate!(category, value, policy)
      raise Invalid, "invalid value '#{value}' for '#{category}'" unless allowed(category, policy).include?(value)

      value
    end

    def valid?(category, value, policy)
      validate!(category, value, policy)
      true
    rescue Invalid
      false
    end

    # override > rule > model, per category.
    def effective(labels)
      rank = { "override" => 3, "rule" => 2, "model" => 1 }
      labels.group_by(&:category).transform_values { |ls| ls.max_by { |l| rank.fetch(l.source) } }
    end
  end
end
