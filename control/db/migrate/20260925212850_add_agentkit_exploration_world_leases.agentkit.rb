# frozen_string_literal: true

# This migration comes from agentkit (originally 20)
class AddAgentkitExplorationWorldLeases < ActiveRecord::Migration[7.1]
  def change
    add_column :agentkit_exploration_worlds, :lease_owner, :string
    add_column :agentkit_exploration_worlds, :lease_expires_at, :datetime
    add_index :agentkit_exploration_worlds, :lease_expires_at,
              name: "idx_agentkit_exploration_worlds_lease"
  end
end
