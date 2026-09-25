class CreateAgentStats < ActiveRecord::Migration[8.1]
  def change
    create_table :agent_stats do |t|
      t.string :agent
      t.float :failure_rate, null: false, default: 0.1
      t.integer :observations, null: false, default: 0

      t.timestamps
    end
    add_index :agent_stats, :agent, unique: true
  end
end
