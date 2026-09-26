class AddDiagnosisToChanges < ActiveRecord::Migration[8.1]
  def change
    add_column :changes, :diagnosis, :jsonb, null: false, default: {}
  end
end
