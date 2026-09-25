class CreateLabelOverrides < ActiveRecord::Migration[8.1]
  def change
    create_table :label_overrides do |t|
      t.references :change, null: false, foreign_key: true
      t.string :category, null: false
      t.string :previous_value
      t.string :new_value, null: false
      t.string :action, null: false
      t.string :reason
      t.string :reviewer

      t.timestamps
    end
  end
end
