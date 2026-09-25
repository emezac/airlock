class CreateMergeBatches < ActiveRecord::Migration[8.1]
  def change
    create_table :merge_batches do |t|
      t.string :repo, null: false
      t.string :state, null: false, default: "running"
      t.integer :size
      t.integer :ci_runs, null: false, default: 0
      t.string :base_sha
      t.string :merged_sha
      t.jsonb :log, null: false, default: []

      t.timestamps
    end
  end
end
