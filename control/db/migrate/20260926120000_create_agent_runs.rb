class CreateAgentRuns < ActiveRecord::Migration[8.1]
  def change
    create_table :agent_runs do |t|
      t.string :repo, null: false
      t.string :agent, null: false
      t.string :task, null: false
      t.string :title
      t.string :status, null: false, default: "waiting"   # waiting, working, pushed, gave_up, error
      t.integer :attempts, null: false, default: 0
      t.integer :input_tokens, null: false, default: 0
      t.integer :output_tokens, null: false, default: 0
      t.string :branch
      t.string :sha
      t.string :last_note
      t.text :gate_output
      t.jsonb :log, null: false, default: []
      t.string :swarm_id
      t.datetime :started_at
      t.datetime :finished_at
      t.timestamps
    end
    add_index :agent_runs, [:repo, :created_at]
    add_index :agent_runs, :swarm_id
  end
end
