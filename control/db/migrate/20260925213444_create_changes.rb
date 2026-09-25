class CreateChanges < ActiveRecord::Migration[8.1]
  def change
    create_table :changes do |t|
      t.string :repo, null: false
      t.string :ref, null: false
      t.string :pusher, null: false
      t.string :base_sha
      t.string :head_sha, null: false
      t.string :state, null: false, default: "pending"
      t.float :risk_score
      t.jsonb :risk_features, null: false, default: {}
      t.jsonb :classification, null: false, default: {}
      t.jsonb :review_reasons, null: false, default: []
      t.references :gate_decision, null: false, foreign_key: true
      t.bigint :merge_batch_id
      t.datetime :review_requested_at
      t.datetime :reviewed_at
      t.string :reviewer
      t.string :outcome

      t.timestamps
    end
    add_index :changes, [:repo, :state, :created_at]
    add_index :changes, :merge_batch_id
  end
end
