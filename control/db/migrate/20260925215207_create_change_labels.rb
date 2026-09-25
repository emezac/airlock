class CreateChangeLabels < ActiveRecord::Migration[8.1]
  def change
    create_table :change_labels do |t|
      t.references :change, null: false, foreign_key: true
      t.string :category, null: false
      t.string :value, null: false
      t.string :source, null: false
      t.float :confidence
      t.string :reason

      t.timestamps
    end
    add_index :change_labels, [:change_id, :category, :source], unique: true
  end
end
