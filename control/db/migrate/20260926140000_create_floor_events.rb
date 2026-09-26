class CreateFloorEvents < ActiveRecord::Migration[8.1]
  def change
    create_table :floor_events do |t|
      t.string :repo, null: false
      t.string :kind, null: false
      t.string :actor              # the agent, or the station's own character
      t.bigint :change_id
      t.string :task
      t.string :text               # one short caption
      t.jsonb :data, null: false, default: {}
      t.datetime :occurred_at, null: false
    end
    add_index :floor_events, [:repo, :id]
    add_index :floor_events, [:repo, :occurred_at]
  end
end
