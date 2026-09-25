class CreateGateDecisions < ActiveRecord::Migration[8.1]
  def change
    create_table :gate_decisions do |t|
      t.string :repo, null: false
      t.string :ref, null: false
      t.string :pusher, null: false
      t.string :old_sha
      t.string :new_sha
      t.string :verdict, null: false
      t.string :review
      t.jsonb :reasons, null: false, default: []
      t.jsonb :review_reasons, null: false, default: []
      t.integer :commit_count
      t.jsonb :changed_paths, null: false, default: []

      t.timestamps
    end
    add_index :gate_decisions, [:repo, :created_at]
    add_index :gate_decisions, :verdict
  end
end
