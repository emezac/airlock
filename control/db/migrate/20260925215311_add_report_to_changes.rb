class AddReportToChanges < ActiveRecord::Migration[8.1]
  def change
    add_column :changes, :report, :jsonb, null: false, default: {}
  end
end
