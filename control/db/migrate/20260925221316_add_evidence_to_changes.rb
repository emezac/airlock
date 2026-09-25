class AddEvidenceToChanges < ActiveRecord::Migration[8.1]
  def change
    add_column :changes, :evidence_status, :string
    add_column :changes, :evidence_command, :string
    add_column :changes, :evidence_checkpoint, :string
    add_column :changes, :evidence_output, :text
  end
end
